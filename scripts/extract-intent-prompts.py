#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Extract verbatim user prompts from agent session logs for docs/intent provenance.

Scans Cursor agent transcripts, Forge conversation logs, Claude Code project
JSONL, and Codex history/session files. Emits YAML or Markdown with:
timestamp, source_tool, session_id, verbatim_prompt, file_path.

Requires: Python 3.10+ (stdlib + pathlib only).
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from collections.abc import Iterator
from dataclasses import dataclass
from datetime import UTC, datetime
from pathlib import Path
from typing import Any

USER_QUERY_RE = re.compile(
    r"<user_query>\s*(.*?)\s*</user_query>",
    flags=re.DOTALL | re.IGNORECASE,
)
FORGE_CONVERSATION_NAME = re.compile(r"conversation", re.IGNORECASE)
REPO_FILTER_DEFAULT = ("hexakit", "genesis")


@dataclass(frozen=True)
class IntentPrompt:
    timestamp: str
    source_tool: str
    session_id: str
    verbatim_prompt: str
    file_path: str
    line_number: int | None = None

    def to_dict(self) -> dict[str, Any]:
        row: dict[str, Any] = {
            "timestamp": self.timestamp,
            "source_tool": self.source_tool,
            "session_id": self.session_id,
            "verbatim_prompt": self.verbatim_prompt,
            "file_path": self.file_path,
        }
        if self.line_number is not None:
            row["line_number"] = self.line_number
        return row


def _iter_jsonl(path: Path) -> Iterator[tuple[int, dict[str, Any]]]:
    try:
        raw = path.read_text(encoding="utf-8", errors="replace")
    except OSError:
        return
    for lineno, line in enumerate(raw.splitlines(), 1):
        line = line.strip()
        if not line:
            continue
        try:
            obj = json.loads(line)
        except json.JSONDecodeError:
            continue
        if isinstance(obj, dict):
            yield lineno, obj


def _iso_from_obj(obj: dict[str, Any], fallback: datetime) -> str:
    for key in ("timestamp", "ts", "created_at", "time", "event_ts"):
        value = obj.get(key)
        if isinstance(value, str) and value.strip():
            text = value.strip().replace("Z", "+00:00")
            try:
                return datetime.fromisoformat(text).astimezone(UTC).isoformat()
            except ValueError:
                pass
        if isinstance(value, (int, float)):
            try:
                return datetime.fromtimestamp(float(value), tz=UTC).isoformat()
            except (OSError, OverflowError, ValueError):
                pass
    return fallback.astimezone(UTC).isoformat()


def _file_mtime_iso(path: Path) -> str:
    try:
        return datetime.fromtimestamp(path.stat().st_mtime, tz=UTC).isoformat()
    except OSError:
        return datetime.now(tz=UTC).isoformat()


def _session_id_from_path(path: Path) -> str:
    stem = path.stem
    if stem and stem != path.name:
        return stem
    parent = path.parent.name
    return parent or stem or "unknown"


def _extract_session_id(obj: dict[str, Any], path: Path) -> str:
    for key in (
        "session_id",
        "sessionId",
        "conversation_id",
        "conversationId",
        "id",
        "uuid",
    ):
        value = obj.get(key)
        if isinstance(value, str) and value.strip():
            return value.strip()
    return _session_id_from_path(path)


def _text_from_message_content(content: Any) -> str | None:
    if isinstance(content, str):
        text = content.strip()
        return text or None
    if not isinstance(content, list):
        return None
    parts: list[str] = []
    for block in content:
        if not isinstance(block, dict):
            continue
        block_type = block.get("type")
        if block_type in ("tool_result", "tool_use"):
            continue
        if block_type == "text":
            text = block.get("text")
            if isinstance(text, str) and text.strip():
                parts.append(text.strip())
        elif isinstance(block.get("content"), str) and block.get("content", "").strip():
            parts.append(str(block["content"]).strip())
    if not parts:
        return None
    return "\n".join(parts).strip()


def _strip_user_query(text: str) -> str:
    match = USER_QUERY_RE.search(text)
    if match:
        return match.group(1).strip()
    return text.strip()


def _substantive_prompt(text: str) -> bool:
    cleaned = text.strip()
    if len(cleaned) < 2:
        return False
    noise = (
        "<local-command",
        "<command-name>",
        "<command-message>",
        "/clear",
        "/compact",
    )
    return not any(marker in cleaned for marker in noise)


def _matches_repo_filter(text: str, repo: str | None) -> bool:
    if not repo:
        return True
    haystack = text.casefold()
    needles = [repo.casefold()]
    if repo.casefold() == "hexakit":
        needles.extend(term.casefold() for term in REPO_FILTER_DEFAULT)
    return any(term in haystack for term in needles)


def _cursor_prompts(path: Path) -> Iterator[IntentPrompt]:
    session_id = _session_id_from_path(path)
    fallback_ts = datetime.fromisoformat(_file_mtime_iso(path))
    for lineno, obj in _iter_jsonl(path):
        if obj.get("role") != "user":
            continue
        message = obj.get("message")
        if not isinstance(message, dict):
            continue
        text = _text_from_message_content(message.get("content"))
        if not text:
            continue
        prompt = _strip_user_query(text)
        if not _substantive_prompt(prompt):
            continue
        yield IntentPrompt(
            timestamp=_iso_from_obj(obj, fallback_ts),
            source_tool="cursor",
            session_id=session_id,
            verbatim_prompt=prompt,
            file_path=str(path),
            line_number=lineno,
        )


def _claude_prompts(path: Path) -> Iterator[IntentPrompt]:
    fallback_ts = datetime.fromisoformat(_file_mtime_iso(path))
    for lineno, obj in _iter_jsonl(path):
        is_user = obj.get("type") == "user" or obj.get("role") == "user"
        if not is_user:
            continue
        message = obj.get("message")
        text: str | None = None
        if isinstance(message, dict):
            text = _text_from_message_content(message.get("content"))
        if not text:
            text = _text_from_message_content(obj.get("content"))
        if not text or not _substantive_prompt(text):
            continue
        yield IntentPrompt(
            timestamp=_iso_from_obj(obj, fallback_ts),
            source_tool="claude",
            session_id=_extract_session_id(obj, path),
            verbatim_prompt=text,
            file_path=str(path),
            line_number=lineno,
        )


def _codex_history_prompts(path: Path) -> Iterator[IntentPrompt]:
    fallback_ts = datetime.fromisoformat(_file_mtime_iso(path))
    for lineno, obj in _iter_jsonl(path):
        text = obj.get("text")
        if not isinstance(text, str) or not _substantive_prompt(text):
            continue
        session_id = obj.get("session_id")
        if not isinstance(session_id, str) or not session_id.strip():
            session_id = _session_id_from_path(path)
        yield IntentPrompt(
            timestamp=_iso_from_obj(obj, fallback_ts),
            source_tool="codex",
            session_id=session_id.strip(),
            verbatim_prompt=text.strip(),
            file_path=str(path),
            line_number=lineno,
        )


def _generic_user_prompts(path: Path, source_tool: str) -> Iterator[IntentPrompt]:
    """Best-effort extraction for Forge or unknown JSON/JSONL conversation logs."""
    fallback_ts = datetime.fromisoformat(_file_mtime_iso(path))
    suffix = path.suffix.lower()
    if suffix == ".jsonl":
        for lineno, obj in _iter_jsonl(path):
            prompt = _prompt_from_generic_obj(obj)
            if not prompt:
                continue
            yield IntentPrompt(
                timestamp=_iso_from_obj(obj, fallback_ts),
                source_tool=source_tool,
                session_id=_extract_session_id(obj, path),
                verbatim_prompt=prompt,
                file_path=str(path),
                line_number=lineno,
            )
        return

    if suffix != ".json":
        return

    try:
        raw = path.read_text(encoding="utf-8", errors="replace")
        data = json.loads(raw)
    except (OSError, json.JSONDecodeError):
        return

    if isinstance(data, list):
        for index, item in enumerate(data, 1):
            if not isinstance(item, dict):
                continue
            prompt = _prompt_from_generic_obj(item)
            if not prompt:
                continue
            yield IntentPrompt(
                timestamp=_iso_from_obj(item, fallback_ts),
                source_tool=source_tool,
                session_id=_extract_session_id(item, path),
                verbatim_prompt=prompt,
                file_path=str(path),
                line_number=index,
            )
        return

    if isinstance(data, dict):
        messages = data.get("messages")
        if isinstance(messages, list):
            for index, item in enumerate(messages, 1):
                if not isinstance(item, dict):
                    continue
                prompt = _prompt_from_generic_obj(item)
                if not prompt:
                    continue
                yield IntentPrompt(
                    timestamp=_iso_from_obj(item, fallback_ts),
                    source_tool=source_tool,
                    session_id=_extract_session_id(data, path),
                    verbatim_prompt=prompt,
                    file_path=str(path),
                    line_number=index,
                )


def _prompt_from_generic_obj(obj: dict[str, Any]) -> str | None:
    role = obj.get("role")
    msg_type = obj.get("type")
    if role not in (None, "user") and msg_type not in (None, "user", "human"):
        return None

    for key in ("prompt", "user_prompt", "input", "text", "content"):
        value = obj.get(key)
        if isinstance(value, str) and _substantive_prompt(value):
            return _strip_user_query(value)

    message = obj.get("message")
    if isinstance(message, dict):
        text = _text_from_message_content(message.get("content"))
        if text and _substantive_prompt(text):
            return _strip_user_query(text)
    if isinstance(message, str) and _substantive_prompt(message):
        return message.strip()

    text = _text_from_message_content(obj.get("content"))
    if text and _substantive_prompt(text):
        return _strip_user_query(text)
    return None


def _is_forge_conversation_file(path: Path) -> bool:
    name = path.name.casefold()
    if FORGE_CONVERSATION_NAME.search(name):
        return True
    for part in path.parts:
        if part.casefold() in {"conversations", "conversation", "sessions", "session"}:
            return True
    return False


def _is_codex_candidate(path: Path) -> bool:
    name = path.name.casefold()
    if name == "history.jsonl":
        return True
    markers = ("session", "conversation", "rollout", "history", "transcript")
    return any(marker in name for marker in markers)


def _discover_cursor_files(root: Path) -> list[Path]:
    if not root.is_dir():
        return []
    if root.name == "agent-transcripts" or (root / "agent-transcripts").is_dir():
        base = root if root.name == "agent-transcripts" else root / "agent-transcripts"
        return sorted(base.rglob("*.jsonl"))
    return sorted(root.rglob("**/agent-transcripts/**/*.jsonl"))


def _discover_claude_files(root: Path) -> list[Path]:
    if not root.is_dir():
        return []
    return sorted(root.rglob("*.jsonl"))


def _discover_forge_files(roots: list[Path]) -> list[Path]:
    found: list[Path] = []
    seen: set[Path] = set()
    for root in roots:
        if not root.is_dir():
            continue
        for path in root.rglob("*"):
            if not path.is_file():
                continue
            if path.suffix.lower() not in {".json", ".jsonl"}:
                continue
            if not _is_forge_conversation_file(path):
                continue
            resolved = path.resolve()
            if resolved in seen:
                continue
            seen.add(resolved)
            found.append(path)
    return sorted(found)


def _discover_codex_files(home: Path, codex_root: Path) -> list[Path]:
    found: list[Path] = []
    seen: set[Path] = set()

    def add(path: Path) -> None:
        if not path.is_file():
            return
        resolved = path.resolve()
        if resolved in seen:
            return
        seen.add(resolved)
        found.append(path)

    history = codex_root / "history.jsonl"
    if history.is_file():
        add(history)

    search_roots = [
        codex_root / "sessions",
        codex_root / "log",
        codex_root,
        home / "codex-work",
    ]
    for root in search_roots:
        if not root.is_dir():
            continue
        for path in root.rglob("*"):
            if not path.is_file():
                continue
            if path.suffix.lower() not in {".json", ".jsonl"}:
                continue
            if path.name.casefold() == "history.jsonl" or _is_codex_candidate(path):
                add(path)
    return sorted(found)


def _first_only(rows: list[IntentPrompt]) -> list[IntentPrompt]:
    seen: set[tuple[str, str]] = set()
    out: list[IntentPrompt] = []
    for row in rows:
        key = (row.source_tool, row.session_id)
        if key in seen:
            continue
        seen.add(key)
        out.append(row)
    return out


def _yaml_escape_scalar(value: str) -> str:
    if not value:
        return '""'
    if "\n" in value or value[0] in "-?:#&*!|>'\"%@`" or value.endswith(" "):
        return "|-\n" + "\n".join(f"  {line}" for line in value.splitlines())
    if value.isdigit() or value in {"true", "false", "null", "yes", "no", "on", "off"}:
        return json.dumps(value)
    if any(ch in value for ch in (":", "{", "}", "[", "]", ",")):
        return json.dumps(value)
    return value


def _render_yaml(rows: list[IntentPrompt]) -> str:
    lines = ["prompts:"]
    for row in rows:
        lines.append("  - timestamp: " + _yaml_escape_scalar(row.timestamp))
        lines.append("    source_tool: " + _yaml_escape_scalar(row.source_tool))
        lines.append("    session_id: " + _yaml_escape_scalar(row.session_id))
        lines.append("    file_path: " + _yaml_escape_scalar(row.file_path))
        if row.line_number is not None:
            lines.append(f"    line_number: {row.line_number}")
        lines.append("    verbatim_prompt: " + _yaml_escape_scalar(row.verbatim_prompt))
    return "\n".join(lines) + "\n"


def _render_markdown(rows: list[IntentPrompt]) -> str:
    chunks: list[str] = ["# Intent prompt provenance", ""]
    for index, row in enumerate(rows, 1):
        chunks.extend(
            [
                f"## Prompt {index}",
                "",
                f"- **timestamp**: {row.timestamp}",
                f"- **source_tool**: {row.source_tool}",
                f"- **session_id**: `{row.session_id}`",
                f"- **file_path**: `{row.file_path}`",
            ]
        )
        if row.line_number is not None:
            chunks.append(f"- **line_number**: {row.line_number}")
        chunks.extend(["", "### verbatim_prompt", "", "```text", row.verbatim_prompt, "```", ""])
    return "\n".join(chunks)


def collect_prompts(
    *,
    home: Path,
    cursor_root: Path | None,
    forge_roots: list[Path],
    claude_root: Path | None,
    codex_root: Path | None,
    sources: set[str],
    all_messages: bool,
    repo_filter: str | None,
) -> list[IntentPrompt]:
    rows: list[IntentPrompt] = []

    if "cursor" in sources and cursor_root is not None:
        for path in _discover_cursor_files(cursor_root):
            rows.extend(_cursor_prompts(path))

    if "forge" in sources:
        for path in _discover_forge_files(forge_roots):
            rows.extend(_generic_user_prompts(path, "forge"))

    if "claude" in sources and claude_root is not None:
        for path in _discover_claude_files(claude_root):
            rows.extend(_claude_prompts(path))

    if "codex" in sources and codex_root is not None:
        for path in _discover_codex_files(home, codex_root):
            if path.name.casefold() == "history.jsonl":
                rows.extend(_codex_history_prompts(path))
            else:
                rows.extend(_generic_user_prompts(path, "codex"))

    rows = [row for row in rows if _matches_repo_filter(row.verbatim_prompt, repo_filter)]
    rows.sort(key=lambda row: (row.timestamp, row.source_tool, row.session_id, row.file_path))
    if not all_messages:
        rows = _first_only(rows)
    return rows


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Extract verbatim user prompts from agent session logs.",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python scripts/extract-intent-prompts.py --repo HexaKit
  python scripts/extract-intent-prompts.py --source cursor --format markdown
  python scripts/extract-intent-prompts.py --all --out-dir docs/intent/prompts
        """,
    )
    parser.add_argument(
        "--home",
        type=Path,
        default=Path.home(),
        help="User home directory (default: Path.home())",
    )
    parser.add_argument(
        "--cursor-dir",
        type=Path,
        default=None,
        help="Cursor agent-transcripts root (default: ~/.cursor/projects/*/agent-transcripts)",
    )
    parser.add_argument(
        "--forge-dir",
        action="append",
        default=[],
        help="Forge root to scan for conversation json/jsonl (repeatable)",
    )
    parser.add_argument(
        "--claude-dir",
        type=Path,
        default=None,
        help="Claude Code projects root (default: ~/.claude/projects)",
    )
    parser.add_argument(
        "--codex-dir",
        type=Path,
        default=None,
        help="Codex data root (default: ~/.codex)",
    )
    parser.add_argument(
        "--source",
        action="append",
        choices=("cursor", "forge", "claude", "codex", "all"),
        default=[],
        help="Limit to one or more sources (default: all)",
    )
    parser.add_argument(
        "--all",
        dest="all_messages",
        action="store_true",
        help="Emit every user message, not just the first per session",
    )
    parser.add_argument(
        "--repo",
        default="",
        help="Keep prompts whose text mentions this repo (HexaKit also matches genesis)",
    )
    parser.add_argument(
        "--format",
        choices=("yaml", "markdown"),
        default="yaml",
        help="Output format (default: yaml)",
    )
    parser.add_argument(
        "--out-dir",
        type=Path,
        default=None,
        help="Write intent-prompts.<ext> here instead of stdout",
    )
    return parser


def _configure_stdout() -> None:
    if hasattr(sys.stdout, "reconfigure"):
        try:
            sys.stdout.reconfigure(encoding="utf-8", errors="replace")
        except (AttributeError, OSError, ValueError):
            pass


def main(argv: list[str] | None = None) -> int:
    _configure_stdout()
    parser = build_parser()
    args = parser.parse_args(argv)

    home: Path = args.home.expanduser()
    sources = set(args.source or ["all"])
    if "all" in sources:
        sources = {"cursor", "forge", "claude", "codex"}

    cursor_root = args.cursor_dir
    if cursor_root is None:
        cursor_root = home / ".cursor" / "projects"

    forge_roots = [Path(p).expanduser() for p in args.forge_dir]
    if not forge_roots:
        forge_roots = [home / "forge", home / ".forge"]

    claude_root = args.claude_dir or (home / ".claude" / "projects")
    codex_root = args.codex_dir or (home / ".codex")

    repo_filter = args.repo.strip() or None

    rows = collect_prompts(
        home=home,
        cursor_root=cursor_root,
        forge_roots=forge_roots,
        claude_root=claude_root,
        codex_root=codex_root,
        sources=sources,
        all_messages=args.all_messages,
        repo_filter=repo_filter,
    )

    rendered = _render_yaml(rows) if args.format == "yaml" else _render_markdown(rows)

    if args.out_dir:
        out_dir = args.out_dir.expanduser()
        out_dir.mkdir(parents=True, exist_ok=True)
        ext = "yaml" if args.format == "yaml" else "md"
        out_path = out_dir / f"intent-prompts.{ext}"
        out_path.write_text(rendered, encoding="utf-8")
        print(f"Wrote {len(rows)} prompt(s) to {out_path}", file=sys.stderr)
    else:
        sys.stdout.write(rendered)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
