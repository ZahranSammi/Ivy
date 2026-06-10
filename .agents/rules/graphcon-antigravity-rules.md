# GraphCon — Antigravity Agent Rules
# Location: .agents/rules/graphcon.md
# Scope: Workspace-specific (GraphCon project only)
# Limit: 12,000 chars max

---

## 1. Project Identity

GraphCon adalah MCP Server untuk agentic security recon dan attack surface analysis.
Stack: Rust (recon engine) + Python (MCP layer) + Neo4j (graph DB) + SvelteKit/React (UI).
Kamu adalah coding agent yang membantu implement dan debug sistem ini.

---

## 2. Hard Constraints (Non-Negotiable)

### 2.1 Security Boundaries — NEVER violate

- JANGAN pernah generate code yang menghilangkan, mem-bypass, atau melemahkan gatekeeper
  approval token di FR-6/FR-7. Active scan HARUS melalui token validation.
- JANGAN pernah generate code yang melakukan network request ke target tanpa validasi
  scope whitelist terlebih dahulu (`scope.json`).
- JANGAN embed API key, credential, atau secret apapun di dalam source code.
  Semua secret dibaca dari environment variable via `.env` (gitignored).
- JANGAN generate code yang menulis langsung ke Neo4j bypass `save_to_graph` —
  semua write harus lewat satu pintu yang terkontrol.
- JANGAN expose Rust Engine atau Vuln Tool modules langsung sebagai MCP endpoint.
  Yang di-expose hanya GraphCon MCP tool interface (lihat SRS Section 5).

### 2.2 Process Management

- Semua subprocess Rust HARUS di-spawn dengan `start_new_session=True`
  (atau `os.setsid()`) sehingga masuk process group terpisah.
- Python HARUS register signal handler untuk `SIGTERM` dan `SIGINT` yang mengirim
  `SIGTERM` ke seluruh process group sebelum exit.
- Rust engine HARUS punya internal watchdog/timeout (configurable, default 30 menit)
  sebagai defense terhadap parent yang kena SIGKILL.

### 2.3 State Management

- State kontrol (`job_id`, `approval_token`, status) HARUS disimpan ke SQLite
  (`state/graphcon.db`), bukan hanya in-memory.
- Token `CONSUMED` HARUS ditandai atomik di SQLite SEBELUM eksekusi scan dimulai.
- Saat startup, semua job berstatus `RUNNING` di SQLite HARUS di-reconcile
  menjadi `INTERRUPTED`.

---

## 3. Coding Standards

### 3.1 Rust

- Gunakan `tokio` async runtime. Jangan pakai `std::process::Command` untuk subprocess
  blocking — pakai `tokio::process::Command`.
- Semua struct yang jadi output ke Python HARUS derive `serde::Serialize`
  dan `schemars::JsonSchema`.
- Payload output HARUS include field `schema_version: String`.
- Error handling: pakai `thiserror` untuk library error, `anyhow` untuk binary/main.
- Jangan pakai `unwrap()` atau `expect()` di production path.
  Gunakan `?` propagation atau explicit match.
- Rate limiter dan timeout HARUS dikonfigurasi dari `config.toml`, bukan hardcoded.

### 3.2 Python (MCP Layer)

- Semua inbound JSON dari Rust Engine HARUS divalidasi via Pydantic model
  sebelum diproses lebih lanjut.
- Jika `schema_version` tidak cocok → raise `SchemaMismatchError`, jangan panic diam-diam.
- Gunakan `mcp` SDK resmi untuk tool registration. Jangan buat transport layer sendiri.
- Semua tool call parameters dari agent eksternal dianggap UNTRUSTED —
  validasi server-side sebelum diteruskan ke engine.
- Cypher query dari `query_graph` HARUS di-parse untuk reject operasi write
  (`CREATE`, `MERGE`, `DELETE`, `SET`, `REMOVE`) sebelum dieksekusi.

### 3.3 General

- Jangan over-engineer untuk v1. Ikuti roadmap fase: CLI dulu, MCP belakangan.
- Setiap fungsi yang menyentuh target network HARUS ada unit test dengan mock.
- Audit log (`logs/audit.jsonl`) HARUS diisi di setiap eksekusi tool yang
  menyentuh target (append-only, JSON per baris).

---

## 4. File & Directory Structure

```
graphcon/
├── engine/          # Rust recon engine (cargo workspace)
├── mcp_server/      # Python MCP layer
│   ├── tools/       # Satu file per MCP tool
│   ├── validators/  # Scope validator, input sanitizer, Cypher filter
│   ├── state/       # SQLite state store
│   └── llm/         # LLM adapter (provider-agnostic)
├── vuln_modules/    # Active scan modules (gated)
├── frontend/        # SvelteKit atau React + Cytoscape.js
├── docker-compose.yml
├── config.toml      # Konfigurasi runtime (non-secret)
├── scope.json       # Authorized targets (dibuat user sebelum sesi)
├── .env             # Secrets — GITIGNORED
└── .agents/
    └── rules/
        └── graphcon.md  # File ini
```

---

## 5. Data Contract (Single Source of Truth)

Rust Engine → Python menggunakan struct `ReconResult`. Tipe HARUS konsisten:
- `http_status`: `integer` (bukan string)
- `port.number`: `integer`
- `asn`: `integer | null`
- `status`: enum string `["complete", "partial", "failed"]`
- `schema_version`: `string` (mis. `"1.0"`)

Jangan pernah ubah tipe field tanpa update `schema_version` dan Pydantic model.

---

## 6. What You SHOULD Do When Stuck

- Kalau ada ambiguitas antara speed dan safety → pilih safety.
- Kalau ada dua cara implement dan salah satunya bypass safety layer → pilih yang tidak.
- Kalau requirements tidak jelas → tanya satu pertanyaan paling kritis, jangan asumsi.
- Kalau kamu temukan potensi bug/gap di luar task yang diminta → flag sebagai comment,
  jangan diam-diam "fix" sesuatu yang tidak diminta.

---

## 7. Out of Scope — Jangan Implement Tanpa Instruksi Eksplisit

- Proxy rotation / anonymization
- Multi-user / auth layer
- Active exploitation (shell, PoC generation, dll)
- CVE enrichment dari feed eksternal
- Auto-report PDF
- CAPTCHA bypass
- Modifikasi apapun pada gatekeeper logic tanpa konfirmasi eksplisit dari user
