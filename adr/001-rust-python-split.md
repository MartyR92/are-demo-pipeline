# ADR 001: Rust-First Architecture with Isolated Python Bridge

**Status:** Accepted  
**Date:** 2025-Q4  
**Context:** The real estate pipeline operates in a high-regulatory EU market (GDPR, EU AI Act, local zoning laws). Correctness, auditability, and data governance are primary requirements. Hallucination on zoning law or property tax is not acceptable.

## Problem Statement
How should we structure a system that requires high-performance data harvesting, deterministic geospatial analysis, and modern AI/LLM integration while maintaining strict compliance guarantees?

## Options Evaluated

| Option | Pros | Cons |
|---|---|---|
| **Full Python Stack** | Rapid development, vast AI library ecosystem. | No compile-time type safety. Runtime errors can reach production. High memory overhead for parallel harvesting. |
| **Low-code / n8n** | Visual workflows, fast to iterate. | Impossible to enforce strict type contracts at module boundaries. Scalability and version control challenges. |
| **Rust for Everything** | Maximum safety, high performance. | AI inference and computer vision libraries are overwhelmingly Python-first. Re-implementing them in Rust would be cost-prohibitive. |
| **Rust + Python (PyO3 Bridge)** | Rust controls all core logic. Python is isolated to inference. | Complexity of managing two environments and a bridge layer. |

## Decision
**We chose Option 4: Rust-First Architecture with an isolated Python Bridge via PyO3.**

Rust handles all critical business logic, data fetching, schema enforcement, and compliance gates. Python is invoked *only* for specialized tasks where it is the industry standard (e.g., LLM inference, OpenCV processing).

## Consequences
1.  **Structural Compliance:** GDPR compliance is enforced at the architecture level. `SanitizedContext` is the only type accepted by AI call sites. The compiler rejects any code that attempts to pass `RawLead` to an AI module.
2.  **Performance:** Parallel ingestion is handled efficiently by the Tokio runtime.
3.  **Audits:** Compliance logs are deterministic (PostGIS results), making the system fully auditable.
4.  **Isolation:** Python's large dependency footprint is restricted to the inference layer, keeping the core pipeline lean and stable.
