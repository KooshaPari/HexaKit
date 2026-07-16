# extract-intent-prompts.py

Scrape verbatim user prompts from local agent session logs for `docs/intent/` provenance.

No network access. Python 3.10+ stdlib only (`pathlib`, `json`, `argparse`, `re`, `datetime`).

## Sources

| Source | Default location | Notes |
|--------|------------------|-------|
| **cursor** | `~/.cursor/projects/*/agent-transcripts/**/*.jsonl` | Parses `role=user`; strips `<user_query>` tags |
| **forge** | `~/forge` (not `~/.forge` on this machine) | Heuristic JSON/JSONL under `conversation*` paths; **1,190 conversations** in `~/forge/.forge.db` — use `forge conversation dump <id> --format json` for full prompts until DB export is wired into this script |
| **claude** | `~/.claude/projects/**/*.jsonl` | Claude Code `type=user` messages |
| **codex** | `~/.codex/history.jsonl`, `~/.codex/sessions/**` | `sessions/2026/` dirs exist; JSONL may be sparse — listing can fail if tree is empty |

## Output fields

Each record includes:

- `timestamp` — ISO-8601 UTC when available (else file mtime)
- `source_tool` — `cursor`, `forge`, `claude`, or `codex`
- `session_id` — from log metadata or file name
- `verbatim_prompt` — user text as stored in the log
- `file_path` — absolute path to the source file
- `line_number` — JSONL line when applicable

## Usage

```bash
# First user prompt per session, HexaKit/genesis mentions only, YAML to stdout
python scripts/extract-intent-prompts.py --repo HexaKit

# All user messages from Cursor only, Markdown file
python scripts/extract-intent-prompts.py --source cursor --all --format markdown --out-dir docs/intent/prompts

# Explicit roots
python scripts/extract-intent-prompts.py \
  --cursor-dir "$HOME/.cursor/projects/C-Users-koosh/agent-transcripts" \
  --claude-dir "$HOME/.claude/projects" \
  --codex-dir "$HOME/.codex" \
  --forge-dir "$HOME/forge" \
  --repo HexaKit \
  --out-dir docs/intent/prompts
```

## Flags

| Flag | Description |
|------|-------------|
| `--home DIR` | Home directory for default paths (default: `Path.home()`) |
| `--cursor-dir DIR` | Cursor `agent-transcripts` root |
| `--forge-dir DIR` | Forge root (repeatable) |
| `--claude-dir DIR` | Claude Code projects root |
| `--codex-dir DIR` | Codex data root |
| `--source {cursor,forge,claude,codex,all}` | Limit sources (repeatable; default: all) |
| `--all` | Every user message (default: first per session) |
| `--repo NAME` | Filter prompts mentioning `NAME` (`HexaKit` also matches `genesis`) |
| `--format {yaml,markdown}` | Output format (default: `yaml`) |
| `--out-dir DIR` | Write `intent-prompts.yaml` or `intent-prompts.md` |

## Integration with genesis intent docs

Typical flow for HexaKit genesis scaffolding:

1. Run with `--repo HexaKit --out-dir docs/intent/prompts`.
2. Review `intent-prompts.yaml` and copy curated entries into `docs/intent/` or synthesize in `docs/intent/synthesis.md`.
3. Re-run after major agent sessions to refresh provenance.

## Limitations

- Forge stores most history in **`~/forge/.forge.db`** (SQLite), not loose JSONL — heuristic scan may miss prompts; manual: `forge conversation list` then `forge conversation dump <id> --format json`.
- `~/.forge` is **not present** here; pass `--forge-dir C:/Users/koosh/forge` if needed.
- Codex `sessions/` trees can be large or empty; only filename patterns like `history`, `session`, `conversation`, and `rollout` are scanned.
- Tool-result-only Claude user lines (empty human text) are skipped.

## forge `-p` parallel lanes

Launch from **Git Bash** (PowerShell `bash` may not resolve the path):

```bash
bash "C:/Users/koosh/.claude/tools/fp.sh" <job-id> <promptfile> <cwd>
```

The SOTA dimension forge lane failed with exit 127 (path not found); content was filled directly in-repo instead.
