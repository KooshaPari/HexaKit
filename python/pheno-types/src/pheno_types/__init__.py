"""Core types for Phenotype.

Traces to:
- FR-TYPES-001: Task Types
- FR-TYPES-002: Task State Enum
- FR-TYPES-003: Research Types
- FR-TYPES-004: Confidence Score Validation
- FR-TYPES-005: Skill Types
- FR-TYPES-006: JSON Schema Export
- FR-TYPES-007: Strict Pydantic Config
- FR-TYPES-008: Zero Runtime Dependencies
"""

# Task types (FR-TYPES-001, FR-TYPES-002)
from .task import Task, TaskID, TaskState, TaskResult, create_task_id, generate_task_id

# Research types (FR-TYPES-003, FR-TYPES-004)
from .research import (
    ConfidenceScore,
    Citation,
    Evidence,
    ResearchReport,
)

# Skill types (FR-TYPES-005)
from .skill import Skill, SkillManifest, SkillInput, SkillOutput

# Schema export (FR-TYPES-006)
from .schemas import export, export_all, list_types, register_type

# Legacy types (for backward compatibility)
from .legacy import RFQState, OrderState, ShippingState

# Register Pydantic models for schema export
from . import schemas

schemas.register_type("ConfidenceScore", ConfidenceScore)
schemas.register_type("Citation", Citation)
schemas.register_type("Evidence", Evidence)
schemas.register_type("ResearchReport", ResearchReport)
schemas.register_type("SkillInput", SkillInput)
schemas.register_type("SkillOutput", SkillOutput)
schemas.register_type("SkillManifest", SkillManifest)

__all__ = [
    # Task types
    "Task",
    "TaskID",
    "TaskState",
    "TaskResult",
    "create_task_id",
    "generate_task_id",
    # Research types
    "ConfidenceScore",
    "Citation",
    "Evidence",
    "ResearchReport",
    # Skill types
    "Skill",
    "SkillManifest",
    "SkillInput",
    "SkillOutput",
    # Schema export
    "export",
    "export_all",
    "list_types",
    "register_type",
    # Legacy types
    "RFQState",
    "OrderState",
    "ShippingState",
]

__version__ = "0.1.0"
