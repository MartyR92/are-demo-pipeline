# Compliance Pipeline: Step-by-Step Flow

The ARE Audit Pipeline follows a rigid, six-step process for every incoming property lead to ensure 100% compliance with GDPR and local real estate laws.

## 1. Intake
A new lead arrives via the website, email, or a real estate portal (e.g., Idealista). 
*   **Object Type:** `RawLead`
*   **Data Status:** `ingested`

## 2. GDPR Shield (Rust Layer)
Before any further processing, the `RawLead` is passed to the Rust privacy filter. 
*   **Action:** Regex-based local stripping of Name, Phone, Email, and exact Address.
*   **Result:** A `SanitizedContext` object is constructed. PII is now structurally absent.
*   **Data Status:** `sanitized`

## 3. Geospatial Audit (PostGIS Layer)
The system retrieves property coordinates from the Kataster WFS and runs a deterministic spatial query.
*   **Check:** Intersection of property bounds with *Red Natura 2000* or coastal protection zones.
*   **Decision Logic:** Pure SQL/PostGIS. No AI involved.
*   **Result:** `compliant` or `restricted`.
*   **Data Status:** `audited`

## 4. RAG Context Engineering
If compliant, the system retrieves relevant legal texts and market data from the Qdrant vector database.
*   **Prompt Builder:** Combines the audited compliance results with the `SanitizedContext`.
*   **Output:** A grounded prompt for the LLM.

## 5. AI Synthesis (Python Bridge)
The LLM (via the PyO3 bridge) receives the grounded prompt to generate a readable report.
*   **Note:** The AI never sees raw customer data; it only works with the audited, sanitized context.
*   **Action:** OpenCV/Vertex AI generate optimized media assets.
*   **Data Status:** `media_ready`

## 6. Nexus Delivery
The final report is published to the Astro dashboard and the CRM is updated.
*   **Action:** Privacy-safe logs are written to HubSpot.
*   **Data Status:** `published`
