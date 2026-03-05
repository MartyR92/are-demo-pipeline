# Milestone 3: Renaming to are-demo-pipeline Requirements

## **Objective**
Systematically rename all occurrences of `are-demo-pipeline` to `are-demo-pipeline` across the repository to align with the new project name.

## **Functional Requirements**
### **1. Global Replacement**
- **FR.1:** Replace `are-demo-pipeline` with `are-demo-pipeline` in:
  - `README.md` (Title and Badges)
  - `Cargo.toml` (Package Name)
  - `CHANGELOG.md`
  - `1-UAT.md`
  - `demo_repo_are.md` (Instruction source)
  - `.planning/PROJECT.md`
  - `.planning/research/SUMMARY.md`

### **2. Link Integrity**
- **FR.2:** Ensure GitHub URLs in badges and documentation correctly point to the renamed repository (once pushed).

## **Non-Functional Requirements**
### **3. Consistency**
- **NFR.1:** The replacement must be surgical, affecting only the string "are-demo-pipeline" without altering surrounding context or formatting.

## **Success Criteria**
1.  `grep_search` for `are-demo-pipeline` returns zero results.
2.  All modified files maintain their original structural integrity.
