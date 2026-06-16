from typing import TypedDict, List, Dict, Any, Annotated

class RFQState(TypedDict):
    messages: Annotated[List[Any], "Chat messages"]
    rfq_id: Annotated[str, "RFQ identifier"]
    customer_id: Annotated[str, "Customer identifier"]
    status: Annotated[str, "RFQ status"]
    items: Annotated[List[Dict[str, Any]], "Requested items"]
    quotes: Annotated[List[Dict[str, Any]], "Generated quotes"]
    selected_quote: Annotated[Dict[str, Any], "Selected quote"]

class OrderState(TypedDict):
    messages: Annotated[List[Any], "Chat messages"]
    order_id: Annotated[str, "Order identifier"]
    customer_id: Annotated[str, "Customer identifier"]
    status: Annotated[str, "Current order status"]
    items: Annotated[List[Dict[str,Any]], "Order items"]
    total: Annotated[float, "Order total amount"]

class ShippingState(TypedDict):
    messages: Annotated[List[Any], "Chat messages"]
    order_id: Annotated[str, "Order identifier"]
    status: Annotated[str, "Shipping status"]
    address: Annotated[Dict[str,Any], "Shipping address"]
    carrier: Annotated[str, "Shipping carrier"]
    tracking_number: Annotated[str, "Tracking number"]
    cost: Annotated[float, "Shipping cost"]
