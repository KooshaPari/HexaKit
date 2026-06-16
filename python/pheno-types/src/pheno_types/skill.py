"""
Skill type definitions.

Traces to: FR-TYPES-005
"""

from typing import TypedDict, Optional, Any, Dict, List
from dataclasses import dataclass, field
from pydantic import BaseModel, Field, ConfigDict


class SkillInput(BaseModel):
    """Skill input model.
    
    Traces to: FR-TYPES-005
    """
    model_config = ConfigDict(strict=True)
    
    name: str = Field(..., description="Input parameter name")
    type: str = Field(..., description="Type annotation (e.g., 'str', 'int', 'List[str]')")
    description: Optional[str] = Field(None, description="Input description")
    required: bool = Field(True, description="Whether this input is required")
    default: Optional[Any] = Field(None, description="Default value if not provided")


class SkillOutput(BaseModel):
    """Skill output model.
    
    Traces to: FR-TYPES-005
    """
    model_config = ConfigDict(strict=True)
    
    name: str = Field(..., description="Output name")
    type: str = Field(..., description="Type annotation")
    description: Optional[str] = Field(None, description="Output description")


class SkillManifest(BaseModel):
    """Skill manifest - describes a skill's interface.
    
    Traces to: FR-TYPES-005
    """
    model_config = ConfigDict(strict=True)
    
    name: str = Field(..., min_length=1, description="Skill name")
    version: str = Field(default="1.0.0", description="Skill version")
    description: Optional[str] = Field(None, description="Skill description")
    inputs: List[SkillInput] = Field(default_factory=list, description="Input parameters")
    outputs: List[SkillOutput] = Field(default_factory=list, description="Output parameters")
    tags: List[str] = Field(default_factory=list, description="Skill tags")
    metadata: Optional[Dict[str, Any]] = Field(None, description="Additional metadata")


@dataclass(frozen=True)
class Skill:
    """Skill dataclass.
    
    Traces to: FR-TYPES-005
    """
    manifest: SkillManifest
    handler: Optional[Any] = None  # Runtime handler (not serialized)
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary for JSON serialization."""
        return {
            "manifest": self.manifest.model_dump(),
            "has_handler": self.handler is not None,
        }
