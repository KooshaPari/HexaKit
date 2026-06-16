"""
Task type definitions.

Traces to: FR-TYPES-001, FR-TYPES-002
"""

from enum import Enum
from typing import TypedDict, NewType, Optional, Any, Dict
from dataclasses import dataclass
from uuid import UUID, uuid4

# Branded type for Task IDs
TaskID = NewType("TaskID", str)


class TaskState(str, Enum):
    """Task state enum.
    
    Traces to: FR-TYPES-002
    """
    PENDING = "pending"
    RUNNING = "running"
    SUCCEEDED = "succeeded"
    FAILED = "failed"
    TIMED_OUT = "timed_out"


class TaskResult(TypedDict, total=False):
    """Task result type.
    
    Traces to: FR-TYPES-001
    """
    success: bool
    data: Optional[Any]
    error: Optional[str]
    duration_ms: Optional[int]


@dataclass(frozen=True)
class Task:
    """Task dataclass.
    
    Traces to: FR-TYPES-001
    """
    id: TaskID
    name: str
    state: TaskState
    result: Optional[TaskResult] = None
    parent_id: Optional[TaskID] = None
    metadata: Optional[Dict[str, Any]] = None
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary for JSON serialization."""
        return {
            "id": self.id,
            "name": self.name,
            "state": self.state.value,
            "result": self.result,
            "parent_id": self.parent_id,
            "metadata": self.metadata,
        }


def create_task_id(uuid: UUID) -> TaskID:
    """Create a TaskID from a UUID."""
    return TaskID(str(uuid))


def generate_task_id() -> TaskID:
    """Generate a new TaskID."""
    return TaskID(str(uuid4()))
