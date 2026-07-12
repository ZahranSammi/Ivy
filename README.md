# 🌿 Ivy — Information Verification & Yield

> AI-powered OSINT platform that crawls like ivy — slowly, quietly, reaching every crack.

[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Backend-Rust-orange.svg)](https://www.rust-lang.org/)
[![Next.js](https://img.shields.io/badge/Frontend-Next.js-black.svg)](https://nextjs.org/)

---

## What is Ivy?

**Ivy** is an open-source OSINT (Open Source Intelligence) platform that uses AI to orchestrate security reconnaissance tools through the **Model Context Protocol (MCP)**. Results are stored and visualized as an interactive **graph database**, giving you a complete picture of your target's attack surface.

### Key Features

- 🤖 **AI-Orchestrated Recon** — AI plans and executes the optimal tool chain
- 📊 **Graph Visualization** — Interactive graph showing relationships between entities
- 🔌 **MCP Integration** — Standardized protocol for AI ↔ Tool communication
- 🧩 **Plugin System** — Exploit module shipped as separate, opt-in plugin
- 🔒 **Sandboxed Execution** — All tools run in isolated Docker containers
- 🌐 **Provider-Agnostic** — Works with Gemini, Claude, OpenAI, Ollama, and more

---

## Quick Start

### Prerequisites

- [Docker Engine](https://docs.docker.com/engine/install/) 24+
- [Docker Compose](https://docs.docker.com/compose/install/) v2+
- An LLM API key (Gemini, Claude, OpenAI, or local Ollama)

### Installation

```bash
git clone https://github.com/ivy-osint/ivy.git
cd ivy
cp .env.example .env          # Configure your LLM API key
docker compose up -d           # Start all services
```

Open **http://localhost:3000** in your browser.

### Development Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js (v20+)
# https://nodejs.org/

# Start development
make dev
```

---

## Architecture

```
┌─────────────┐    ┌──────────────┐    ┌──────────────────┐
│  Frontend    │◄──►│  Backend     │◄──►│  Graph Database  │
│  (Next.js)   │    │  (Rust/Axum) │    │  (Neo4j)         │
└─────────────┘    └──────┬───────┘    └──────────────────┘
                          │
                   ┌──────┴───────┐
                   │  MCP Layer   │
                   │  (AI + Tools)│
                   └──────┬───────┘
                          │
             ┌────────────┼────────────┐
             │            │            │
        ┌────┴───┐  ┌─────┴────┐  ┌────┴────┐
        │Passive │  │ Active   │  │ Exploit │
        │Tools   │  │ Tools    │  │ Plugin  │
        └────────┘  └──────────┘  └─────────┘
```

---

## Tech Stack

| Component | Technology |
|-----------|-----------|
| Backend | Rust (Axum) |
| Frontend | Next.js + TypeScript |
| Graph DB | Neo4j (Community Edition) |
| AI/LLM | Provider-Agnostic (Gemini, Claude, Ollama) |
| Protocol | Model Context Protocol (MCP) |
| Cache | Redis |
| App DB | PostgreSQL |
| Containers | Docker |

---

## Documentation

- [Product Requirements (PRD)](docs/Ivy_prd.md)
- [Software Requirements (SRS)](docs/Ivy_srs.md)
- [Project Structure](docs/Ivy_project_structure.md)

---

## ⚠️ Legal Disclaimer

Ivy is designed for **authorized security testing ONLY**. You **MUST** have explicit written permission from the target owner before running any scans or exploits. Unauthorized access to computer systems is illegal.

---

## Contributing

See [CONTRIBUTING.md](docs/contributing.md) for guidelines.

## License

MIT License — see [LICENSE](LICENSE) for details.
