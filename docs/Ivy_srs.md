# 🌿 Ivy — Software Requirements Specification (SRS)

> **Ivy** — *Information Verification & Yield*
> SRS v2.0 — simplified to MVP scope (supersedes v1.0)

---

## 1. Pendahuluan

### 1.1 Tujuan Dokumen

Dokumen ini mendefinisikan kebutuhan teknis untuk **MVP** Ivy: satu loop utuh yang bisa
dibuktikan jalan sebelum menambah fitur lain. Requirement di luar loop ini sengaja
dipisah ke [Backlog](#8-backlog--deferred-v11) — bukan dihapus, tapi belum dikerjakan.

### 1.2 Ruang Lingkup Produk

Ivy adalah **OSINT recon tool open-source, self-hosted, single-user** (dijalankan
lokal oleh satu operator — tidak ada login/multi-tenant di v1):
- Menerima target domain + scope dari user
- AI (via MCP) memilih & menjalankan security tools — passive dulu, active setelah
  user konfirmasi
- Hasil dikorelasikan ke graph database (Neo4j)
- Exploit tools tetap sebagai plugin terpisah, opt-in (tidak berubah dari sebelumnya)

### 1.3 Definisi & Akronim

| Istilah | Definisi |
|---------|----------|
| **OSINT** | Open Source Intelligence — pengumpulan informasi dari sumber publik |
| **MCP** | Model Context Protocol — protokol standar untuk komunikasi AI ↔ Tools |
| **LLM** | Large Language Model — model AI yang digunakan sebagai orchestrator |
| **Passive Recon** | Pengumpulan informasi tanpa berinteraksi langsung dengan target |
| **Active Recon** | Pengumpulan informasi dengan berinteraksi langsung dengan target |
| **Payload** | Kode/data yang dikirim untuk mengeksploitasi kerentanan (di luar scope v1) |
| **Consent Gate** | Konfirmasi eksplisit user sebelum tool active/payload-capable dijalankan |

### 1.4 Referensi

| Dokumen | Lokasi |
|---------|--------|
| Ivy PRD | [Ivy_prd.md](./Ivy_prd.md) *(belum disederhanakan — lihat catatan di §9)* |
| Project rules (canonical) | [.agents/rules/overview.md](../.agents/rules/overview.md) |
| MCP Specification | https://modelcontextprotocol.io/specification |
| Neo4j Documentation | https://neo4j.com/docs/ |

### 1.5 Keputusan Arsitektur

| Keputusan | Pilihan |
|-----------|---------|
| Backend | Rust (Axum) |
| Frontend | Next.js + TypeScript |
| Graph DB | Neo4j (Community Edition) |
| App DB | PostgreSQL |
| LLM | Provider-agnostic via MCP (Gemini, Claude, Ollama, dll.) |
| Auth | Tidak ada di v1 — single-user, self-hosted lokal |
| Tool Execution | Host subprocess langsung (no sandbox) di v1 — Docker sandbox di-defer, lihat §8 |
| Deployment | Native — Neo4j & Postgres diinstall langsung di host, tanpa Docker sama sekali di v1 |
| Exploit Module | Plugin terpisah (`plugins/ivy-exploit`), opt-in, double-consent |

---

## 2. MVP Loop

```
user input (target + scope)
        │
        ▼
  MCP connect ke LLM provider
        │
        ▼
  AI pilih & jalanin passive recon tools  ──► Neo4j (nodes/edges)
        │
        ▼
  user konfirmasi: lanjut active recon? ──► tidak ──► selesai, lihat graph
        │ ya
        ▼
  AI pilih & jalanin active recon tools  ──► Neo4j (nodes/edges)
        │
        ▼
  user lihat hasil di graph view
```

Di v1, semua tool (passive maupun active) jalan sebagai subprocess langsung di host (bukan
Docker container) — lihat FR-003.

### 2.1 Batasan & Asumsi

- Tool recon (whois, amass, nmap, dst) harus sudah ke-install manual di host — tidak lagi
  dibungkus Docker image per tool di v1 (lihat FR-003).
- Tidak pakai Docker sama sekali di v1 — Neo4j dan PostgreSQL diinstall & dijalankan native
  di host, bukan lewat container.
- Butuh internet untuk passive recon tools dan LLM API (kecuali pakai Ollama lokal).
- User bertanggung jawab punya otorisasi legal untuk scan target — ditegaskan lagi
  sebagai bagian dari consent gate (FR-005), bukan modul terpisah.

---

## 3. Kebutuhan Fungsional

#### FR-001: Target & Scope Input

| Atribut | Detail |
|---------|--------|
| **Prioritas** | High |
| **Deskripsi** | User memasukkan target domain dan level scope/intensity |
| **Input** | Domain string, intensity level (Passive Only / Normal / Aggressive) |
| **Output** | Target + scope config tersimpan |

**Acceptance Criteria:**
- [ ] Terima domain valid, tolak input yang jelas invalid (format salah)
- [ ] User pilih salah satu intensity level
- [ ] Scope config ini yang jadi acuan validasi di FR-003 dan FR-005

#### FR-002: MCP Setup & LLM Provider Connection

| Atribut | Detail |
|---------|--------|
| **Prioritas** | High |
| **Deskripsi** | User konfigurasi koneksi ke satu LLM provider melalui MCP |
| **Input** | Provider type, API key/endpoint |
| **Output** | Koneksi AI aktif, siap orchestrate tools |

**Acceptance Criteria:**
- [ ] Minimal support 1 provider (bebas mau Gemini/Claude/Ollama dulu — provider
      lain ditambah kalau memang dipakai, bukan di-generalize di awal)
- [ ] Test connection sebelum dipakai jalanin recon
- [ ] API key tidak pernah tersimpan/tampil sebagai plaintext di log

#### FR-003: Tool Execution (Host Subprocess — v1, No Sandbox)

| Atribut | Detail |
|---------|--------|
| **Prioritas** | High |
| **Deskripsi** | Tool (passive & active) dijalankan sebagai subprocess langsung di host — bukan di
  Docker container. Sandboxing di-defer ke v1.1+ (lihat §8 Backlog) supaya tahap awal gak perlu
  ngurus per-tool Dockerfile + container lifecycle sekaligus. |
| **Prasyarat** | FR-001 (scope sudah ada) |

**Acceptance Criteria:**
- [ ] Tool dipanggil sebagai child process dari backend
- [ ] Scope check (target cocok scope config) tetap **wajib** dijalankan di kode SEBELUM
      subprocess dipanggil — ini bagian yang **tidak** ikut di-defer
- [ ] Timeout per-tool tetap di-enforce di level proses (kill process kalau timeout)
- [ ] Tool binary yang dibutuhkan (whois, amass, subfinder, nmap, dst) harus sudah ke-install
      di host — didokumentasikan di setup script, bukan tanggung jawab aplikasi
- [ ] **Catatan risiko**: tanpa container isolation, tool jalan dengan akses network host penuh
      (tidak ada egress filtering otomatis) — jalankan Ivy di environment terpisah/VM kalau
      khawatir soal itu

#### FR-004: AI-Orchestrated Passive Reconnaissance

| Atribut | Detail |
|---------|--------|
| **Prioritas** | Critical |
| **Deskripsi** | AI memilih dan menjalankan passive recon tools yang relevan untuk target |
| **Input** | Target + scope (FR-001), koneksi AI (FR-002) |
| **Output** | Hasil temuan tiap tool |
| **Prasyarat** | FR-001, FR-002, FR-003 |

**Acceptance Criteria:**
- [ ] AI pilih tool dari registry passive (whois, dns, amass, subfinder, crt.sh)
- [ ] Tool jalan tanpa perlu konfirmasi tambahan (passive = tidak menyentuh target
      langsung, jadi tidak perlu consent gate)
- [ ] Kalau satu tool gagal, lanjut ke tool lain (tidak menghentikan seluruh run)
- [ ] Hasil diteruskan ke FR-007 (graph storage)

#### FR-005: Consent Gate for Active Reconnaissance

| Atribut | Detail |
|---------|--------|
| **Prioritas** | Critical |
| **Deskripsi** | Sebelum tool active dijalankan, sistem WAJIB minta konfirmasi eksplisit user |
| **Prasyarat** | FR-004 selesai (passive results tersedia) |

**Acceptance Criteria:**
- [ ] User melihat daftar tool active yang AI mau jalankan sebelum eksekusi
- [ ] Konfirmasi berisi pengingat singkat: user harus punya otorisasi legal untuk target
- [ ] Tanpa konfirmasi eksplisit ("ya, lanjutkan"), active recon tidak jalan
- [ ] User bisa berhenti di sini dan langsung lihat hasil passive saja

#### FR-006: AI-Orchestrated Active Reconnaissance

| Atribut | Detail |
|---------|--------|
| **Prioritas** | High |
| **Deskripsi** | Setelah consent (FR-005), AI menjalankan active recon tools |
| **Prasyarat** | FR-005 (consent diberikan) |

**Acceptance Criteria:**
- [ ] AI pilih tool dari registry active (rustscan, nmap, httpx, ffuf)
- [ ] `nuclei` dan tool payload-capable lain **belum** diaktifkan — deferred sampai
      consent-gate ini terbukti solid (lihat `.agents/rules/overview.md`)
- [ ] Scope enforcement sama seperti FR-003/FR-004 (tetap host subprocess, tanpa Docker sandbox
      di v1)
- [ ] Hasil diteruskan ke FR-007

#### FR-007: Result Correlation & Graph Storage

| Atribut | Detail |
|---------|--------|
| **Prioritas** | Critical |
| **Deskripsi** | Hasil semua tool disimpan sebagai nodes/edges di Neo4j |
| **Prasyarat** | FR-004 dan/atau FR-006 menghasilkan output |

**Acceptance Criteria:**
- [ ] Entity dasar jadi node: Domain, Subdomain, IPAddress, Port, Service,
      Technology, Certificate, Email, URL
- [ ] Relasi antar-entity jadi edge (mis. `Domain-[:HAS_SUBDOMAIN]->Subdomain`)
- [ ] Dedup berdasarkan unique key per entity type (mis. subdomain yang sama dari
      Amass & Subfinder tidak dobel)
- [ ] Incremental merge, bukan replace, tiap kali ada hasil baru

#### FR-008: Graph Visualization (Basic)

| Atribut | Detail |
|---------|--------|
| **Prioritas** | High |
| **Deskripsi** | User bisa melihat hasil recon sebagai graph interaktif |
| **Prasyarat** | FR-007 |

**Acceptance Criteria:**
- [ ] Render graph (nodes + edges) dari data Neo4j
- [ ] Klik node menampilkan detail properties-nya
- [ ] Zoom/pan dasar

> Kalau frontend belum sempat dibangun, Neo4j Browser bawaan
> (`http://localhost:7474`) sudah cukup untuk lihat graph — jangan anggap custom
> graph UI blocker untuk MVP loop di atas.

---

## 4. Kebutuhan Non-Fungsional

- **Security** — API key LLM tidak boleh plaintext di log; container non-privileged;
  egress filtering aktif (lihat FR-003).
- **Reliability** — satu tool gagal tidak boleh menghentikan seluruh recon run
  (FR-004/FR-006).
- **Responsiveness** — user harus bisa lihat progress tool berjalan (minimal: status
  per tool, tidak harus real-time streaming penuh di v1).

Tidak ada target angka performa/skalabilitas formal di v1 — belum ada baseline nyata
untuk ditargetkan. Tambahkan kalau sudah ada data pemakaian aktual.

---

## 5. Data Requirements

### 5.1 PostgreSQL (app data)

```sql
CREATE TABLE targets (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    domain      VARCHAR(255) NOT NULL,
    scope_config JSONB NOT NULL DEFAULT '{}',
    created_at  TIMESTAMP DEFAULT NOW()
);

CREATE TABLE scan_sessions (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    target_id   UUID REFERENCES targets(id),
    phase       VARCHAR(20) NOT NULL, -- 'passive' | 'active'
    status      VARCHAR(20) DEFAULT 'pending',
    created_at  TIMESTAMP DEFAULT NOW(),
    completed_at TIMESTAMP
);

CREATE TABLE tool_executions (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_id  UUID REFERENCES scan_sessions(id),
    tool_id     VARCHAR(50) NOT NULL,
    status      VARCHAR(20) DEFAULT 'queued',
    result_summary JSONB,
    error_message TEXT,
    created_at  TIMESTAMP DEFAULT NOW()
);

CREATE TABLE llm_configs (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    provider    VARCHAR(50) NOT NULL,
    model_name  VARCHAR(100),
    api_key_encrypted BYTEA,
    endpoint_url VARCHAR(500),
    created_at  TIMESTAMP DEFAULT NOW()
);
```

*(Tidak ada tabel `users`/`audit_logs` — tidak ada auth/multi-tenant di v1.)*

### 5.2 Neo4j (graph data)

Node & edge schema sama seperti sebelumnya, dipakai apa adanya:

```
(:Domain {name, registrar, creation_date, expiry_date})
(:Subdomain {name, source, first_seen})
(:IPAddress {address, asn, geo_country, isp})
(:Port {number, protocol, state})
(:Service {name, version, banner})
(:Technology {name, version})
(:Certificate {serial, issuer, not_before, not_after})
(:Email {address, source})
(:URL {full_url, status_code})
```

```
(:Domain)-[:HAS_SUBDOMAIN]->(:Subdomain)
(:Subdomain)-[:RESOLVES_TO]->(:IPAddress)
(:IPAddress)-[:HAS_PORT]->(:Port)
(:Port)-[:RUNS_SERVICE]->(:Service)
(:Service)-[:USES_TECHNOLOGY]->(:Technology)
(:Domain)-[:HAS_CERTIFICATE]->(:Certificate)
(:Domain)-[:ASSOCIATED_EMAIL]->(:Email)
(:Subdomain)-[:DISCOVERED_URL]->(:URL)
```

`Vulnerability` node sengaja tidak dimasukkan dulu — belum ada tool yang mengisinya
selama nuclei/exploit masih deferred (FR-006).

---

## 6. External Interface (ringkas)

### 6.1 REST API

| Method | Endpoint | Deskripsi |
|--------|----------|-----------|
| `POST` | `/targets` | Buat target baru + scope config (FR-001) |
| `GET` | `/targets/:id` | Lihat detail target |
| `POST` | `/targets/:id/scan/passive` | Mulai passive recon (FR-004) |
| `POST` | `/targets/:id/scan/active` | Mulai active recon setelah consent (FR-005/006) |
| `GET` | `/targets/:id/scan/status` | Status tool yang berjalan |
| `GET` | `/targets/:id/graph` | Ambil graph data (FR-008) |
| `GET` | `/settings/llm` / `PUT` | Baca/atur konfigurasi LLM (FR-002) |

### 6.2 MCP Tool Schema

Format tetap sama seperti sebelumnya — lihat `tools/ivy-amass/tool.json` sebagai
reference pattern (juga dirujuk di `.agents/rules/overview.md`).

---

## 7. Deployment

Tidak pakai Docker/docker-compose sama sekali di v1 — baik untuk tool (FR-003) maupun untuk
app services sendiri. Neo4j dan PostgreSQL diinstall & dijalankan native di host.

Redis juga dihapus dari stack v1 — belum ada kebutuhan cache/pub-sub konkret tanpa
sesi multi-user. Tambahkan kalau ada use case nyata (mis. WebSocket fan-out lintas
instance), jangan disiapkan duluan.

```bash
git clone <repo-url>
cd ivy
cp .env.example .env    # isi LLM API key + connection string Neo4j/Postgres lokal

cd backend && cargo run      # :3001
cd frontend && npm run dev   # :3000
```

---

## 8. Backlog / Deferred (v1.1+)

Fitur ini **bukan dibuang**, cuma belum relevan sebelum MVP loop di §2 terbukti jalan:

| Fitur | Kenapa deferred |
|-------|------------------|
| Multi-user & auth | v1 self-hosted single-operator, tidak ada kebutuhan login |
| Project CRUD + audit log | Baru relevan kalau sudah multi-user/multi-project |
| Auto-generated report (PDF/HTML) | Belum ada cukup data nyata untuk dirangkum |
| Kill switch (stop-all) | Nice-to-have safety UX, tambah setelah loop dasar stabil |
| AI chat & NL→Cypher query | Bukan bagian dari loop inti; Cypher via MCP tetap harus
  read-only & whitelisted kalau nanti ditambah |
| Plugin marketplace | Baru relevan kalau plugin ekosistem sudah lebih dari satu |
| `nuclei` / exploit tool wiring | Eksplisit deferred, pending validasi consent-gate |
| Docker sandbox per-tool (isolasi container + egress filter) | v1 jalanin tool langsung
  sebagai host subprocess (FR-003) buat kurangin kompleksitas infra tahap awal — gak perlu
  ngurus Dockerfile per tool + container lifecycle sebelum loop dasar kebukti jalan |

---

## 9. Catatan

`Ivy_prd.md` dan `task_division.md` belum ikut disederhanakan di dokumen ini —
keduanya kemungkinan masih mengacu ke scope lama (multi-user, module sebanyak SRS
v1.0). Perlu direview terpisah supaya konsisten dengan SRS v2.0 ini.

---

*Document Version: 2.0 (simplified)*
*Last Updated: 2026-07-13*
