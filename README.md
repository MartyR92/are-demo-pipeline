# are-demo-pipeline
## Public Architecture Documentation

![Status](https://img.shields.io/badge/Status-Production-brightgreen?style=flat-square)
![Stack](https://img.shields.io/badge/Stack-Rust_%2B_Python-1A5F7A?style=flat-square)
![GDPR](https://img.shields.io/badge/GDPR-Compile--Time_Enforced-0E8A5F?style=flat-square)
![EU AI Act](https://img.shields.io/badge/EU_AI_Act-Compliant-0E8A5F?style=flat-square)
![CI](https://github.com/MartyR92/are-demo-pipeline/actions/workflows/ci.yml/badge.svg)

> Autonomous property intelligence pipeline. Rust controls the data. Python handles the AI. GDPR compliance is enforced by the compiler, not by convention.

---

## The Problem
A real estate agency in one of Europe's most regulated markets (Canary Islands, Spain) spent **14 days** per property manually aggregating geospatial data from incompatible government sources, running legal compliance checks across unstructured PDFs and legacy SOAP APIs, and producing marketing materials. 
*   **Staff cost per property:** €450
*   **Human error rate on land zoning checks:** ~15%

---

## Results — Real Estate Automation at Scale

| Metric | Before | After |
|---|---|---|
| **Processing time** | 14 days | **45 minutes** |
| **Cost per report** | €450 | **€3.50** |
| **Compliance error rate** | ~15% | **0% (deterministic)** |
| **Time to production** | — | 4 weeks |

---

## Technical Edge: GDPR Enforced at Compile Time

Standard AI integrations check for PII at runtime — if the check is skipped or misconfigured, data leaks. This system uses the Rust type system to make it structurally impossible to pass PII to an AI module.

*   `RawLead` contains PII fields (Name, Phone, Email, Address).
*   `SanitizedContext` does not.
*   The LLM/Python Bridge **only** accepts `SanitizedContext`.

The compiler rejects any code that attempts to bypass the privacy filter. Not a runtime check. Not a policy. A **compile-time guarantee**.

---

## Architecture Overview — Four Modules

| Module | Name | Responsibility |
|---|---|---|
| **A** | **The Harvester** | Parallel ingestion: WFS, Kataster, BOC PDFs, AEMET weather |
| **B** | **The Auditor** | RAG compliance: PostGIS × Red Natura 2000, Qdrant legal texts |
| **C** | **The Studio** | AI media: Vertex AI image restoration, OpenCV, video generation |
| **D** | **The Nexus** | Delivery: Astro dashboard, Supabase realtime, dynamic Value Stack |

---

## Tech Stack

| Layer | Tools |
|---|---|
| **Backend** | Rust (Axum · Tokio · Polars · PyO3 · Rig · Spider_rs) |
| **AI Bridge** | Python via PyO3 (OpenCV · Google Vertex AI) |
| **LLM / RAG** | OpenRouter · Qdrant · Context Engineering |
| **Data** | Polars DataFrames · PostGIS · Parquet · Supabase |
| **Frontend** | Astro (Server Islands) · Tailwind CSS |
| **Infra** | Docker · Hetzner EU · Vercel |
| **Compliance** | GDPR/DSGVO · EU AI Act · Red Natura 2000 |

---

## Documentation Links
→ **Architecture Details:** [ARCHITECTURE.md](./ARCHITECTURE.md)  
→ **Full Case Study:** [CASE_STUDY.md](./CASE_STUDY.md)  
→ **GDPR Flow:** [docs/gdpr-architecture.md](./docs/gdpr-architecture.md)

---

## Note on Source Code
Full source is private (client confidentiality). Architecture documentation, anonymized code examples, and the full case study are provided here for technical validation.

**Contact:** [martin.reiter@revivelapalma.com](mailto:martin.reiter@revivelapalma.com)
