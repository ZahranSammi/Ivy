# Software Requirements Specification (SRS)

**Project:** GraphCon — Graph-based Agentic Recon & Attack Surface Analysis
**Version:** 1.1 (MVP, Local-Only) — adds state persistence, process lifecycle, data contract
**Based on:** PRD v2.0
**Standard:** Adapted from IEEE 830-1998

---

## 1. Introduction

### 1.1 Purpose
Dokumen ini menetapkan kebutuhan fungsional dan non-fungsional GraphCon secara presisi dan dapat diverifikasi. Audiens: developer (implementasi), reviewer (validasi), dan diri sendiri sebagai acuan testing.

### 1.2 Scope
GraphCon adalah MCP Server lokal yang meng-expose tooling recon (Rust) dan active vulnerability testing (gated) ke agent LLM eksternal, menyimpan temuan ke Neo4j, dan menyediakan analisis attack path via GraphRAG. Sistem berjalan single-user di localhost.

**In scope:** passive recon, gated active scan (SQLi/IDOR detection), graph construction, graph query, attack-path analysis, web visualization.
**Out of scope:** multi-user, auth, proxy rotation, exploitation beyond detection, auto-report, CVE enrichment.

### 1.3 Definitions & Acronyms
| Istilah | Arti |
|---|---|
| MCP | Model Context Protocol |
| Agent | MCP Client eksternal (Claude Code, Codex, dll) yang meng-orchestrate |
| Engine | Rust subprocess untuk recon cepat |
| Gated tool | Tool yang butuh approval token sebelum eksekusi |
| Scope | Daftar target yang di-authorize user (scope.json) |
| GraphRAG | Retrieval-augmented generation di atas graph database |
| Finding | Satu temuan recon atau vuln |

### 1.4 References
- PRD GraphCon v2.0
- MCP Specification (modelcontextprotocol.io)
- OWASP Top 10:2021
- Neo4j Cypher Manual

---

## 2. Overall Description

### 2.1 Product Perspective
GraphCon adalah komponen server dalam ekosistem MCP. Ia tidak berdiri sendiri sebagai aplikasi end-user penuh — orchestration dilakukan agent eksternal. GraphCon menyediakan: (a) MCP tool interface, (b) Rust recon engine, (c) Neo4j persistence, (d) web UI read-only untuk visualisasi.

### 2.2 Product Functions (ringkasan)
- Definisi dan validasi scope.
- Passive recon otonom.
- Active scan dengan human approval gate.
- Konstruksi graph dari findings.
- Query graph read-only.
- Analisis attack path berbasis LLM.
- Visualisasi graph interaktif.

### 2.3 User Characteristics
Single user: security researcher / bug hunter dengan kompetensi teknis tinggi, paham CLI, Docker, dan konsep recon. Tidak ada role lain.

### 2.4 Constraints
- Berjalan hanya di localhost.
- Tidak ada auth layer (diasumsikan trusted local environment).
- LLM internal opsional; butuh API key jika diaktifkan.
- Active testing dibatasi detection, bukan exploitation.

### 2.5 Assumptions & Dependencies
- User punya otorisasi legal atas target di scope.json.
- Docker, Rust toolchain, Python 3.11+, Node.js 20+ tersedia.
- Agent eksternal support MCP (stdio atau SSE).

---

## 3. Functional Requirements

Format: setiap requirement punya ID, deskripsi, input, proses, output, dan acceptance criteria. Prioritas: **MUST** / **SHOULD** / **MAY**.

### 3.1 Scope Management

**FR-1 — Set Scope** (MUST)
- **Input:** `domains[]`, `ips[]`, `session_label` via tool `set_scope`.
- **Proses:** Validasi format tiap domain (RFC 1123) dan IP/CIDR (parseable). Persist ke memori sesi dan `scope.json`.
- **Output:** `scope_id` (UUID).
- **Acceptance:** Domain/IP invalid → tool return error dengan daftar entri yang gagal; tidak ada scope tersimpan. Scope valid → `scope_id` di-generate dan dapat dipakai tool lain.

**FR-2 — Scope Enforcement** (MUST)
- **Proses:** Setiap tool yang menyentuh target (`run_passive_recon`, `run_active_scan`) memvalidasi target terhadap scope aktif sebelum eksekusi.
- **Acceptance:** Target di luar scope → eksekusi ditolak, entry audit log dibuat dengan status `REJECTED_OUT_OF_SCOPE`, tidak ada network request keluar.

### 3.2 Passive Recon

**FR-3 — Run Passive Recon** (MUST)
- **Input:** `scope_id`, `domain`.
- **Proses:** Rust Engine jalankan: (a) subdomain enumeration (DNS brute-force + crt.sh), (b) DNS resolution, (c) port scan top 1000 TCP, (d) HTTP probe + deteksi exposed asset (`/.env`, `/.git`, dll).
- **Output:** `job_id`, dan saat selesai JSON terstruktur findings.
- **Acceptance:** Tanpa konfirmasi user (otonom). Hasil JSON memuat minimal: subdomains, IPs, open ports, http_status. Jika sebagian gagal → JSON memuat `"status": "partial"`.

**FR-4 — Async Job Status** (MUST)
- **Input:** `job_id` via `get_job_status`.
- **Output:** `status` (enum: `QUEUED`, `RUNNING`, `COMPLETED`, `PARTIAL`, `FAILED`, `INTERRUPTED`), `progress` (0–100).
- **Acceptance:** Polling tidak memblok. `job_id` invalid → error `JOB_NOT_FOUND`. Status dibaca dari state store persisten (lihat NFR-17), bukan dari memori proses.

**FR-4a — Job Reconciliation on Startup** (MUST)
- **Proses:** Saat GraphCon start, scan semua job di state store yang berstatus `QUEUED`/`RUNNING`. Karena proses Rust yang mengerjakannya sudah mati saat host sebelumnya berhenti, tandai ulang job tersebut menjadi `INTERRUPTED`.
- **Output:** Tidak ada (operasi internal startup).
- **Acceptance:** Setelah restart, tidak ada job yang tertinggal di status `RUNNING` tanpa proses pendukung. Klien yang query job lama mendapat status `INTERRUPTED`, bukan `RUNNING` palsu atau `JOB_NOT_FOUND`.

### 3.3 Active Scan (Gated)

**FR-5 — Request Active Scan** (MUST)
- **Input:** `job_id`, `targets[]`, `test_types[]` (enum: `sqli`, `idor`).
- **Proses:** Validasi targets ⊆ scope. Buat approval request dengan status `PENDING_APPROVAL`. Notifikasi ke UI/CLI.
- **Output:** `approval_request_id`.
- **Acceptance:** Tidak ada eksekusi scan pada tahap ini. Target di luar scope → request ditolak.

**FR-6 — User Approval** (MUST)
- **Input:** `approval_request_id`, keputusan user (approve/reject) via UI/CLI.
- **Proses:** Jika approve → generate `approval_token` (single-use, TTL 5 menit, terikat `job_id`). Token beserta status konsumsinya disimpan di state store persisten (NFR-17).
- **Output:** `approval_token` atau status `REJECTED`.
- **Acceptance:** Approval HANYA dari user (bukan dari agent/tool call). Token expired/reused → invalid. Status token bertahan melintasi restart server.

**FR-7 — Run Active Scan** (MUST, gated)
- **Input:** `approval_token`, `job_id`.
- **Proses:** Validasi token (valid, belum dipakai, belum expired, cocok dengan job_id) terhadap state store persisten. Jika valid → tandai token `CONSUMED` (atomik, sebelum eksekusi) → eksekusi test_types terhadap targets. SQLi: deteksi error-based/boolean-based. IDOR: deteksi akses objek tidak ter-otorisasi via manipulasi parameter.
- **Output:** findings JSON dengan field `type`, `owasp_category`, `severity`, `confidence`.
- **Acceptance:** Tanpa token valid → eksekusi ditolak dengan error `INVALID_OR_MISSING_APPROVAL`. Token tidak bisa dipakai dua kali, **termasuk setelah restart server** (status `CONSUMED` persisten). Penandaan `CONSUMED` terjadi sebelum eksekusi sehingga crash di tengah scan tidak membuka token untuk reuse.

### 3.4 Graph Operations

**FR-8 — Save to Graph** (MUST)
- **Input:** `job_id`, findings JSON.
- **Proses:** Translate JSON → Cypher `MERGE` (idempotent). Push ke Neo4j. Buat node `ScanSession` dan relasi.
- **Output:** `nodes_created`, `nodes_merged`, `edges_created`.
- **Acceptance:** Re-run job yang sama tidak menghasilkan node duplikat (idempotency). Schema sesuai PRD Section 7.

**FR-9 — Query Graph** (MUST)
- **Input:** `scope_id`, `cypher_query`.
- **Proses:** Parse query, tolak jika memuat operasi write (`CREATE`, `MERGE`, `DELETE`, `SET`, `REMOVE`). Eksekusi read-only.
- **Output:** `nodes[]`, `edges[]`.
- **Acceptance:** Query write dari agent → ditolak dengan `WRITE_NOT_ALLOWED`. Query read valid → return hasil.

**FR-10 — Analyze Attack Path** (SHOULD)
- **Input:** `scope_id`, `question` (natural language).
- **Proses:** LLM adapter generate Cypher (mis. `shortestPath` ke node `Vulnerability` severity tinggi), eksekusi read-only, LLM susun penjelasan.
- **Output:** `analysis_text`, `path[]` (urutan node).
- **Acceptance:** Untuk pertanyaan "jalan terpendek ke RCE", sistem return path valid jika ada node RCE di graph; jika tidak ada, return penjelasan bahwa tidak ditemukan.

### 3.5 Visualization

**FR-11 — Render Graph** (SHOULD)
- **Proses:** Frontend query Neo4j, render node+edge via Cytoscape.js. Default depth 2 (Domain→Subdomain→IP). Node lain expand on-click.
- **Acceptance:** Initial render maksimal 200 node. Node di luar batas → paginated/collapsed.

**FR-12 — Approval UI** (MUST)
- **Proses:** UI/CLI menampilkan pending active-scan request dengan detail target + test_types. User dapat approve/reject.
- **Acceptance:** Approval action di UI memicu FR-6.

---

## 4. Non-Functional Requirements

### 4.1 Performance
- **NFR-1:** Passive recon untuk 1 domain dengan ≤100 subdomain selesai ≤ 5 menit pada koneksi normal. (Rust engine, rate limit 50 rps)
- **NFR-2:** `query_graph` untuk graph ≤ 5000 node return ≤ 2 detik.
- **NFR-3:** UI initial render ≤ 200 node tanpa freeze (>30 fps interaksi).

### 4.2 Reliability
- **NFR-4:** Partial failure pada recon tidak meng-crash engine; hasil parsial tetap dikembalikan dengan flag.
- **NFR-5:** Rate-limit hit (429/503) → exponential backoff, max 3 retry, lalu skip + log. Tidak ada crash.
- **NFR-6:** `save_to_graph` idempotent — re-run tidak menduplikasi data.
- **NFR-17 — State Persistence (MUST):** Seluruh state kontrol — `job_id` + status, `approval_request_id`, `approval_token` + status konsumsi — disimpan di local lightweight DB (SQLite via modul `sqlite3` bawaan Python). State bertahan melintasi restart/crash server MCP. State store hanya melacak metadata kontrol, bukan hasil recon mentah (hasil tetap di Neo4j).
  - *Acceptance:* Restart server di tengah job aktif → state job & token tetap terbaca konsisten (lihat FR-4a, FR-7).
- **NFR-18 — Child Process Lifecycle / Orphan Prevention (MUST):** Python wajib men-spawn proses Rust dalam process group terpisah (mis. `start_new_session=True` / `os.setsid`). Saat GraphCon menerima `SIGTERM`/`SIGINT`, ia wajib mengirim sinyal terminasi ke seluruh process group sehingga engine Rust ikut mati. Tidak boleh ada engine Rust yang terus menembak target setelah host-nya berhenti.
  - *Acceptance:* Kill host Python saat fuzzer jalan → verifikasi (via `ps`) tidak ada proses Rust yang tertinggal.
- **NFR-19 — Engine-Side Watchdog (MUST):** Karena `SIGKILL` (-9) pada parent tidak bisa ditangkap dan meninggalkan child sebagai orphan, engine Rust wajib punya self-timeout/watchdog internal yang menghentikan dirinya setelah durasi maksimum (configurable, default mis. 30 menit) tanpa bergantung pada sinyal dari parent. Defense in depth bersama NFR-18.
  - *Acceptance:* Parent di-`SIGKILL` → engine Rust tetap berhenti sendiri dalam batas watchdog.

### 4.3 Security & Safety
- **NFR-7:** Tidak ada active scan tereksekusi tanpa approval token valid (zero bypass).
- **NFR-8:** Semua eksekusi tercatat di audit log append-only.
- **NFR-9:** Semua parameter tool call divalidasi server-side; parameter dari agent tidak dipercaya buta.
- **NFR-10:** Tidak ada target di luar scope yang menerima network traffic.
- **NFR-11:** API key LLM tidak pernah di-log atau di-commit; dibaca dari env var.

### 4.4 Portability / Compatibility
- **NFR-12:** GraphCon dapat dipanggil minimal 2 agent berbeda (Claude Code + satu lainnya) tanpa perubahan kode, hanya konfigurasi MCP client.
- **NFR-13:** LLM backend dapat di-swap via `config.toml` tanpa perubahan kode (Claude/OpenAI/Gemini/Ollama).

### 4.5 Maintainability
- **NFR-14:** LLM provider di-abstraksi via interface tunggal (`LLMProvider`).
- **NFR-15:** Tool baru dapat ditambah tanpa mengubah core gatekeeper logic.

### 4.6 Usability
- **NFR-16:** Pesan error tool informatif dan actionable (menyebut penyebab dan field yang salah).

---

## 5. External Interface Requirements

### 5.1 MCP Tool Interface
Transport: `stdio` (default) atau SSE. Tools sesuai PRD Section 5. Setiap tool mengembalikan structured content + `isError` flag sesuai spec MCP. Error codes: `OUT_OF_SCOPE`, `JOB_NOT_FOUND`, `INVALID_OR_MISSING_APPROVAL`, `WRITE_NOT_ALLOWED`, `VALIDATION_ERROR`.

### 5.2 Rust Engine Interface
GraphCon ↔ Engine via subprocess. Input: argumen CLI / JSON via stdin. Output: JSON via stdout, log via stderr. Exit code non-zero = failure.

### 5.3 Neo4j Interface
Bolt protocol `localhost:7687`. Auth via env var. Read query lewat `query_graph`, write lewat `save_to_graph`.

### 5.4 LLM Provider Interface
HTTP ke provider sesuai `config.toml`. Interface internal: `complete(system, prompt) -> text`. Timeout 30 detik, retry 2x.

### 5.5 Frontend Interface
Frontend ↔ Neo4j read-only via backend endpoint atau Neo4j driver. Polling untuk update real-time (interval ≤ 3 detik) atau WebSocket (MAY).

---

## 6. Data Requirements

### 6.1 Graph Schema
Sesuai PRD Section 7 (node: ScanSession, Domain, Subdomain, IPAddress, Port, Service, ExposedFile, Endpoint, Vulnerability).

### 6.2 Configuration Data
`config.toml` (llm, mcp, engine), `scope.json` (authorized targets), `.env` (secrets, gitignored).

### 6.3 Audit Data
`logs/audit.jsonl` — append-only, satu JSON object per baris.

### 6.4 Engine Data Contract (Single Source of Truth)
Boundary Rust→Python adalah pertemuan dua type system. "JSON" saja bukan kontrak. Struktur berikut adalah satu-satunya sumber kebenaran untuk payload yang dikeluarkan Rust Engine dan dikonsumsi GraphCon.

**Aturan kontrak:**
- Struct didefinisikan di Rust dengan `serde` (`#[derive(Serialize)]`) sebagai sumber. JSON Schema di-generate dari struct via `schemars`.
- Python memvalidasi inbound JSON dengan Pydantic model yang mirror struct yang sama (idealnya digenerate dari JSON Schema agar tidak drift).
- Tipe harus eksplisit dan konsisten — `http_status` adalah integer di kedua sisi, bukan string.
- Setiap payload memuat `schema_version`. Python menolak versi yang tidak kompatibel dengan error jelas (`SCHEMA_VERSION_MISMATCH`), bukan panic.

**Struktur `ReconResult` (referensi):**
```
ReconResult {
  schema_version: string        // mis. "1.0"
  job_id:         string (UUID)
  status:         enum { complete, partial, failed }
  scanned_at:     string (ISO-8601 UTC)
  findings: {
    subdomains: [ { name: string, source: enum, is_wildcard: bool } ],
    hosts: [ {
      ip:    string,
      asn:   integer | null,
      ports: [ { number: integer, state: enum, protocol: enum } ]
    } ],
    services: [ {
      host: string, port: integer,
      name: string | null, version: string | null,
      http_status: integer | null, server_header: string | null
    } ],
    exposed_files: [ { url: string, path: string, http_status: integer, content_length: integer } ]
  }
  errors: [ { stage: string, target: string, message: string } ]   // diisi saat status=partial/failed
}
```
- *Acceptance:* Payload yang melanggar tipe/skema ditolak di boundary dengan error tervalidasi, tidak pernah menyebabkan panic/crash di Python.

### 6.5 Control State Store (SQLite)
Skema minimal untuk state persisten (NFR-17):
- `jobs(job_id PK, scope_id, phase, status, progress, created_at, updated_at)`
- `approvals(approval_request_id PK, job_id FK, targets_json, test_types_json, status, created_at)`
- `tokens(token PK, job_id FK, approval_request_id FK, status[ACTIVE|CONSUMED|EXPIRED], issued_at, expires_at, consumed_at)`

File: `state/graphcon.db`. Penandaan token `CONSUMED` bersifat atomik dan dilakukan sebelum eksekusi scan (FR-7).

---

## 7. Acceptance & Verification Matrix

| Req | Metode Verifikasi |
|---|---|
| FR-1, FR-2 | Unit test scope validator + test target out-of-scope ditolak |
| FR-3, FR-4 | Integration test recon ke lab target, cek JSON & status |
| FR-5–FR-7 | Test gatekeeper: coba run_active_scan tanpa/with token expired/reused → semua ditolak |
| FR-8 | Test idempotency: run save 2x, assert no duplicate |
| FR-9 | Test query write ditolak, read diterima |
| FR-10 | Test shortest-path query di graph dengan node RCE |
| FR-11, FR-12 | Manual UI test render + approval flow |
| NFR-7 | Security test: enumerasi semua jalur ke active scan, pastikan tidak ada bypass |
| NFR-12 | Test panggil dari Claude Code + Codex |
| NFR-13 | Swap provider di config, jalankan analyze_attack_path |
| FR-4a, NFR-17 | Restart server di tengah job → assert job jadi INTERRUPTED, token state konsisten |
| NFR-18 | Kill host Python saat fuzzer jalan → `ps` assert tidak ada proses Rust orphan |
| NFR-19 | SIGKILL parent → assert engine Rust berhenti sendiri dalam batas watchdog |
| FR-7 (persist) | Restart server lalu coba reuse token CONSUMED → ditolak |
| 6.4 | Feed JSON dengan tipe salah / schema_version beda → assert ditolak tanpa panic |

---

## 8. Future Considerations (Post-MVP)
Proxy rotation, multi-user + auth, CVE enrichment, PoC generation, auto-report, screenshot capture, distributed scanning.
