# ShieldMedical-AI 🛡️ 
### A High-Performance Rust-Driven Guardrail Pipeline Against Prompt Injection in Medical LLMs

An open-source proof-of-concept demonstrating a hybrid infrastructure approach to LLM Safety. This architecture leverages **Rust** as an immutable, lightning-fast hardware-level firewall (First-Line Defense) to sanitize user prompts before they reach the **Python**-driven LLM application layer, effectively mitigating OWASP Top 10 LLM risks (Prompt Injection & Jailbreaking).

---

## 🔬 Core Architecture & Research Value

In highly regulated environments like healthcare, relying solely on Python-level soft-guardrails for LLMs introduces latency and security vulnerabilities (e.g., memory corruption exploits on the validation tools themselves). 

This project implements a **Dual-Layer Defense**:
1. **The Ingestion Layer (Rust):** A compiled, memory-safe dynamic library built with `PyO3` that parses input text at native speed, scanning for deterministic jailbreak signatures.
2. **The Processing Layer (Python):** Orchestrates the data pipeline, simulating LLM reasoning and routing only sanitized data to secure data sinks.

---

## ⚡ Key Features & Security Mitigations

* **OWASP LLM01 Mitigation:** Prevents prompt injections from overriding system instructions.
* **Deterministic Guardrails:** Bypasses LLM latency by filtering toxic prompts via a high-performance compiled engine before hitting the LLM API.
* **Memory Safety:** Eliminates classic buffer overflow vulnerabilities common in C/C++ native extensions.

---

## 🛠️ Project Structure

```text
shieldmedical_ai/
├── medical_guard/       # Rust Core Module
│   ├── Cargo.toml       # Rust Manifest
│   └── src/
│       └── lib.rs       # PyO3 Binding & Guardrail Engine
├── pipeline.py          # Python End-to-End Ingestion Simulation
└── README.md            # Technical Documentation





