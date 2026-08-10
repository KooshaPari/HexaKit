"""Type stubs for the `behave` BDD framework.

Minimal stub covering only the symbol surface used by HexaKit's
`python/tests/bdd/steps/{given,when,then}_steps.py` (v0.11 backlog Lane
F2). The real `behave` package is untyped, so this stub lets mypy
validate the step definitions without `# type: ignore[import]`
annotations.

Usage: add `mypy_path = "tests/stubs"` to `[tool.mypy]` in
`pyproject.toml` (or invoke `mypy --custom-typeshed-dir tests/stubs`).
"""

from __future__ import annotations

from typing import Any, Callable, TypeVar

F = TypeVar("F", bound=Callable[..., Any])


class Context:
    """Stub for behave.runner.Context.

    Behave populates arbitrary attributes on the context object. The
    stub types it as `Any` so attributes can be assigned freely.
    """

    # Any attribute access is permitted at the stub level.
    def __getattr__(self, name: str) -> Any: ...
    def __setattr__(self, name: str, value: Any) -> None: ...


def given(step_text: str, **kwargs: Any) -> Callable[[F], F]:
    """Decorator factory for a Given step."""
    def decorator(func: F) -> F:
        return func
    return decorator


def when(step_text: str, **kwargs: Any) -> Callable[[F], F]:
    """Decorator factory for a When step."""
    def decorator(func: F) -> F:
        return func
    return decorator


def then(step_text: str, **kwargs: Any) -> Callable[[F], F]:
    """Decorator factory for a Then step."""
    def decorator(func: F) -> F:
        return func
    return decorator


def step(step_text: str, **kwargs: Any) -> Callable[[F], F]:
    """Decorator factory for a generic step (Given/When/Then-agnostic)."""
    def decorator(func: F) -> F:
        return func
    return decorator


__all__ = ["Context", "given", "when", "then", "step"]
