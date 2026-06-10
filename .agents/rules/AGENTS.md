# AGENTS.md — GraphCon
# Cross-tool agent instructions (Antigravity, Claude Code, Codex, Cursor, dll)
# Location: project root
# Standard: AGENTS.md v1.20.3+

---

## Project Overview

GraphCon adalah MCP Server untuk agentic security recon dan attack surface analysis.

- **Rust engine** — fast async recon (subdomain enum, port scan, HTTP probe)
- **Python MCP layer** — tool orchestration, scope validation, gatekeeper
- **Neo4j** — graph storage & attack path analysis
- **Frontend** — Cytoscape.js visualization

SRS: `docs/graphcon-srs.md` | PRD: `docs/graphcon-prd-v2.md`

---

## Critical Safety Rules (Semua Agent Wajib Ikuti)

1. **Gatekeeper tidak boleh disentuh** — Jangan modifikasi logic approval token
   (FR-6/FR-7 di SRS) tanpa instruksi eksplisit. Ini zero-bypass requirement.

2. **Scope validation wajib ada** — Setiap fungsi yang mengirim network request
   ke target harus memanggil scope validator terlebih dahulu.

3. **No secrets in code** — Semua credential dari `.env`. File `.env` di-gitignore.

4. **State ke SQLite, bukan RAM** — job_id, token, approval state harus persisten.

5. **Process group isolation** — Rust subprocess harus di-spawn dengan session baru.
   Python harus handle SIGTERM/SIGINT untuk kill seluruh process group.

---

## Tech Stack & Conventions

### Rust (engine/)
- Toolchain: stable
- Async: `tokio`
- HTTP: `reqwest`
- Serialization: `serde` + `schemars` (wajib untuk semua output struct)
- Error: `thiserror` (lib), `anyhow` (binary)
- Tidak ada `unwrap()`/`expect()` di production path

### Python (mcp_server/)
- Version: 3.11+
- MCP: `mcp` SDK resmi
- Validation: `pydantic` v2
- DB: `sqlite3` (stdlib)
- Neo4j: `neo4j` driver resmi

### Konfigurasi
- `config.toml` — runtime config (rate limit, timeout, LLM provider, transport)
- `scope.json` — authorized targets (dibuat user)
- `.env` — secrets (gitignored)

---

## Data Contract

Payload Rust → Python menggunakan `ReconResult`. Field kritis:

| Field | Type | Note |
|---|---|---|
| `schema_version` | string | Wajib. Bump saat ada breaking change |
| `status` | enum string | `complete` / `partial` / `failed` |
| `http_status` | integer \| null | Bukan string |
| `port.number` | integer | Bukan string |
| `asn` | integer \| null | |

Python **harus** reject payload dengan `schema_version` tidak cocok.

---

## What NOT to Build (Unless Explicitly Asked)

- Bypass atau shortcut di active scan gatekeeper
- Proxy rotation
- Multi-user / auth
- Active exploitation (post-detection)
- Auto-report generation
- Perubahan MCP tool interface tanpa konfirmasi

---

## Cara Kerja Saat Ada Ambiguitas

- Safety > Speed
- Tanya satu pertanyaan paling penting, jangan asumsi
- Flag potential bug sebagai comment, jangan silent-fix di luar scope task
