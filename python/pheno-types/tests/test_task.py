"""Tests for task types.

Traces to: FR-TYPES-001, FR-TYPES-002
"""

import pytest
from uuid import UUID, uuid4
from pheno_types import (
    Task,
    TaskID,
    TaskState,
    TaskResult,
    create_task_id,
    generate_task_id,
)


# ============================================================================
# TaskID Tests
# ============================================================================

# Traces to: FR-TYPES-001
def test_create_task_id_from_uuid():
    """Test creating TaskID from UUID."""
    uuid = uuid4()
    task_id = create_task_id(uuid)
    assert isinstance(task_id, str)
    assert task_id == str(uuid)


# Traces to: FR-TYPES-001
def test_generate_task_id_unique():
    """Test that generated TaskIDs are unique."""
    id1 = generate_task_id()
    id2 = generate_task_id()
    assert isinstance(id1, str)
    assert isinstance(id2, str)
    assert id1 != id2
    # Verify they are valid UUIDs
    UUID(id1)
    UUID(id2)


# ============================================================================
# Task Tests
# ============================================================================

# Traces to: FR-TYPES-001
def test_task_creation():
    """Test basic Task creation."""
    task_id = generate_task_id()
    task = Task(
        id=task_id,
        name="test-task",
        state=TaskState.PENDING,
    )
    assert task.id == task_id
    assert task.name == "test-task"
    assert task.state == TaskState.PENDING
    assert task.result is None
    assert task.parent_id is None
    assert task.metadata is None


# Traces to: FR-TYPES-001
def test_task_with_result():
    """Test Task with result."""
    task = Task(
        id=generate_task_id(),
        name="completed-task",
        state=TaskState.SUCCEEDED,
        result={
            "success": True,
            "data": {"key": "value"},
            "duration_ms": 100,
        },
    )
    assert task.result["success"] is True
    assert task.result["data"]["key"] == "value"


# Traces to: FR-TYPES-001
def test_task_with_parent():
    """Test Task with parent_id."""
    parent_id = generate_task_id()
    child = Task(
        id=generate_task_id(),
        name="child-task",
        state=TaskState.PENDING,
        parent_id=parent_id,
    )
    assert child.parent_id == parent_id


# Traces to: FR-TYPES-001
def test_task_with_metadata():
    """Test Task with metadata."""
    task = Task(
        id=generate_task_id(),
        name="task-with-meta",
        state=TaskState.PENDING,
        metadata={"priority": "high", "tags": ["urgent"]},
    )
    assert task.metadata["priority"] == "high"
    assert "urgent" in task.metadata["tags"]


# Traces to: FR-TYPES-001
def test_task_to_dict():
    """Test Task.to_dict() serialization."""
    task = Task(
        id=generate_task_id(),
        name="test-task",
        state=TaskState.RUNNING,
    )
    d = task.to_dict()
    assert d["name"] == "test-task"
    assert d["state"] == "running"
    assert "id" in d


# ============================================================================
# TaskState Tests
# ============================================================================

# Traces to: FR-TYPES-002
def test_task_state_enum_values():
    """Test TaskState enum values."""
    assert TaskState.PENDING.value == "pending"
    assert TaskState.RUNNING.value == "running"
    assert TaskState.SUCCEEDED.value == "succeeded"
    assert TaskState.FAILED.value == "failed"
    assert TaskState.TIMED_OUT.value == "timed_out"


# Traces to: FR-TYPES-002
def test_task_state_from_string():
    """Test creating TaskState from string."""
    assert TaskState("pending") == TaskState.PENDING
    assert TaskState("running") == TaskState.RUNNING
    assert TaskState("succeeded") == TaskState.SUCCEEDED
    assert TaskState("failed") == TaskState.FAILED
    assert TaskState("timed_out") == TaskState.TIMED_OUT


# Traces to: FR-TYPES-002
def test_task_state_invalid_value():
    """Test that invalid state raises ValueError."""
    with pytest.raises(ValueError):
        TaskState("invalid_state")


# ============================================================================
# TaskResult Tests
# ============================================================================

# Traces to: FR-TYPES-001
def test_task_result_optional_fields():
    """Test TaskResult with optional fields."""
    result_minimal: TaskResult = {"success": True}
    assert result_minimal["success"] is True
    
    result_full: TaskResult = {
        "success": False,
        "error": "Something went wrong",
        "data": None,
        "duration_ms": 500,
    }
    assert result_full["success"] is False
    assert result_full["error"] == "Something went wrong"
