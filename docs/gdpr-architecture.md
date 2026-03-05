# GDPR Architecture: Compile-Time Data Governance

## The Concept
Standard PII (Personally Identifiable Information) handling relies on runtime checks or post-hoc filtering. If the logic that redacts data is skipped or misconfigured, sensitive data leaks to an external LLM. 

This system moves GDPR enforcement into the **Rust type system**. We use types to make it structurally impossible to pass PII to an AI module.

## Type-Based Protection
1.  **`RawLead`:** Contains PII (Name, Phone, Email, Address). Only exists in the initial intake and the secure CRM sync modules.
2.  **`SanitizedContext`:** A separate struct that physically does not have fields for PII. It only contains the property inquiry and metadata.

### The Guardrail
The function that calls the Python AI bridge accepts **only** `SanitizedContext`. 

```rust
// This signature makes it impossible to accidentally pass PII.
pub fn call_ai_bridge(ctx: SanitizedContext) -> Result<Summary, Error> { ... }
```

If a developer attempts to pass a `RawLead` to `call_ai_bridge`, the code will not compile. Not a runtime error — a **structural rejection**.

## Classification of PII
We classify the following as PII in the real estate context:
*   **Direct Identifiers:** Name, Phone, Email.
*   **Indirect Identifiers:** Exact property address (which could be mapped back to an owner).

The `strip_pii()` module removes these locally on a dedicated EU-hosted Hetzner server before any data reaches the OpenRouter or Vertex AI endpoints.

## Infrastructure Sovereignty
*   **Hetzner DE:** All data processing occurs in Falkenstein (Germany).
*   **Supabase Self-Hosted:** No cloud-hosted database; everything runs within the sovereign EU environment.
*   **EU AI Act Compliance:** By automating deterministic compliance checks and isolating AI, the system adheres to transparency requirements without falling into the "high-risk" category (as defined by Annex III).
