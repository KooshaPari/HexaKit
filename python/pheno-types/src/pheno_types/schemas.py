"""
JSON Schema export utilities.

Traces to: FR-TYPES-006
"""

from typing import Dict, Any, Type
from pydantic import BaseModel

# Registry of exportable types
_type_registry: Dict[str, Type[BaseModel]] = {}


def register_type(name: str, model_class: Type[BaseModel]) -> None:
    """Register a type for schema export.
    
    Args:
        name: The name to use for this type
        model_class: The Pydantic model class
    """
    _type_registry[name] = model_class


def export(type_name: str) -> Dict[str, Any]:
    """Export JSON Schema for a named type.
    
    Traces to: FR-TYPES-006
    
    Args:
        type_name: The name of the type to export
        
    Returns:
        JSON Schema as a dictionary
        
    Raises:
        KeyError: If type_name is not registered
    """
    if type_name not in _type_registry:
        raise KeyError(f"Type '{type_name}' not found in registry. "
                      f"Available types: {list(_type_registry.keys())}")
    
    model_class = _type_registry[type_name]
    return model_class.model_json_schema()


def export_all() -> Dict[str, Dict[str, Any]]:
    """Export JSON Schema for all registered types.
    
    Traces to: FR-TYPES-006
    
    Returns:
        Dictionary mapping type names to their JSON schemas
    """
    return {name: model_class.model_json_schema() 
            for name, model_class in _type_registry.items()}


def list_types() -> list[str]:
    """List all registered type names."""
    return list(_type_registry.keys())


# Register types on module import
# (This is done in __init__.py to avoid circular imports)
