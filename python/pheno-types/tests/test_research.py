"""Tests for research types.

Traces to: FR-TYPES-003, FR-TYPES-004
"""

import pytest
from datetime import datetime
from phenotype_types import (
    ConfidenceScore,
    Citation,
    Evidence,
    ResearchReport,
)


# ============================================================================
# ConfidenceScore Tests
# ============================================================================

# Traces to: FR-TYPES-004
def test_confidence_score_valid():
    """Test valid confidence scores."""
    score = ConfidenceScore(value=0.5)
    assert score.value == 0.5
    
    score_max = ConfidenceScore(value=1.0)
    assert score_max.value == 1.0
    
    score_min = ConfidenceScore(value=0.0)
    assert score_min.value == 0.0


# Traces to: FR-TYPES-004
def test_confidence_score_out_of_range():
    """Test that out-of-range confidence scores raise error."""
    with pytest.raises(ValueError):
        ConfidenceScore(value=1.5)
    
    with pytest.raises(ValueError):
        ConfidenceScore(value=-0.1)


# Traces to: FR-TYPES-004
def test_confidence_score_pydantic_validation():
    """Test Pydantic validation of confidence scores."""
    # Valid at boundaries
    assert ConfidenceScore(value=0.0).value == 0.0
    assert ConfidenceScore(value=1.0).value == 1.0
    
    # Valid in middle
    score = ConfidenceScore(value=0.75)
    assert score.value == 0.75


# ============================================================================
# Citation Tests
# ============================================================================

# Traces to: FR-TYPES-003
def test_citation_minimal():
    """Test minimal Citation creation."""
    citation = Citation(source="example.com")
    assert citation.source == "example.com"
    assert citation.url is None
    assert citation.timestamp is None
    assert citation.quote is None


# Traces to: FR-TYPES-003
def test_citation_full():
    """Test full Citation creation."""
    now = datetime.utcnow()
    citation = Citation(
        source="example.com/page",
        url="https://example.com/page",
        timestamp=now,
        quote="This is a quote",
    )
    assert citation.url == "https://example.com/page"
    assert citation.timestamp == now
    assert citation.quote == "This is a quote"


# ============================================================================
# Evidence Tests
# ============================================================================

# Traces to: FR-TYPES-003
def test_evidence_creation():
    """Test Evidence creation."""
    evidence = Evidence(
        claim="The sky is blue",
        confidence=ConfidenceScore(value=0.95),
    )
    assert evidence.claim == "The sky is blue"
    assert evidence.confidence.value == 0.95
    assert evidence.citations == []  # Default empty list


# Traces to: FR-TYPES-003
def test_evidence_with_citations():
    """Test Evidence with citations."""
    citation1 = Citation(source="source1.com")
    citation2 = Citation(source="source2.com")
    
    evidence = Evidence(
        claim="Important fact",
        confidence=ConfidenceScore(value=0.8),
        citations=[citation1, citation2],
    )
    assert len(evidence.citations) == 2
    assert evidence.citations[0].source == "source1.com"


# Traces to: FR-TYPES-003
def test_evidence_with_metadata():
    """Test Evidence with metadata."""
    evidence = Evidence(
        claim="Test claim",
        confidence=ConfidenceScore(value=0.7),
        metadata={"category": "science", "verified": True},
    )
    assert evidence.metadata["category"] == "science"


# ============================================================================
# ResearchReport Tests
# ============================================================================

# Traces to: FR-TYPES-003
def test_research_report_creation():
    """Test ResearchReport creation."""
    report = ResearchReport(
        title="Test Report",
        summary="This is a summary",
        confidence_overall=ConfidenceScore(value=0.85),
    )
    assert report.title == "Test Report"
    assert report.summary == "This is a summary"
    assert report.confidence_overall.value == 0.85
    assert report.evidence == []  # Default empty list
    assert isinstance(report.created_at, datetime)


# Traces to: FR-TYPES-003
def test_research_report_with_evidence():
    """Test ResearchReport with evidence."""
    evidence1 = Evidence(
        claim="Claim one",
        confidence=ConfidenceScore(value=0.9),
    )
    evidence2 = Evidence(
        claim="Claim two",
        confidence=ConfidenceScore(value=0.7),
    )
    
    report = ResearchReport(
        title="Evidence Report",
        summary="Contains evidence",
        evidence=[evidence1, evidence2],
        confidence_overall=ConfidenceScore(value=0.8),
    )
    assert len(report.evidence) == 2
    assert report.evidence[0].claim == "Claim one"


# Traces to: FR-TYPES-003
def test_research_report_json_export():
    """Test ResearchReport JSON export."""
    report = ResearchReport(
        title="JSON Test",
        summary="Testing JSON export",
        confidence_overall=ConfidenceScore(value=0.9),
    )
    json_data = report.model_dump_json()
    assert "JSON Test" in json_data
    assert "0.9" in json_data


# Traces to: FR-TYPES-003, FR-TYPES-006
def test_research_report_schema():
    """Test ResearchReport JSON schema generation."""
    schema = ResearchReport.model_json_schema()
    assert "title" in schema["properties"]
    assert "summary" in schema["properties"]
    assert "confidence_overall" in schema["properties"]
