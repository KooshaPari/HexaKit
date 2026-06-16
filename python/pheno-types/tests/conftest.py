"""pytest configuration and fixtures."""

import pytest
from pheno_types import register_type
from pheno_types.research import ConfidenceScore, Citation, Evidence, ResearchReport
from pheno_types.skill import SkillInput, SkillOutput, SkillManifest

# Register Pydantic models for schema tests
# This is done here to ensure types are registered before tests run
register_type("ConfidenceScore", ConfidenceScore)
register_type("Citation", Citation)
register_type("Evidence", Evidence)
register_type("ResearchReport", ResearchReport)
register_type("SkillInput", SkillInput)
register_type("SkillOutput", SkillOutput)
register_type("SkillManifest", SkillManifest)


@pytest.fixture
def sample_confidence():
    """Fixture for a sample ConfidenceScore."""
    return ConfidenceScore(value=0.8)


@pytest.fixture
def sample_citation():
    """Fixture for a sample Citation."""
    return Citation(
        source="example.com",
        url="https://example.com/article",
    )


@pytest.fixture
def sample_evidence(sample_confidence, sample_citation):
    """Fixture for sample Evidence."""
    return Evidence(
        claim="Test claim",
        confidence=sample_confidence,
        citations=[sample_citation],
    )
