# Milestone 2: Quality Audit & Deployment Requirements

## **Objective**
Audit the initial implementation for consistency with `demo_repo_are.md`, fix all identified issues (missing assets, spec deviations), and prepare the repository for public deployment on GitHub.

## **Functional Requirements**
### **1. Asset Completeness**
- **FR.1:** The `diagrams/` folder must contain both `system-overview.png` and `pipeline-flow.png`.
- **FR.2:** `ARCHITECTURE.md` must link to and display both `system-overview.png` and `pipeline-flow.png`.

### **2. Consistency & Formatting**
- **FR.3:** Image extensions in the `diagrams/` folder and all markdown files must be `.png` (per spec) or consistently updated to `.jpg` if the original asset is preferred.
- **FR.4:** All badges in `README.md` (CI, Status, Stack, etc.) must correctly point to the final repository target.

### **3. Code Verification**
- **FR.5:** Rust examples (`privacy_filter.rs`) must pass `clippy` linting.
- **FR.6:** Python examples (`context_builder.py`) must pass basic syntax checks.
- **FR.7:** SQL schema (`schema.sql`) must be valid PostgreSQL.

## **Non-Functional Requirements**
### **4. Public Presence**
- **NFR.1:** The repository must be initialized, tagged `v1.0.0`, and pushed to the public remote.
- **NFR.2:** All English-first documentation requirements must be strictly maintained.

## **Success Criteria**
1.  All identified gaps in `SUMMARY.md` are closed.
2.  `cargo clippy` passes on the example code.
3.  The repository is public on GitHub with a "production" status badge.
