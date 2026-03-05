from dataclasses import dataclass
from typing import Optional, Dict

@dataclass
class PropertyContext:
    """
    Sanitized context for LLM calls.
    Constructed from SanitizedContext (Rust) via PyO3 bridge.
    Contains NO PII (Personally Identifiable Information) fields by design.
    """
    property_type: str
    budget_range: str
    location_zone: str
    compliance_status: str          # 'compliant' | 'restricted'
    restriction_notes: Optional[str]
    market_data: Dict

def build_audit_prompt(ctx: PropertyContext, template: str) -> str:
    """
    Build a grounded prompt. 
    No raw client data enters here — only the deterministic audit results 
    from PostGIS and the sanitized inquiry metadata.
    """
    return template.format(
        property_type=ctx.property_type,
        budget=ctx.budget_range,
        zone=ctx.location_zone,
        compliance=ctx.compliance_status,
        restrictions=ctx.restriction_notes or "none",
        market_stats=ctx.market_data.get('avg_price_sqm', 'N/A')
    )

# --- Example of Isolated AI Bridge Usage ---

AUDIT_TEMPLATE = """
As a senior real estate advisor, summarize the following property audit:
- Property Type: {property_type}
- Market Zone: {zone}
- Budget Range: {budget}
- Compliance Status: {compliance}
- Restriction Notes: {restrictions}
- Current Zone Price (avg): {market_stats}

Provide a concise, professional assessment for a report.
"""

if __name__ == "__main__":
    # This data is received from the Rust backend (already sanitized)
    sanitized_from_rust = PropertyContext(
        property_type="Villa",
        budget_range="500k-700k",
        location_zone="Zone B (Coastal)",
        compliance_status="compliant",
        restriction_notes=None,
        market_data={"avg_price_sqm": "2400 €"}
    )
    
    prompt = build_audit_prompt(sanitized_from_rust, AUDIT_TEMPLATE)
    print("--- Grounded Prompt (Zero PII) ---")
    print(prompt)
