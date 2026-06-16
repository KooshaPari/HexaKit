"""Tests for skill types.

Traces to: FR-TYPES-005
"""

import pytest
from phenotype_types import (
    Skill,
    SkillManifest,
    SkillInput,
    SkillOutput,
)


# ============================================================================
# SkillInput Tests
# ============================================================================

# Traces to: FR-TYPES-005
def test_skill_input_minimal():
    """Test minimal SkillInput creation."""
    input_def = SkillInput(name="query", type="str")
    assert input_def.name == "query"
    assert input_def.type == "str"
    assert input_def.required is True  # Default
    assert input_def.description is None
    assert input_def.default is None


# Traces to: FR-TYPES-005
def test_skill_input_full():
    """Test full SkillInput creation."""
    input_def = SkillInput(
        name="limit",
        type="int",
        description="Maximum results",
        required=False,
        default=10,
    )
    assert input_def.name == "limit"
    assert input_def.type == "int"
    assert input_def.description == "Maximum results"
    assert input_def.required is False
    assert input_def.default == 10


# ============================================================================
# SkillOutput Tests
# ============================================================================

# Traces to: FR-TYPES-005
def test_skill_output_creation():
    """Test SkillOutput creation."""
    output_def = SkillOutput(name="results", type="List[Dict]")
    assert output_def.name == "results"
    assert output_def.type == "List[Dict]"
    assert output_def.description is None


# Traces to: FR-TYPES-005
def test_skill_output_with_description():
    """Test SkillOutput with description."""
    output_def = SkillOutput(
        name="count",
        type="int",
        description="Number of results",
    )
    assert output_def.description == "Number of results"


# ============================================================================
# SkillManifest Tests
# ============================================================================

# Traces to: FR-TYPES-005
def test_skill_manifest_minimal():
    """Test minimal SkillManifest creation."""
    manifest = SkillManifest(name="test-skill")
    assert manifest.name == "test-skill"
    assert manifest.version == "1.0.0"  # Default
    assert manifest.description is None
    assert manifest.inputs == []
    assert manifest.outputs == []
    assert manifest.tags == []


# Traces to: FR-TYPES-005
def test_skill_manifest_full():
    """Test full SkillManifest creation."""
    input1 = SkillInput(name="query", type="str")
    output1 = SkillOutput(name="result", type="str")
    
    manifest = SkillManifest(
        name="search-skill",
        version="2.1.0",
        description="Performs a search",
        inputs=[input1],
        outputs=[output1],
        tags=["search", "api"],
    )
    assert manifest.name == "search-skill"
    assert manifest.version == "2.1.0"
    assert manifest.description == "Performs a search"
    assert len(manifest.inputs) == 1
    assert len(manifest.outputs) == 1
    assert "search" in manifest.tags


# ============================================================================
# Skill Tests
# ============================================================================

# Traces to: FR-TYPES-005
def test_skill_creation():
    """Test Skill creation."""
    manifest = SkillManifest(name="test-skill")
    skill = Skill(manifest=manifest)
    assert skill.manifest == manifest
    assert skill.handler is None


# Traces to: FR-TYPES-005
def test_skill_with_handler():
    """Test Skill with handler."""
    manifest = SkillManifest(name="handler-skill")
    
    def handler(input_data):
        return {"result": "success"}
    
    skill = Skill(manifest=manifest, handler=handler)
    assert skill.handler is not None
    assert skill.handler({}) == {"result": "success"}


# Traces to: FR-TYPES-005
def test_skill_to_dict():
    """Test Skill.to_dict() serialization."""
    manifest = SkillManifest(
        name="dict-test",
        version="1.0.0",
    )
    skill = Skill(manifest=manifest)
    
    d = skill.to_dict()
    assert d["manifest"]["name"] == "dict-test"
    assert d["manifest"]["version"] == "1.0.0"
    assert d["has_handler"] is False


# Traces to: FR-TYPES-005
def test_skill_to_dict_with_handler():
    """Test Skill.to_dict() with handler."""
    manifest = SkillManifest(name="handler-test")
    skill = Skill(manifest=manifest, handler=lambda x: x)
    
    d = skill.to_dict()
    assert d["has_handler"] is True


# Traces to: FR-TYPES-005, FR-TYPES-006
def test_skill_manifest_schema():
    """Test SkillManifest JSON schema generation."""
    schema = SkillManifest.model_json_schema()
    assert "name" in schema["properties"]
    assert "version" in schema["properties"]
    assert "description" in schema["properties"]
