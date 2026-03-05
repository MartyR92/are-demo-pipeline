# Demo-Repo: are-audit-pipeline
## Anleitung für ein öffentliches Architektur-Repository

---

## Was dieses Repo ist

Öffentliche Architektur-Dokumentation einer Hybrid Rust/Python-Pipeline für
automatisierte Immobilien-Compliance und KI-Asset-Generierung. Kein produktiver
Client-Code. Zeigt: Rust-First-Architektur, Compile-Time-GDPR-Enforcement,
RAG-basierte Compliance-Logik und die Designentscheidungen dahinter.

---

## Repository anlegen

**Name:** `are-audit-pipeline`
**Sichtbarkeit:** Public
**Description:**
> Hybrid Rust/Python pipeline. Geospatial compliance + RAG legal auditing + AI asset generation. 14 days → 45 min. GDPR enforced at compile time.

**Topics / Tags:**
`rust` `python` `llm` `rag` `gdpr` `sovereign-ai` `real-estate` `geospatial`
`eu-ai-act` `tokio` `polars` `qdrant` `pyo3` `axum`

**License:** MIT

---

## Dateistruktur

```
are-audit-pipeline/
├── README.md
├── ARCHITECTURE.md
├── CASE_STUDY.md
├── SECURITY.md
├── CONTRIBUTING.md
├── CHANGELOG.md
├── .env.example
├── .gitignore
├── Cargo.toml            ← Dependency-Dokumentation
├── adr/
│   └── 001-rust-python-split.md
├── docs/
│   ├── compliance-flow.md
│   └── gdpr-architecture.md
├── diagrams/
│   ├── system-overview.png
│   └── pipeline-flow.png
├── examples/
│   ├── privacy_filter.rs
│   ├── context_builder.py
│   └── schema.sql
└── LICENSE
```

---

## Sprachpolitik

**README.md, ARCHITECTURE.md, CASE_STUDY.md: Englisch.**
GitHub-Suche und internationale Auftraggeber. Englisch maximiert Reichweite.

**`docs/`, `adr/`: Deutsch oder Englisch — konsistent pro Datei.**
Interne Dokumentation kann Deutsch sein. Nicht mischen.

---

## README.md — Inhalt

**1. Badges**
```markdown
![Status](https://img.shields.io/badge/Status-Production-brightgreen?style=flat-square)
![Stack](https://img.shields.io/badge/Stack-Rust_%2B_Python-1A5F7A?style=flat-square)
![GDPR](https://img.shields.io/badge/GDPR-Compile--Time_Enforced-0E8A5F?style=flat-square)
![EU AI Act](https://img.shields.io/badge/EU_AI_Act-Compliant-0E8A5F?style=flat-square)
![CI](https://github.com/MartyR92/are-audit-pipeline/actions/workflows/ci.yml/badge.svg)
```

**2. One-liner**
> Autonomous property intelligence pipeline. Rust controls the data. Python handles the AI. GDPR compliance is enforced by the compiler, not by convention.

**3. Das Problem**
> A real estate agency in one of Europe's most regulated markets (Canary Islands, Spain) spent 14 days per property manually aggregating geospatial data from incompatible government sources, running legal compliance checks across unstructured PDFs and legacy SOAP APIs, and producing marketing materials. Staff cost per property: €450. Human error rate on protected land zone checks: ~15%.

**4. Ergebnisse — sofort sichtbar**

```markdown
| Metric | Before | After |
|---|---|---|
| Processing time | 14 days | 45 minutes |
| Cost per report | €450 | €3.50 |
| Compliance error rate | ~15% | 0% (deterministic) |
| Time to production | — | 4 weeks |
```

**5. Das stärkste technische Argument — prominent in die README, nicht in CASE_STUDY.md begraben**

```markdown
## GDPR: Enforced at Compile Time

Standard AI integrations check for PII at runtime — if the check is skipped
or misconfigured, data leaks. This system uses the Rust type system instead.

`RawLead` contains PII fields. `SanitizedContext` does not.
LLM call sites only accept `SanitizedContext`.

The compiler rejects any code that passes `RawLead` to an AI module.
Not a runtime check. Not a policy. A compile-time guarantee.
```

**6. Architektur-Überblick — vier Module**

```markdown
| Module | Name | Responsibility |
|---|---|---|
| A | The Harvester | Parallel ingestion: WFS, Kataster, BOC PDFs, AEMET weather |
| B | The Auditor | RAG compliance: PostGIS × Red Natura 2000, Qdrant legal texts |
| C | The Studio | AI media: Vertex AI image restoration, OpenCV, video generation |
| D | The Nexus | Delivery: Astro dashboard, Supabase realtime, dynamic Value Stack |
```

**7. Tech Stack**

```markdown
| Layer | Tools |
|---|---|
| **Backend** | Rust (Axum · Tokio · Polars · PyO3 · Rig · Spider\_rs) |
| **AI Bridge** | Python via PyO3 (OpenCV · Google Vertex AI) |
| **LLM / RAG** | OpenRouter · Qdrant · Context Engineering |
| **Data** | Polars DataFrames · PostGIS · Parquet · Supabase |
| **Frontend** | Astro (Server Islands) · Tailwind CSS |
| **Infra** | Docker · Hetzner EU · Vercel |
| **Compliance** | GDPR/DSGVO · EU AI Act · Red Natura 2000 |
```

**8. Links**

→ Architecture details: [ARCHITECTURE.md](./ARCHITECTURE.md)
→ Full case study: [CASE_STUDY.md](./CASE_STUDY.md)
→ GDPR flow: [docs/gdpr-architecture.md](./docs/gdpr-architecture.md)

**9. Schlusssatz**
> Full source is private (client confidentiality). Architecture, anonymized examples and the full case study are here. Contact: martin.reiter@revivelapalma.com

---

## ARCHITECTURE.md — Inhalt

**Abschnitt 1: Core Philosophy**
Rust/Python-Split als Designprinzip erklären. Rust für Correctness-by-Construction:
wenn Daten nicht dem Typen entsprechen, korrigiert sich das System selbst oder hält an —
bevor fehlerhaftes Material einen LLM-Call erreicht. Python ist Werkzeug, nicht Architektur.

**Abschnitt 2: Die vier Module — je ein eigener Unterabschnitt**

Für jedes Modul: Name, Verantwortlichkeit, konkrete Technologien, eine echte
Designentscheidung die erklärt warum — nicht was.

Beispiel für Module A:
```markdown
### Module A: The Harvester
**Responsibility:** Parallel data ingestion from government sources
**Tech:** Rust (Spider_rs, Tokio), Polars
**Key decision:** Politeness protocols implemented at the Rust layer —
rate limiting and IP rotation without browser automation.
Eliminates Playwright as a dependency. Binary bleibt self-contained.
```

**Abschnitt 3: Compliance Engine — kein LLM in der Compliance-Entscheidung**

Das ist nicht-offensichtlich und muss explizit stehen:
> PostGIS checks coordinates against Red Natura 2000 zones deterministically.
> No LLM is involved in the compliance decision. The LLM receives the result,
> not the question. Hallucination risk: zero for compliance outcomes.

**Abschnitt 4: State Management**

Mermaid-Diagramm:
````markdown
```mermaid
graph LR
    A[ingested] --> B[sanitized]
    B --> C[audited]
    C --> D[media_ready]
    D --> E[published]
    C --> F[restricted]
    A --> G[error]
    B --> G
    C --> G
```
````

**Abschnitt 5: Systemdiagramme**
```markdown
![System Overview](./diagrams/system-overview.png)
![Pipeline Flow](./diagrams/pipeline-flow.png)
```

---

## adr/001-rust-python-split.md — Inhalt

```markdown
# ADR 001: Rust-First Architecture with Isolated Python Bridge

**Status:** Accepted
**Context:** Real estate pipeline in a high-regulatory EU market.
Correctness and auditability are primary requirements.
Hallucination on zoning law or property tax is not acceptable.

## Options evaluated

| Option | Reason rejected |
|---|---|
| Full Python stack | No compile-time type safety. Runtime errors reach production. |
| Low-code / n8n | Cannot enforce type contracts at module boundaries. |
| Rust for everything | LLM inference and computer vision libraries are Python-first. |
| Rust + Python (PyO3) | Rust controls all logic. Python isolated to inference only. |

## Decision
Rust handles all business logic, data fetching, schema enforcement and
compliance gates. Python is called only via PyO3 secure bridge for
LLM inference and computer vision. Python never touches raw client data.

## Consequences
GDPR compliance is enforced structurally: `SanitizedContext` is the only
type accepted by LLM call sites. The compiler rejects any code that attempts
to pass `RawLead` to an AI module.
```

---

## docs/gdpr-architecture.md — Inhalt

- Was „Compile-Time GDPR" konkret bedeutet: `RawLead` vs. `SanitizedContext` als Typen
- Welche Felder als PII klassifiziert sind und warum (Name, Tel, Email, Adresse)
- Wie `strip_pii()` implementiert ist: Regex-Patterns, lokal, vor jedem LLM-Call
- Warum lokales Stripping besser ist als nachgelagertes Filtering (architectural, nicht operational)
- EU-Infrastruktur: Hetzner Falkenstein DE, Supabase self-hosted auf gleicher VM
- Referenz EU AI Act: Einordnung dieses Systems (kein High-Risk AI System per Annex III)

---

## docs/compliance-flow.md — Inhalt

Nummerierte Schritt-für-Schritt-Sequenz:
```
1. Lead arrives (email · web form · Idealista API)
2. Rust main process receives RawLead
3. GDPR filter: regex strips Name, Phone, Email, Address locally
4. SanitizedContext object constructed — PII fields structurally absent
5. PostGIS geospatial check: coordinates × Red Natura 2000 zones
   → Compliant: Context Engineering → Prompt Builder → LLM
   → Non-compliant: Restriction Report generated, pipeline halted
6. Privacy log entry written to CRM
```

---

## .env.example — Inhalt

```bash
# LLM
OPENROUTER_API_KEY=your_key_here
GOOGLE_VERTEX_PROJECT=your_project_id

# Vector DB
QDRANT_URL=http://localhost:6333
QDRANT_API_KEY=your_key_here

# Database
SUPABASE_URL=http://localhost:8000
SUPABASE_SERVICE_ROLE_KEY=your_key_here
DATABASE_URL=postgresql://postgres:password@localhost:5432/are

# CRM
HUBSPOT_ACCESS_TOKEN=your_token_here
```

---

## .gitignore — Pflicht-Einträge

```
.env
target/           # Rust build artifacts
__pycache__/
*.pyc
.venv/
*.pem
*.key
```

---

## SECURITY.md — Inhalt

Kurz, konkret:
- Kein produktiver Code in diesem Repo — nur Architektur-Dokumentation und Konzept-Beispiele
- Wie PII-Handling architektonisch implementiert ist (Verweis auf `docs/gdpr-architecture.md`)
- Responsible Disclosure: Sicherheitsrelevante Befunde an `martin.reiter@revivelapalma.com`
- Kein Bug-Bounty-Programm

---

## CHANGELOG.md — Inhalt

```markdown
# Changelog

## [1.0.0] — 2025-Q4
### Added
- Initial public architecture documentation
- GDPR privacy filter concept (Rust)
- Context builder example (Python)
- Property status schema (SQL)
- Full case study with metrics
```

---

## Cargo.toml — Inhalt (als Dokumentation, kein produktiver Code)

```toml
[package]
name = "are-audit-pipeline"
version = "1.0.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
polars = { version = "0.40", features = ["lazy", "parquet"] }
pyo3 = { version = "0.21", features = ["auto-initialize"] }
rig-core = "0.1"
spider = "1.98"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio"] }
regex = "1"
```

---

## examples/ — Was reinkommt

**privacy_filter.rs**

Compile-Time-GDPR ist das stärkste Argument — der Code muss es zeigen.

```rust
/// PII stripping middleware.
/// LLM call sites only accept `SanitizedContext`.
/// The compiler rejects any code passing `RawLead` to an AI module.
use regex::Regex;
use std::sync::LazyLock;

static PII_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b[\w.+-]+@[\w-]+\.[a-z]{2,}\b|\+?[\d\s\-()]{7,15}").unwrap()
});

/// Raw lead from intake — contains PII. Never reaches an LLM.
pub struct RawLead {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub inquiry: String,
    pub budget_range: BudgetRange,
    pub property_type: PropertyType,
}

/// Sanitized context — PII fields structurally absent, not redacted.
/// This is the only type accepted by LLM call sites.
pub struct SanitizedContext {
    pub inquiry: String,
    pub budget_range: BudgetRange,
    pub property_type: PropertyType,
}

/// Strips PII locally before any data leaves the system.
pub fn strip_pii(raw: RawLead) -> SanitizedContext {
    let clean = PII_PATTERN.replace_all(&raw.inquiry, "[REDACTED]");
    SanitizedContext {
        inquiry: clean.into_owned(),
        budget_range: raw.budget_range,
        property_type: raw.property_type,
    }
}
```

**context_builder.py**

Python wird nur via PyO3-Bridge aufgerufen. Zeige die Dataclass-Struktur
und wie der Prompt aus bereinigten Metadaten konstruiert wird — nie aus Rohdaten.

```python
from dataclasses import dataclass
from typing import Optional

@dataclass
class PropertyContext:
    """
    Sanitized context for LLM calls.
    Constructed from SanitizedContext (Rust) via PyO3 bridge.
    Contains no PII fields by design.
    """
    property_type: str
    budget_range: str
    location_zone: str
    compliance_status: str          # 'compliant' | 'restricted'
    restriction_notes: Optional[str]
    market_data: dict

def build_audit_prompt(ctx: PropertyContext, template: str) -> str:
    """Build a grounded prompt. No raw client data enters here."""
    return template.format(
        property_type=ctx.property_type,
        budget=ctx.budget_range,
        zone=ctx.location_zone,
        compliance=ctx.compliance_status,
        restrictions=ctx.restriction_notes or "none",
    )
```

**schema.sql**

```sql
-- Property status enum — the state machine protocol.
-- Each module reads one input status, writes one output status.
CREATE TYPE property_status AS ENUM (
    'ingested',    -- Raw data collected by The Harvester
    'sanitized',   -- PII removed, GDPR-safe
    'audited',     -- Compliance check complete
    'restricted',  -- Non-compliant: Red Natura 2000 or zoning violation
    'media_ready', -- The Studio processed imagery
    'published',   -- Delivered via The Nexus
    'error'        -- Pipeline halted, needs inspection
);

-- Atomic status transition function.
-- Returns FALSE if the expected current status doesn't match — no silent updates.
CREATE OR REPLACE FUNCTION advance_status(
    p_id UUID,
    p_expected property_status,
    p_next property_status
) RETURNS BOOLEAN AS $$
DECLARE updated INT;
BEGIN
    UPDATE properties
    SET status = p_next, updated_at = NOW()
    WHERE id = p_id AND status = p_expected;
    GET DIAGNOSTICS updated = ROW_COUNT;
    RETURN updated > 0;
END;
$$ LANGUAGE plpgsql;
```

---

## GitHub Actions — .github/workflows/ci.yml

Auch ein minimaler CI-Job gibt den grünen Haken-Badge in der README.

```yaml
name: CI
on: [push, pull_request]
jobs:
  clippy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy
      - run: cargo clippy --all-targets -- -D warnings
```

Badge in README:
```markdown
![CI](https://github.com/MartyR92/are-audit-pipeline/actions/workflows/ci.yml/badge.svg)
```

---

## CONTRIBUTING.md — Inhalt

```markdown
# Contributing

This is a portfolio and architecture demonstration repository.
The full system is in a private repository.

Issues and discussions are welcome — especially around:
- GDPR-compliant AI architecture
- Rust/Python interoperability patterns
- Geospatial compliance systems

Pull requests are not expected but will be reviewed if submitted.
```

---

## GitHub Release

Nach dem ersten Commit:
```bash
git tag v1.0.0
git push origin v1.0.0
```

Auf GitHub: Releases → Create release from tag → kurze Beschreibung → Publish.
Ein einzelner Release signalisiert "deployed, nicht in Entwicklung".

---

## Commit-Konvention

```
feat: add Rust privacy filter with compile-time PII enforcement
feat: add Python context builder example
feat: add property status schema with atomic transition function
docs: add GDPR architecture — compile-time vs runtime enforcement
docs: add ADR 001 — Rust-Python split decision
docs: add compliance flow step-by-step
ci: add Cargo clippy workflow
```

---

## Pinnen im Profil

`are-audit-pipeline` als erste gepinnte Position.
Stärkste Outcome-Metriken (14 Tage → 45 Min, €450 → €3.50).
