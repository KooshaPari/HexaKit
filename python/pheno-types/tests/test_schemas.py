"""Tests for schema export utilities.

Traces to: FR-TYPES-006
"""

import pytest
from pheno_types import (
    export,
    export_all,
    list_types,
    register_type,
)
from pheno_types.research import ConfidenceScore, ResearchReport


# ============================================================================
# Schema Export Tests
# ============================================================================

# Traces to: FR-TYPES-006
def test_export_single_type():
    """Test exporting single type schema."""
    schema = export("ConfidenceScore")
    assert "properties" in schema
    assert "value" in schema["properties"]


# Traces to: FR-TYPES-006
def test_export_all_types():
    """Test exporting all registered types."""
    all_schemas = export_all()
    assert isinstance(all_schemas, dict)
    assert len(all_schemas) > 0
    
    # Check for expected types
    assert "ConfidenceScore" in all_schemas
    assert "ResearchReport" in all_schemas


# Traces to: FR-TYPES-006
def test_list_types():
    """Test listing registered types."""
    types = list_types()
    assert isinstance(types, list)
    assert len(types) > 0
    assert "ConfidenceScore" in types
    assert "ResearchReport" in types


# Traces to: FR-TYPES-006
def test_export_nonexistent_type():
    """Test exporting non-existent type raises error."""
    with pytest.raises(KeyError) as exc_info:
        export("NonExistentType")
    
    assert "not found in registry" in str(exc_info.value)
    assert "NonExistentType" in str(exc_info.value)


# Traces to: FR-TYPES-006
def test_export_includes_type_info():
    """Test exported schema includes type information."""
    schema = export("ResearchReport")
    
    # Check structure
    assert "title" in schema
    assert "type" in schema
    assert schema["type"] == "object"
    assert "properties" in schema
    
    # Check for expected properties
    props = schema["properties"]
    assert "id" in props
    assert "title" in props
    assert "summary" in props
    assert "evidence" in props
