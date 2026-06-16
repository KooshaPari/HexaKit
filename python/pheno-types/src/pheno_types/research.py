"""
Research type definitions with Pydantic validation.

Traces to: FR-TYPES-003, FR-TYPES-004, FR-TYPES-007
"""

from typing import List, Optional, Any, Dict
from dataclasses import dataclass, field
from pydantic import BaseModel, Field, ConfigDict, field_validator
from datetime import datetime
from uuid import UUID, uuid4


class ConfidenceScore(BaseModel):
    """Confidence score with range validation.
    
    Traces to: FR-TYPES-004
    """
    model_config = ConfigDict(strict=True)
    
    value: float = Field(..., ge=0.0, le=1.0, description="Confidence value between 0.0 and 1.0")
    
    @field_validator("value")
    @classmethod
    def validate_range(cls, v: float) -> float:
        """Validate confidence is in [0.0, 1.0]."""
        if not 0.0 <= v <= 1.0:
            raise ValueError(f"ConfidenceScore must be in [0.0, 1.0], got {v}")
        return v


class Citation(BaseModel):
    """Citation model.
    
    Traces to: FR-TYPES-003
    """
    model_config = ConfigDict(strict=True)
    
    source: str = Field(..., min_length=1, description="Source identifier")
    url: Optional[str] = Field(None, description="URL to source")
    timestamp: Optional[datetime] = Field(None, description="When accessed")
    quote: Optional[str] = Field(None, description="Relevant quote from source")


class Evidence(BaseModel):
    """Evidence model.
    
    Traces to: FR-TYPES-003
    """
    model_config = ConfigDict(strict=True)
    
    id: str = Field(default_factory=lambda: str(uuid4()))
    claim: str = Field(..., min_length=1, description="The claim being made")
    confidence: ConfidenceScore = Field(..., description="Confidence in this evidence")
    citations: List[Citation] = Field(default_factory=list, description="Supporting citations")
    metadata: Optional[Dict[str, Any]] = Field(None, description="Additional metadata")


class ResearchReport(BaseModel):
    """Research report model.
    
    Traces to: FR-TYPES-003
    """
    model_config = ConfigDict(strict=True)
    
    id: str = Field(default_factory=lambda: str(uuid4()))
    title: str = Field(..., min_length=1, description="Report title")
    summary: str = Field(..., min_length=1, description="Executive summary")
    evidence: List[Evidence] = Field(default_factory=list, description="Supporting evidence")
    confidence_overall: ConfidenceScore = Field(..., description="Overall confidence")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    metadata: Optional[Dict[str, Any]] = Field(None, description="Additional metadata")
