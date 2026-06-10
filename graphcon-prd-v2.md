# Product Requirements Document (PRD)

**Project Name:** GraphCon — Graph-based Agentic Recon & Attack Surface Analysis
**Version:** MVP 1.0 (Local-Only)
**Status:** Draft
**Deployment Target:** Single-user, localhost
**Target Persona:** Security Researcher, Bug Hunter

---

## 1. Objective & Value Proposition

**Problem:** Tools recon konvensional menghasilkan output teks linear (JSON/TXT) yang susah dianalisis. LLM kesulitan memahami attack surface dari ribuan baris log statis, dan recon-to-exploitation workflow masih manual dan terputus-putus.

**Solution:** GraphCon adalah **MCP Server** yang meng-expose security tooling (recon cepat berbasis Rust + active vuln testing) ke agent LLM manapun. Agent secara otonom melakukan recon, memetakan hasil ke Graph DB, dan menganalisis attack path lewat GraphRAG. Active/destructive testing dijaga oleh human-in-the-loop gatekeeper.

**Dua prinsip arsitektur utama:**
1. **GraphCon = MCP Server** — dapat dipanggil oleh agent manapun yang support MCP (Claude Code, Codex, Antigravity, Cursor, dll).
2. **LLM-agnostic** — backend LLM untuk GraphRAG dapat di-swap via `config.toml` (Claude, OpenAI, Gemini, Ollama).

---

## 2. Core Architecture

GraphCon berperan sebagai **MCP Server**. Agent LLM eksternal adalah **MCP Client / Orchestrator** yang otonom.

```
[External Agent / MCP Client]
   Claude Code | Codex | Antigravity | Cursor
                  |
                  |  (MCP protocol — stdio / SSE)
                  v
        ┌─────────────────────────────┐
        │   GraphCon MCP Server       │
        │   (Python — mcp SDK)        │
        │                             │
        │  - Tool registry            │
        │  - Scope validator          │
        │  - Active-scan gatekeeper   │
        │  - LLM-agnostic adapter     │
        └─────────────────────────────┘
           |          |           |
           v          v           v
   [Rust Engine]  [Vuln Tools]  [Neo4j DB]
   (recon cepat)  (sqli/idor)  (localhost:7687)
           |          |
           v          v
       [Target Network]
```

| Komponen | Stack | Transport / Port |
|---|---|---|
| MCP Server (GraphCon core) | Python (`mcp` SDK) | `stdio` atau SSE ke agent |
| Rust Engine | Tokio + Reqwest | subprocess, JSON via stdout |
| Vuln Test Modules | Python atau Rust | dipanggil internal oleh MCP Server |
| Graph DB | Neo4j (Docker) | `localhost:7687` (Bolt) |
| LLM Adapter | Python (provider-agnostic) | HTTP ke provider API |
| Frontend UI | SvelteKit / React + Cytoscape.js | `localhost:5173` |

**Catatan:** Rust Engine dan Vuln Tools **tidak** di-expose langsung sebagai MCP tool terpisah ke agent luar. Mereka internal. Yang di-expose adalah set tool GraphCon yang sudah dibungkus safety layer (lihat Section 4).

---

## 3. Agentic Flow

```
1. TRIGGER
   User → agent: "Recon target.com, petakan attack surface, cari celah OWASP"

2. ORCHESTRATION (LLM otonom)
   Agent baca tool registry GraphCon via MCP, susun rencana step-by-step.

3. PHASE 1 — FAST RECON (otonom, no confirmation)
   Agent panggil run_passive_recon → Rust Engine jalanin enum subdomain,
   port scan, deteksi exposed asset (/.env, /.git, A05/A06 indicators).
   Hasil JSON balik ke agent.

4. GATEKEEPER (human-in-the-loop)
   Agent baca hasil Phase 1, identifikasi target active testing
   (URL berparameter, endpoint login). Agent TIDAK boleh langsung eksekusi.
   Agent panggil request_active_scan → status PENDING_APPROVAL.
   User approve/reject via UI atau CLI.

5. PHASE 2 — SMART SCAN (otonom SETELAH approval)
   Setelah approved, agent panggil run_active_scan dengan token.
   Tool spesifik (test_sqli, test_idor) dieksekusi untuk A01/A03.
   Hasil JSON balik ke agent.

6. EVIDENCE BOARD (Graph push)
   Agent panggil save_to_graph. JSON → Cypher MERGE → Neo4j.
   Node saling terhubung: Domain → IP → Port → Service → Vuln.

7. VISUALIZATION
   Web UI query Neo4j, render node graph interaktif real-time.

8. GRAPHRAG LOOP
   User prompt lagi: "Tunjukin jalan terpendek ke RCE dari graf ini."
   Agent panggil analyze_attack_path → LLM query Neo4j (shortest path,
   centrality) → jawab dengan konteks spasial yang jelas.
```

**Prinsip otonomi:**
- Phase 1 (passive/recon) → **fully autonomous**, tidak perlu konfirmasi.
- Phase 2 (active/vuln testing) → **gated**, wajib approval user sebelum eksekusi.
- Agent boleh *merekomendasikan* active scan, tapi tidak boleh *mengeksekusi* tanpa token approval.

---

## 4. Security & Safety Layer (Mandatory)

### 4.1 Scope Whitelist
- User definisikan `scope.json` sebelum sesi.
  ```json
  {
    "authorized_domains": ["target.com", "*.target.com"],
    "authorized_ips": ["192.168.1.0/24"],
    "session_label": "bugbounty-target-q3"
  }
  ```
- Setiap tool call yang menyentuh target divalidasi terhadap whitelist.
- Target di luar scope → ditolak, di-log, tidak dieksekusi.

### 4.2 Active-Scan Gatekeeper (Kill Switch)
- Diimplementasi sebagai dua tool terpisah: `request_active_scan` dan `run_active_scan`.
- `run_active_scan` **wajib** menerima `approval_token` valid yang hanya di-generate setelah user approve `request_active_scan`.
- Token: single-use, expire 5 menit, terikat ke `job_id` spesifik.
- Tanpa mekanisme ini agent tidak bisa auto-escalate ke active testing.

### 4.3 Input Sanitization
- Output parsing LLM (domain, IP, path, payload) divalidasi layer Python sebelum diteruskan ke engine.
- Domain match RFC 1123, IP parseable, path reject `../` traversal.

### 4.4 Audit Log
- Append-only `logs/audit.jsonl`. Tiap entry: timestamp, phase, tool, target, `approved_by_user`, result summary.

### 4.5 Prompt Injection Guard
- Karena agent eksternal yang orchestrate, GraphCon **tidak mempercayai** parameter tool call secara buta.
- Semua parameter divalidasi server-side terhadap scope dan format, bukan diasumsikan benar karena datang dari agent.

---

## 5. MCP Tools (Public Interface)

Tool yang di-expose GraphCon ke agent eksternal:

| Tool | Input | Output | Gated? |
|---|---|---|---|
| `set_scope` | `domains[]`, `ips[]`, `session_label` | `scope_id` | No |
| `run_passive_recon` | `scope_id`, `domain` | `job_id`, recon JSON | No |
| `get_job_status` | `job_id` | `status`, `progress` | No |
| `request_active_scan` | `job_id`, `targets[]`, `test_types[]` | `approval_request_id`, status `PENDING` | No |
| `run_active_scan` | `approval_token`, `job_id` | vuln findings JSON | **Yes** |
| `save_to_graph` | `job_id`, findings JSON | `nodes_created`, `edges_created` | No |
| `query_graph` | `scope_id`, `cypher_query` | `nodes[]`, `edges[]` | No |
| `analyze_attack_path` | `scope_id`, `question` | `analysis_text`, `path[]` | No |

**Catatan desain:**
- `test_sqli` / `test_idor` **tidak** di-expose sebagai tool terpisah. Mereka adalah `test_types` di dalam `run_active_scan`, supaya semua active testing lewat satu gatekeeper.
- `query_graph` membatasi Cypher ke operasi read-only (reject `DELETE`, `CREATE`, `SET` dari agent).

---

## 6. LLM-Agnostic Adapter

LLM hanya dipakai internal untuk GraphRAG reasoning di `analyze_attack_path`. Orchestration dilakukan agent eksternal, jadi LLM internal ini opsional (bisa juga agent eksternal yang reasoning).

```toml
# config.toml
[llm]
provider = "claude"        # claude | openai | gemini | ollama
model = "claude-sonnet-4-20250514"
api_key_env = "ANTHROPIC_API_KEY"
base_url = ""              # untuk Ollama / proxy / OpenAI-compatible

[mcp]
transport = "stdio"        # stdio | sse
expose_active_tools = true

[engine]
rate_limit_rps = 50
request_delay_ms = 200
request_timeout_s = 10
```

Adapter pattern: satu interface `LLMProvider.complete(prompt)` dengan implementasi per-provider. Swap provider = ganti satu baris config.

---

## 7. Graph Data Schema (Neo4j)

| Node Label | Atribut Kunci | Relasi | Target |
|---|---|---|---|
| `ScanSession` | `session_label`, `timestamp`, `phase` | `SCANNED` | `Domain` |
| `Domain` | `name` | `HAS_SUBDOMAIN` | `Subdomain` |
| `Subdomain` | `name`, `is_wildcard`, `source` | `RESOLVES_TO` | `IPAddress` |
| `IPAddress` | `ip`, `asn`, `country` | `HAS_PORT` | `Port` |
| `Port` | `number`, `state`, `protocol` | `RUNS_SERVICE` | `Service` |
| `Service` | `name`, `version`, `http_status`, `server_header` | `EXPOSES` | `ExposedFile` / `Endpoint` |
| `ExposedFile` | `path`, `content_length`, `http_status` | — | Leaf |
| `Endpoint` | `url`, `method`, `has_params` | `HAS_VULN` | `Vulnerability` |
| `Vulnerability` | `type`, `owasp_category`, `cvss_score`, `cve_id`, `severity`, `confidence` | — | Leaf |

**Penting untuk GraphRAG / shortest-path-to-RCE:**
- `Endpoint` node baru — supaya parameterized URL bisa di-link ke vuln (jalur A01/A03).
- `Vulnerability.owasp_category` (enum A01–A10) + `severity` + `confidence` — supaya query "jalan terpendek ke RCE" bisa filter by category dan rank by severity.
- Neo4j shortest-path: `MATCH p=shortestPath((d:Domain)-[*]->(v:Vulnerability {type:'RCE'})) RETURN p`.

---

## 8. Technical Trade-offs & Decisions

| Isu | Decision |
|---|---|
| Otonomi vs safety | Hybrid: Phase 1 otonom, Phase 2 gated via approval token. |
| Active testing = beyond recon | PRD akui ini active vuln testing tool, bukan recon-only. Legal/ToS jadi tanggung jawab user via scope.json. |
| Neo4j vs Memgraph | Neo4j: ekosistem GraphRAG matang, Bloom untuk visualisasi, shortest-path native. Lebih berat tapi worth untuk attack-path analysis. |
| Agent eksternal tidak dipercaya | Semua parameter tool divalidasi server-side, bukan diasumsikan benar. |
| WAF trigger | Rate limit 50 rps + delay 200ms default. Proxy rotation → v2. |
| Browser crash (ribuan node) | Lazy expand depth 2, max 200 node awal. |
| Cypher injection via query_graph | Read-only enforcement, reject write operations dari agent. |

---

## 9. Local Setup

```
Prerequisites:
- Rust (stable)
- Python 3.11+
- Docker + Docker Compose
- Node.js 20+
- API key LLM (opsional, hanya jika pakai LLM internal untuk GraphRAG)
```

```yaml
# docker-compose.yml
services:
  neo4j:
    image: neo4j:5-community
    ports:
      - "7687:7687"   # Bolt
      - "7474:7474"   # Browser UI
    environment:
      - NEO4J_AUTH=neo4j/localpassword
    volumes:
      - neo4j_data:/data

volumes:
  neo4j_data:
```

---

## 10. Execution Roadmap

### Fase 1 — Rust Recon CLI (1–2 minggu)
- CLI: input domain → JSON (subdomain, port, exposed asset).
- Rate limiter, timeout, retry, partial-result handling dari awal.
- Test ke lab pribadi (HackTheBox / TryHackMe / VPS sendiri).

### Fase 2 — MCP Server + Safety Layer (1–2 minggu)
- Bungkus GraphCon jadi MCP Server (`stdio`), register tools Section 5.
- Implementasi scope validator + active-scan gatekeeper (approval token).
- Test: panggil tools dari Claude Code DAN Codex/Cursor — verifikasi multi-agent compat.

### Fase 3 — Neo4j Integration (1 minggu)
- Docker Neo4j up. `save_to_graph` consume JSON → Cypher MERGE.
- Verifikasi node + edge + shortest-path query jalan.

### Fase 4 — Vuln Test Modules (1–2 minggu)
- Implementasi `test_sqli`, `test_idor` di balik `run_active_scan`.
- Pastikan tergated approval token. Test di lab vuln (DVWA, Juice Shop).

### Fase 5 — Frontend + GraphRAG (1–2 minggu)
- Cytoscape.js UI, lazy expand, approval prompt untuk active scan.
- `analyze_attack_path` dengan LLM adapter + shortest-path Cypher.
- E2E test: NL prompt → recon → approval → active scan → graph → attack path query.

---

## 11. Out of Scope (MVP)

- Proxy rotation / anonymization (v2)
- Multi-user / auth layer
- Active exploitation beyond detection (PoC generation, shell, dll)
- CVE auto-enrichment dari feed eksternal
- Auto-report PDF generation
- CAPTCHA bypass
