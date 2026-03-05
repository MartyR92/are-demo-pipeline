# Security Policy

## Scope
This repository contains **architecture documentation and anonymized examples** of the ARE Audit Pipeline. No productive client code or private data is stored in this repository.

## Data Governance (GDPR / DSGVO)
The pipeline is designed with a **Privacy-First** architecture. PII (Personally Identifiable Information) is redacted locally within the EU sovereign trust boundary before any data reaches third-party AI endpoints. 

For more details on the compile-time GDPR enforcement, see [docs/gdpr-architecture.md](./docs/gdpr-architecture.md).

## Reporting Vulnerabilities
If you identify any security-relevant findings in our architectural approach or provided examples, please report them directly to:

**Email:** [martin.reiter@revivelapalma.com](mailto:martin.reiter@revivelapalma.com)

## Responsible Disclosure
*   Please allow a reasonable time for a response before making any information public.
*   No bug bounty program is currently in place for this demonstration repository.
