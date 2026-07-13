# 🌿 Ivy — Software Requirements Specification (SRS)

> **Ivy** — *Information Verification & Yield*
> Software Requirements Specification v1.0

---

## 1. Pendahuluan

### 1.1 Tujuan Dokumen

Dokumen ini mendefinisikan **spesifikasi kebutuhan perangkat lunak** (Software Requirements Specification) untuk Ivy — platform OSINT berbasis AI. SRS ini menerjemahkan kebutuhan produk dari [Ivy PRD](./Ivy_prd.md) menjadi kebutuhan teknis yang dapat diimplementasikan oleh tim pengembang.

### 1.2 Ruang Lingkup Produk

**Ivy** adalah aplikasi OSINT open-source yang:
- Menerima input domain dari user
- Menggunakan AI melalui MCP untuk mengorkestrasikan security tools
- Menyimpan dan memvisualisasikan hasil dalam graph database (Neo4j)
- Menyediakan modul eksploitasi opsional sebagai plugin terpisah

### 1.3 Definisi & Akronim

| Istilah | Definisi |
|---------|----------|
| **OSINT** | Open Source Intelligence — pengumpulan informasi dari sumber publik |
| **MCP** | Model Context Protocol — protokol standar untuk komunikasi AI ↔ Tools |
| **LLM** | Large Language Model — model AI yang digunakan sebagai orchestrator |
| **Recon** | Reconnaissance — proses pengumpulan informasi tentang target |
| **Graph DB** | Graph Database — database yang menyimpan data dalam bentuk node dan edge |
| **Passive Recon** | Pengumpulan informasi tanpa berinteraksi langsung dengan target |
| **Active Recon** | Pengumpulan informasi dengan berinteraksi langsung dengan target |
| **Payload** | Kode atau data yang dikirim untuk mengeksploitasi kerentanan |
| **CVE** | Common Vulnerabilities and Exposures — identifikasi standar kerentanan |
| **CVSS** | Common Vulnerability Scoring System — sistem penilaian severity kerentanan |

### 1.4 Referensi

| Dokumen | Lokasi |
|---------|--------|
| Ivy PRD | [Ivy_prd.md](./Ivy_prd.md) |
| MCP Specification | https://modelcontextprotocol.io/specification |
| Neo4j Documentation | https://neo4j.com/docs/ |
| Axum Documentation | https://docs.rs/axum/latest/axum/ |

### 1.5 Arsitektur Keputusan

Berdasarkan PRD yang sudah di-finalize:

| Keputusan | Pilihan |
|-----------|---------|
| Backend | Rust (Axum) |
| Frontend | Next.js + TypeScript |
| Graph DB | Neo4j (Community Edition) |
| LLM | Provider-Agnostic (Gemini, Claude, Ollama, dll.) |
| Deployment | Self-Hosted |
| License | Open Source |
| Tool Execution | Local (Docker Sandbox) |
| Exploit Module | Plugin Terpisah |

---

## 2. Deskripsi Umum Sistem

### 2.1 Perspektif Produk

Ivy adalah sistem **standalone self-hosted** yang terdiri dari:

```
┌─────────────────────────────────────────────────────────────────┐
│                         IVY SYSTEM                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────┐    ┌──────────────┐    ┌──────────────────┐   │
│  │  Frontend    │◄──►│  Backend     │◄──►│  Graph Database  │   │
│  │  (Next.js)   │    │  (Rust/Axum) │    │  (Neo4j)         │   │
│  └─────────────┘    └──────┬───────┘    └──────────────────┘   │
│                            │                                    │
│                     ┌──────┴───────┐                            │
│                     │  MCP Layer   │                            │
│                     │  (AI + Tools)│                            │
│                     └──────┬───────┘                            │
│                            │                                    │
│               ┌────────────┼────────────┐                      │
│               │            │            │                      │
│          ┌────┴───┐  ┌─────┴────┐  ┌────┴────┐                │
│          │Passive │  │ Active   │  │ Exploit │                │
│          │Tools   │  │ Tools    │  │ Plugin  │                │
│          │(Docker)│  │ (Docker) │  │(Docker) │                │
│          └────────┘  └──────────┘  └─────────┘                │
│                                                                 │
│  External:                                                      │
│  ┌──────────┐  ┌───────────┐  ┌──────────────┐                │
│  │ LLM API  │  │ Redis     │  │ Docker Engine│                │
│  │(Provider │  │ (Cache)   │  │ (Container)  │                │
│  │ Agnostic)│  │           │  │              │                │
│  └──────────┘  └───────────┘  └──────────────┘                │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 Fungsi Utama Produk

| ID | Fungsi | Deskripsi |
|----|--------|-----------|
| F1 | Domain Input & Validation | Menerima dan memvalidasi target domain |
| F2 | AI Planning | AI membuat strategi dan memilih tools |
| F3 | Passive Reconnaissance | Menjalankan tools passive recon |
| F4 | Active Reconnaissance | Menjalankan tools active recon |
| F5 | Graph Generation | Menyimpan dan memvisualisasikan hasil dalam graph |
| F6 | Exploitation (Plugin) | Menjalankan exploit tools atas persetujuan user |
| F7 | Report Generation | Membuat laporan hasil recon/exploit |

### 2.3 Karakteristik User

| User Type | Skill Level | Frekuensi Penggunaan |
|-----------|------------|---------------------|
| Penetration Tester | Advanced | Harian |
| Bug Bounty Hunter | Intermediate-Advanced | Harian |
| Red Team Operator | Advanced | Mingguan |
| Security Researcher | Intermediate | Mingguan |
| SOC Analyst | Intermediate | Harian |

### 2.4 Batasan & Asumsi

**Batasan:**
- Sistem berjalan di environment Linux/macOS/WSL (Docker required)
- Membutuhkan koneksi internet untuk passive recon tools dan LLM cloud
- Neo4j Community Edition memiliki limitasi (single database, no clustering)

**Asumsi:**
- User memiliki otorisasi legal untuk scan target
- Docker Engine sudah terinstall di host
- User memiliki API key untuk LLM provider yang dipilih
- Minimal 8GB RAM dan 4 CPU cores untuk menjalankan concurrent scans

---

## 3. Kebutuhan Fungsional

### 3.1 Modul: Authentication & Project Management

#### FR-001: User Authentication

| Atribut | Detail |
|---------|--------|
| **ID** | FR-001 |
| **Prioritas** | High |
| **Deskripsi** | Sistem harus menyediakan autentikasi user berbasis local account |
| **Input** | Username, password |
| **Output** | Session token (JWT) |
| **Prasyarat** | - |

**Acceptance Criteria:**
- [ ] User dapat register dengan username dan password
- [ ] Password di-hash menggunakan Argon2id
- [ ] Sistem mengeluarkan JWT token dengan expiry 24 jam
- [ ] Endpoint protected memvalidasi JWT di setiap request

#### FR-002: Project Management

| Atribut | Detail |
|---------|--------|
| **ID** | FR-002 |
| **Prioritas** | High |
| **Deskripsi** | User harus bisa membuat, melihat, mengedit, dan menghapus project |
| **Input** | Nama project, deskripsi, target domain(s) |
| **Output** | Project object dengan unique ID |
| **Prasyarat** | FR-001 (authenticated) |

**Acceptance Criteria:**
- [ ] User dapat membuat project baru dengan nama dan target domain
- [ ] Setiap project memiliki graph database namespace terpisah
- [ ] User dapat melihat daftar semua project miliknya
- [ ] User dapat menghapus project (soft delete, data dipertahankan 30 hari)

#### FR-003: Legal Disclaimer Consent

| Atribut | Detail |
|---------|--------|
| **ID** | FR-003 |
| **Prioritas** | Critical |
| **Deskripsi** | Sistem WAJIB menampilkan legal disclaimer sebelum menjalankan scan |
| **Input** | User consent (checkbox + confirm) |
| **Output** | Consent record dengan timestamp |
| **Prasyarat** | FR-002 |

**Acceptance Criteria:**
- [ ] Disclaimer muncul setiap kali user membuat project baru
- [ ] Disclaimer menyebutkan UU ITE, CFAA, dan Computer Misuse Act
- [ ] User harus mencentang checkbox "I have authorization"
- [ ] Consent disimpan dengan timestamp untuk audit trail
- [ ] Scan tidak bisa dijalankan tanpa consent

---

### 3.2 Modul: Domain Input & Validation

#### FR-004: Domain Input

| Atribut | Detail |
|---------|--------|
| **ID** | FR-004 |
| **Prioritas** | High |
| **Deskripsi** | User memasukkan target domain untuk di-scan |
| **Input** | Domain string (e.g., `example.com`) |
| **Output** | Validated domain object |
| **Prasyarat** | FR-003 (consent given) |

**Acceptance Criteria:**
- [ ] Menerima format domain valid (e.g., `example.com`, `sub.example.com`)
- [ ] Menolak input invalid (IP address tanpa konfirmasi, URL lengkap, special chars)
- [ ] Mendukung input multiple domains (batch mode)
- [ ] Menyimpan domain ke project

#### FR-005: Scope Configuration

| Atribut | Detail |
|---------|--------|
| **ID** | FR-005 |
| **Prioritas** | High |
| **Deskripsi** | User memilih scope dan intensity level scanning |
| **Input** | Scope level, in-scope/out-of-scope rules |
| **Output** | Scope configuration object |
| **Prasyarat** | FR-004 |

**Acceptance Criteria:**
- [ ] Tiga level intensity: Passive Only (🟢), Normal (🟡), Aggressive (🔴)
- [ ] User dapat menambah wildcard rules (e.g., `*.example.com` in-scope)
- [ ] User dapat menambah out-of-scope exceptions (e.g., `prod.example.com`)
- [ ] Scope configuration disimpan dan enforced oleh semua tools
- [ ] Scope violations di-log dan di-alert

#### FR-006: Domain Validation

| Atribut | Detail |
|---------|--------|
| **ID** | FR-006 |
| **Prioritas** | High |
| **Deskripsi** | Sistem memvalidasi bahwa domain valid dan resolvable |
| **Input** | Domain string |
| **Output** | Validation result (valid/invalid, DNS records) |
| **Prasyarat** | FR-004 |

**Acceptance Criteria:**
- [ ] Cek DNS resolution (A, AAAA record)
- [ ] Cek domain format validity (RFC 1035)
- [ ] Return error yang jelas jika domain tidak resolve
- [ ] Menyimpan initial DNS records ke graph

---

### 3.3 Modul: AI Orchestration (MCP)

#### FR-007: LLM Provider Configuration

| Atribut | Detail |
|---------|--------|
| **ID** | FR-007 |
| **Prioritas** | High |
| **Deskripsi** | User dapat memilih dan mengkonfigurasi LLM provider |
| **Input** | Provider type, API key, model name, endpoint URL |
| **Output** | LLM connection object |
| **Prasyarat** | FR-001 |

**Acceptance Criteria:**
- [ ] Mendukung provider: Gemini, Claude, OpenAI, Ollama (local)
- [ ] API key disimpan encrypted (AES-256-GCM) di database
- [ ] Connection test sebelum menyimpan konfigurasi
- [ ] Fallback provider jika primary gagal (opsional)
- [ ] Custom endpoint URL untuk self-hosted LLM (Ollama)

#### FR-008: AI Recon Planning

| Atribut | Detail |
|---------|--------|
| **ID** | FR-008 |
| **Prioritas** | High |
| **Deskripsi** | AI membuat rencana reconnaissance berdasarkan domain dan scope |
| **Input** | Domain, scope configuration, user context (opsional) |
| **Output** | Execution plan (ordered list of tools + parameters) |
| **Prasyarat** | FR-006, FR-007 |

**Acceptance Criteria:**
- [x] AI menghasilkan execution plan dalam format structured (JSON)
- [x] Plan berisi: tool name, parameters, execution order, dependencies
- [x] Plan mematuhi scope configuration (tidak include tools di luar scope)
- [ ] Plan ditampilkan ke user sebelum eksekusi (review mode)
- [ ] User dapat mengedit plan sebelum menjalankan
- [x] AI mempertimbangkan intensity level saat memilih tools

#### FR-009: MCP Tool Execution

| Atribut | Detail |
|---------|--------|
| **ID** | FR-009 |
| **Prioritas** | Critical |
| **Deskripsi** | Sistem mengeksekusi security tools melalui MCP protocol |
| **Input** | Tool name, parameters (dari execution plan) |
| **Output** | Tool result (structured output) |
| **Prasyarat** | FR-008 |

**Acceptance Criteria:**
- [ ] Setiap tool di-wrap sebagai MCP Tool dengan input/output schema
- [ ] Tools berjalan di Docker container terisolasi
- [x] Timeout configurable per-tool (default: 300 detik)
- [ ] Output di-stream real-time ke frontend via WebSocket
- [x] Error handling: retry 1x, lalu skip dan lanjut ke tool berikutnya
- [ ] Resource limits per container: max 2GB RAM, 1 CPU core

#### FR-010: AI Result Correlation

| Atribut | Detail |
|---------|--------|
| **ID** | FR-010 |
| **Prioritas** | High |
| **Deskripsi** | AI menganalisis dan mengkorelasikan hasil dari semua tools |
| **Input** | Aggregated tool results |
| **Output** | Correlated findings, entity relationships |
| **Prasyarat** | FR-009 (minimal 1 tool complete) |

**Acceptance Criteria:**
- [x] AI menghubungkan temuan yang overlap (e.g., subdomain dari Amass + Subfinder → deduplicate)
- [x] AI mengidentifikasi relasi antar-entitas (subdomain → IP → port → service)
- [x] AI memberikan confidence score per finding (0.0 - 1.0)
- [x] AI memfilter potential false positives
- [x] Hasil korelasi disimpan ke graph database

#### FR-011: Natural Language Interaction

| Atribut | Detail |
|---------|--------|
| **ID** | FR-011 |
| **Prioritas** | Medium |
| **Deskripsi** | User dapat berkomunikasi dengan AI melalui chat natural language |
| **Input** | Chat message (text) |
| **Output** | AI response (text + optional actions) |
| **Prasyarat** | FR-007 |

**Acceptance Criteria:**
- [ ] User bisa bertanya tentang hasil scan (e.g., "berapa subdomain yang ditemukan?")
- [ ] User bisa memberikan instruksi tambahan (e.g., "scan port 443 pada semua subdomain")
- [ ] AI bisa menjalankan tools tambahan berdasarkan instruksi user
- [ ] Chat history disimpan per-project
- [ ] User bisa query graph dengan bahasa natural (e.g., "tampilkan semua service Apache")

---

### 3.4 Modul: Tool Management

#### FR-012: Tool Registry

| Atribut | Detail |
|---------|--------|
| **ID** | FR-012 |
| **Prioritas** | High |
| **Deskripsi** | Sistem menyimpan registry semua available tools dengan metadata |
| **Input** | Tool definition (name, category, schema, Docker image) |
| **Output** | Registered tool entry |
| **Prasyarat** | - |

**Acceptance Criteria:**
- [ ] Setiap tool memiliki: name, description, category, input schema, output schema
- [ ] Tool categories: Passive, Active, Exploit
- [ ] User dapat melihat daftar semua tools yang tersedia
- [ ] User dapat enable/disable tools per-project
- [ ] Tool status: available, running, completed, failed, disabled

**Built-in Tools (Core):**

| Tool ID | Name | Category | Docker Image |
|---------|------|----------|-------------|
| `ivy_whois` | WHOIS Lookup | Passive | `ivy/tool-whois:latest` |
| `ivy_dns` | DNS Enumeration | Passive | `ivy/tool-dns:latest` |
| `ivy_amass` | Amass | Passive/Active | `ivy/tool-amass:latest` |
| `ivy_subfinder` | Subfinder | Passive | `ivy/tool-subfinder:latest` |
| `ivy_crtsh` | crt.sh | Passive | `ivy/tool-crtsh:latest` |
| `ivy_theharvester` | theHarvester | Passive | `ivy/tool-theharvester:latest` |
| `ivy_wayback` | Wayback Machine | Passive | `ivy/tool-wayback:latest` |
| `ivy_shodan` | Shodan | Passive | `ivy/tool-shodan:latest` |
| `ivy_rustscan` | RustScan | Active | `ivy/tool-rustscan:latest` |
| `ivy_nmap` | Nmap | Active | `ivy/tool-nmap:latest` |
| `ivy_httpx` | httpx | Active | `ivy/tool-httpx:latest` |
| `ivy_nuclei` | Nuclei | Active | `ivy/tool-nuclei:latest` |
| `ivy_ffuf` | ffuf | Active | `ivy/tool-ffuf:latest` |
| `ivy_whatweb` | WhatWeb | Active | `ivy/tool-whatweb:latest` |
| `ivy_nikto` | Nikto | Active | `ivy/tool-nikto:latest` |

#### FR-013: Tool Sandboxing

| Atribut | Detail |
|---------|--------|
| **ID** | FR-013 |
| **Prioritas** | Critical |
| **Deskripsi** | Semua tools HARUS berjalan dalam Docker container terisolasi |
| **Input** | Tool execution request |
| **Output** | Sandboxed execution result |
| **Prasyarat** | FR-012, Docker Engine |

**Acceptance Criteria:**
- [ ] Setiap tool berjalan di container sendiri
- [ ] Container tidak memiliki akses ke host filesystem (kecuali volume output)
- [ ] Network access dibatasi sesuai scope (egress filtering)
- [ ] Resource limits: configurable RAM (default 2GB), CPU (default 1 core)
- [ ] Container di-destroy setelah execution selesai
- [ ] Timeout enforcement: container di-kill setelah timeout

#### FR-014: Plugin System (Exploit Module)

| Atribut | Detail |
|---------|--------|
| **ID** | FR-014 |
| **Prioritas** | Medium |
| **Deskripsi** | Exploit tools di-ship sebagai plugin yang harus di-install terpisah |
| **Input** | Plugin package |
| **Output** | Installed plugin tools |
| **Prasyarat** | FR-012 |

**Acceptance Criteria:**
- [ ] Plugin format standar: manifest.json + Docker image + MCP tool schema
- [ ] Plugin di-install via CLI: `ivy plugin install ivy-exploit`
- [ ] Plugin di-uninstall via CLI: `ivy plugin remove ivy-exploit`
- [ ] Exploit plugin berisi: SQLMap, XSStrike, Commix, Hydra
- [ ] Double consent required sebelum menjalankan exploit tool
- [ ] Plugin marketplace/registry untuk community contributions (future)

**Exploit Plugin Tools:**

| Tool ID | Name | Category |
|---------|------|----------|
| `ivy_sqlmap` | SQLMap | Exploit |
| `ivy_xsstrike` | XSStrike | Exploit |
| `ivy_commix` | Commix | Exploit |
| `ivy_hydra` | Hydra | Exploit |
| `ivy_metasploit` | Metasploit | Exploit |
| `ivy_crackmapexec` | CrackMapExec | Exploit |

---

### 3.5 Modul: Graph Database & Visualization

#### FR-015: Graph Data Storage

| Atribut | Detail |
|---------|--------|
| **ID** | FR-015 |
| **Prioritas** | Critical |
| **Deskripsi** | Semua findings disimpan ke Neo4j sebagai nodes dan edges |
| **Input** | Structured findings dari tools |
| **Output** | Graph nodes dan edges di Neo4j |
| **Prasyarat** | FR-009 |

**Acceptance Criteria:**
- [ ] Setiap entity type menjadi node label di Neo4j
- [ ] Relationships antar-entity menjadi edges
- [ ] Data di-deduplicate berdasarkan unique key per entity type
- [ ] Incremental update (merge, bukan replace)
- [ ] Metadata per-node: source tool, timestamp, confidence score

**Node Schema:**

```
(:Domain {name, registrar, creation_date, expiry_date, whois_data})
(:Subdomain {name, source, first_seen, last_seen})
(:IPAddress {address, version, asn, geo_country, geo_city, isp})
(:Port {number, protocol, state})
(:Service {name, version, banner, cpe})
(:Technology {name, version, category})
(:Vulnerability {cve_id, severity, cvss_score, description, reference_url})
(:Email {address, source})
(:Certificate {serial, issuer, subject, not_before, not_after, san_list})
(:URL {full_url, path, status_code, content_type, content_length})
```

**Edge Schema:**

```
(:Domain)-[:HAS_SUBDOMAIN]->(:Subdomain)
(:Subdomain)-[:RESOLVES_TO]->(:IPAddress)
(:IPAddress)-[:HAS_PORT]->(:Port)
(:Port)-[:RUNS_SERVICE]->(:Service)
(:Service)-[:USES_TECHNOLOGY]->(:Technology)
(:Service)-[:HAS_VULNERABILITY]->(:Vulnerability)
(:Technology)-[:HAS_VULNERABILITY]->(:Vulnerability)
(:Domain)-[:ASSOCIATED_EMAIL]->(:Email)
(:Domain)-[:HAS_CERTIFICATE]->(:Certificate)
(:Subdomain)-[:HAS_CERTIFICATE]->(:Certificate)
(:Domain)-[:DISCOVERED_URL]->(:URL)
(:Subdomain)-[:DISCOVERED_URL]->(:URL)
```

#### FR-016: Graph Visualization

| Atribut | Detail |
|---------|--------|
| **ID** | FR-016 |
| **Prioritas** | High |
| **Deskripsi** | Menampilkan graph interaktif di dashboard web |
| **Input** | Graph data dari Neo4j |
| **Output** | Interactive graph canvas |
| **Prasyarat** | FR-015 |

**Acceptance Criteria:**
- [ ] Render graph menggunakan Cytoscape.js
- [ ] Node diberi warna berdasarkan type (Domain=hijau, Vuln=merah, dll.)
- [ ] Zoom, pan, dan drag support
- [ ] Filter by node type (toggle visibility)
- [ ] Search node by name/property
- [ ] Node clustering untuk mengurangi visual clutter
- [ ] Graph update real-time saat scan berlangsung
- [ ] Performance: render < 2 detik untuk 10.000 nodes

#### FR-017: Node Inspector

| Atribut | Detail |
|---------|--------|
| **ID** | FR-017 |
| **Prioritas** | High |
| **Deskripsi** | Menampilkan detail informasi saat user klik node di graph |
| **Input** | Node click event |
| **Output** | Node properties panel |
| **Prasyarat** | FR-016 |

**Acceptance Criteria:**
- [ ] Klik node menampilkan side panel dengan semua properties
- [ ] Menampilkan related nodes (neighbors)
- [ ] Menampilkan source tool dan timestamp discovery
- [ ] Link ke external references (CVE detail, Shodan page, dll.)
- [ ] Bisa trigger additional scan dari node (e.g., klik IP → scan ports)

#### FR-018: Graph Query

| Atribut | Detail |
|---------|--------|
| **ID** | FR-018 |
| **Prioritas** | Medium |
| **Deskripsi** | User dapat query graph menggunakan Cypher atau natural language |
| **Input** | Cypher query atau natural language question |
| **Output** | Query result (table atau sub-graph) |
| **Prasyarat** | FR-015, FR-011 |

**Acceptance Criteria:**
- [ ] Raw Cypher query mode untuk advanced users
- [ ] Natural language query diterjemahkan ke Cypher oleh AI
- [ ] Result ditampilkan sebagai tabel atau highlighted sub-graph
- [ ] Query history disimpan per-project
- [ ] Pre-built query templates (e.g., "All Critical Vulns", "Open Ports > 1024")

#### FR-019: Graph Export

| Atribut | Detail |
|---------|--------|
| **ID** | FR-019 |
| **Prioritas** | Medium |
| **Deskripsi** | User dapat export graph data ke berbagai format |
| **Input** | Export request + format selection |
| **Output** | File download |
| **Prasyarat** | FR-015 |

**Acceptance Criteria:**
- [ ] Export format: JSON, CSV, GraphML
- [ ] Pilihan: export semua data atau filtered subset
- [ ] Include metadata (source, timestamp, confidence)
- [ ] File size limit warning untuk graph besar (> 100MB)

---

### 3.6 Modul: Reporting

#### FR-020: Auto-Generated Report

| Atribut | Detail |
|---------|--------|
| **ID** | FR-020 |
| **Prioritas** | Medium |
| **Deskripsi** | Sistem menghasilkan report otomatis dari hasil scanning |
| **Input** | Project data + graph data |
| **Output** | PDF/HTML report |
| **Prasyarat** | FR-015 |

**Acceptance Criteria:**
- [ ] Report format: PDF dan HTML
- [ ] Sections: Executive Summary, Scope, Methodology, Findings, Recommendations
- [ ] Findings dikelompokkan berdasarkan severity (Critical → Info)
- [ ] Include graph visualization snapshot
- [ ] Include tabel detail setiap finding
- [ ] AI-generated executive summary
- [ ] Customizable report template

#### FR-021: Finding Classification

| Atribut | Detail |
|---------|--------|
| **ID** | FR-021 |
| **Prioritas** | High |
| **Deskripsi** | Setiap finding diklasifikasi berdasarkan severity level |
| **Input** | Finding data |
| **Output** | Classified finding dengan severity tag |
| **Prasyarat** | FR-010 |

**Acceptance Criteria:**
- [ ] Severity levels: Critical, High, Medium, Low, Informational
- [ ] Auto-classification berdasarkan CVSS score (jika CVE)
- [ ] AI-assisted classification untuk non-CVE findings
- [ ] User dapat override severity classification manual
- [ ] Statistik summary per severity level

---

### 3.7 Modul: Real-time & Monitoring

#### FR-022: Real-time Tool Output Streaming

| Atribut | Detail |
|---------|--------|
| **ID** | FR-022 |
| **Prioritas** | High |
| **Deskripsi** | Output dari tools yang sedang berjalan di-stream real-time ke frontend |
| **Input** | Tool stdout/stderr |
| **Output** | WebSocket stream ke frontend |
| **Prasyarat** | FR-009 |

**Acceptance Criteria:**
- [ ] WebSocket connection per-project
- [ ] Stream: tool name, status, stdout, stderr, progress percentage
- [ ] Terminal-like display di frontend
- [ ] Scroll-back buffer (last 10.000 lines per tool)
- [ ] Download full log per tool

#### FR-023: Scan Progress Tracking

| Atribut | Detail |
|---------|--------|
| **ID** | FR-023 |
| **Prioritas** | High |
| **Deskripsi** | Dashboard menampilkan progress setiap tool yang sedang berjalan |
| **Input** | Tool execution status |
| **Output** | Progress UI |
| **Prasyarat** | FR-009 |

**Acceptance Criteria:**
- [ ] Progress bar per-tool (0-100%)
- [ ] Status badge: Queued, Running, Completed, Failed, Skipped
- [ ] Total scan progress (overall percentage)
- [ ] Estimated time remaining (berdasarkan historical data)
- [ ] Bisa pause/resume/cancel individual tools
- [ ] Bisa cancel semua tools sekaligus (Kill Switch)

#### FR-024: Kill Switch

| Atribut | Detail |
|---------|--------|
| **ID** | FR-024 |
| **Prioritas** | Critical |
| **Deskripsi** | User dapat menghentikan SEMUA tools yang sedang berjalan sekaligus |
| **Input** | Kill switch button press |
| **Output** | All containers stopped |
| **Prasyarat** | FR-009 |

**Acceptance Criteria:**
- [ ] Satu tombol "Emergency Stop" yang visible di setiap halaman
- [ ] Menghentikan semua running containers dalam < 5 detik
- [ ] Menyimpan partial results yang sudah dikumpulkan
- [ ] Log alasan stop (user-initiated)
- [ ] Confirmation dialog sebelum execute

---

### 3.8 Modul: Settings & Configuration

#### FR-025: LLM Provider Settings

| Atribut | Detail |
|---------|--------|
| **ID** | FR-025 |
| **Prioritas** | High |
| **Deskripsi** | UI untuk mengelola konfigurasi LLM provider |
| **Input** | Provider settings |
| **Output** | Saved configuration |
| **Prasyarat** | FR-007 |

**Acceptance Criteria:**
- [ ] Form UI untuk setiap provider (Gemini, Claude, OpenAI, Ollama)
- [ ] Input: API key, model name, endpoint URL, temperature, max tokens
- [ ] Test connection button
- [ ] Set default/primary provider
- [ ] API key masked di UI (show/hide toggle)

#### FR-026: Tool Configuration

| Atribut | Detail |
|---------|--------|
| **ID** | FR-026 |
| **Prioritas** | Medium |
| **Deskripsi** | User dapat mengkonfigurasi default parameters per-tool |
| **Input** | Tool parameters |
| **Output** | Saved tool configuration |
| **Prasyarat** | FR-012 |

**Acceptance Criteria:**
- [ ] UI per-tool untuk mengatur default parameters
- [ ] Override parameters per-project
- [ ] Rate limiting configuration (requests/second)
- [ ] Timeout configuration per-tool
- [ ] Custom wordlists (untuk ffuf, Hydra, dll.)

#### FR-027: Audit Log

| Atribut | Detail |
|---------|--------|
| **ID** | FR-027 |
| **Prioritas** | High |
| **Deskripsi** | Semua aktivitas user dan sistem di-log untuk audit trail |
| **Input** | System events |
| **Output** | Audit log entries |
| **Prasyarat** | - |

**Acceptance Criteria:**
- [ ] Log: user actions, tool executions, configuration changes
- [ ] Log format: timestamp, user_id, action, target, details
- [ ] Searchable dan filterable di UI
- [ ] Export log ke CSV/JSON
- [ ] Log retention: configurable (default 90 hari)
- [ ] Tamper-proof: log append-only, tidak bisa di-edit/delete oleh user

---

## 4. Kebutuhan Non-Fungsional

### 4.1 Performance

| ID | Requirement | Target | Measurement |
|----|------------|--------|-------------|
| NFR-001 | Graph render time | < 2 detik untuk 10.000 nodes | Time to interactive |
| NFR-002 | API response time | < 500ms (p95) untuk REST endpoints | Latency percentile |
| NFR-003 | WebSocket latency | < 100ms tool output delay | Stream delay |
| NFR-004 | AI response time | < 5 detik untuk planning | First token time |
| NFR-005 | Concurrent tools | Hingga 10 tools paralel per-project | Load test |
| NFR-006 | Graph query time | < 1 detik untuk queries pada 100.000 nodes | Cypher query time |

### 4.2 Security

| ID | Requirement | Detail |
|----|------------|--------|
| NFR-007 | Encryption at rest | API keys encrypted AES-256-GCM |
| NFR-008 | Encryption in transit | HTTPS (TLS 1.3) untuk semua komunikasi |
| NFR-009 | Container isolation | Docker containers dengan no-privileged, read-only rootfs |
| NFR-010 | Scope enforcement | Network egress filtering berdasarkan scope |
| NFR-011 | Input sanitization | Semua user input di-sanitize sebelum diproses |
| NFR-012 | Dependency audit | Cargo audit pada setiap release |

### 4.3 Reliability

| ID | Requirement | Target |
|----|------------|--------|
| NFR-013 | Uptime | 99.5% (self-hosted, tidak termasuk planned maintenance) |
| NFR-014 | Data durability | Graph data persisted ke disk, recoverable setelah crash |
| NFR-015 | Graceful degradation | Jika satu tool gagal, scan lanjut ke tool berikutnya |
| NFR-016 | Auto-recovery | Container auto-restart jika crash (max 3 retries) |

### 4.4 Scalability

| ID | Requirement | Target |
|----|------------|--------|
| NFR-017 | Max concurrent projects | 10 active scans per-instance |
| NFR-018 | Max graph size | 500.000 nodes / 2.000.000 edges per-project |
| NFR-019 | Max concurrent users | 20 per-instance |
| NFR-020 | Storage growth | < 500MB per project (average) |

### 4.5 Usability

| ID | Requirement | Detail |
|----|------------|--------|
| NFR-021 | Browser support | Chrome, Firefox, Edge (latest 2 major versions) |
| NFR-022 | Responsive design | Minimum viewport: 1280x720 |
| NFR-023 | Accessibility | Keyboard navigation untuk semua core functions |
| NFR-024 | Documentation | In-app help tooltips + full documentation site |

### 4.6 Maintainability

| ID | Requirement | Detail |
|----|------------|--------|
| NFR-025 | Code coverage | Minimum 70% test coverage |
| NFR-026 | CI/CD | Automated build, test, lint pada setiap PR |
| NFR-027 | Logging | Structured JSON logging (tracing crate) |
| NFR-028 | Config management | Environment-based config (dotenv + TOML) |

---

## 5. External Interface Requirements

### 5.1 User Interface

```
┌──────────────────────────────────────────────────────────────────────┐
│  🌿 Ivy                              [Kill Switch 🛑] [⚙️] [👤]    │
├──────────┬───────────────────────────────────────────────────────────┤
│          │  ┌─ Tabs ──────────────────────────────────────────────┐  │
│ SIDEBAR  │  │ [Graph] [Findings] [Terminal] [Report]             │  │
│          │  └────────────────────────────────────────────────────┘  │
│ Projects │                                                         │
│ ├ Proj 1 │  ┌─ MAIN CONTENT AREA ──────────────────────────────┐  │
│ ├ Proj 2 │  │                                                   │  │
│ └ Proj 3 │  │  (Content changes based on active tab)            │  │
│          │  │                                                   │  │
│ ──────── │  │  Graph Tab: Interactive graph visualization       │  │
│ Scan     │  │  Findings: Sortable vulnerability table           │  │
│ Progress │  │  Terminal: Real-time tool output                  │  │
│ ┌──────┐ │  │  Report: Generated report preview                │  │
│ │amass │ │  │                                                   │  │
│ │██░░░ │ │  └───────────────────────────────────────────────────┘  │
│ │40%   │ │                                                         │
│ └──────┘ │  ┌─ AI CHAT PANEL ──────────────────────────────────┐  │
│ ┌──────┐ │  │ 🤖: Found 23 subdomains across 3 tools           │  │
│ │httpx │ │  │ 🧑: Focus on port 443                            │  │
│ │██████│ │  │ 🤖: Running httpx on 23 targets...               │  │
│ │Done ✓│ │  ├────────────────────────────────────────────────── │  │
│ └──────┘ │  │ [Type message...]                        [Send]  │  │
│          │  └──────────────────────────────────────────────────┘  │
└──────────┴───────────────────────────────────────────────────────────┘
```

**Halaman-halaman UI:**

| Page | Route | Deskripsi |
|------|-------|-----------|
| Login | `/login` | Halaman autentikasi |
| Dashboard | `/` | Daftar projects + quick stats |
| Project Detail | `/project/:id` | Graph + AI Chat + Tools (main workspace) |
| Settings | `/settings` | LLM config, tool config, account |
| Plugin Manager | `/settings/plugins` | Install/remove plugins |
| Audit Log | `/audit` | Searchable activity log |

### 5.2 Backend REST API

**Base URL:** `http://localhost:3001/api/v1`

#### Authentication

| Method | Endpoint | Deskripsi |
|--------|----------|-----------|
| `POST` | `/auth/register` | Register user baru |
| `POST` | `/auth/login` | Login, return JWT |
| `POST` | `/auth/refresh` | Refresh JWT token |

#### Projects

| Method | Endpoint | Deskripsi |
|--------|----------|-----------|
| `GET` | `/projects` | List semua projects |
| `POST` | `/projects` | Create project baru |
| `GET` | `/projects/:id` | Get project detail |
| `PUT` | `/projects/:id` | Update project |
| `DELETE` | `/projects/:id` | Soft delete project |
| `POST` | `/projects/:id/consent` | Record legal consent |

#### Scanning

| Method | Endpoint | Deskripsi |
|--------|----------|-----------|
| `POST` | `/projects/:id/scan/start` | Mulai scan (AI plans + executes) |
| `POST` | `/projects/:id/scan/stop` | Kill switch — stop semua tools |
| `GET` | `/projects/:id/scan/status` | Get scan status + progress |
| `POST` | `/projects/:id/scan/tools/:toolId/run` | Jalankan tool individual |
| `POST` | `/projects/:id/scan/tools/:toolId/stop` | Stop tool individual |

#### Graph

| Method | Endpoint | Deskripsi |
|--------|----------|-----------|
| `GET` | `/projects/:id/graph` | Get full graph data |
| `GET` | `/projects/:id/graph/nodes` | Get nodes (filterable by type) |
| `GET` | `/projects/:id/graph/nodes/:nodeId` | Get node detail |
| `POST` | `/projects/:id/graph/query` | Execute Cypher query |
| `GET` | `/projects/:id/graph/export/:format` | Export graph (json/csv/graphml) |
| `GET` | `/projects/:id/graph/stats` | Graph statistics |

#### AI Chat

| Method | Endpoint | Deskripsi |
|--------|----------|-----------|
| `POST` | `/projects/:id/chat` | Send message ke AI |
| `GET` | `/projects/:id/chat/history` | Get chat history |

#### Tools & Plugins

| Method | Endpoint | Deskripsi |
|--------|----------|-----------|
| `GET` | `/tools` | List semua available tools |
| `GET` | `/tools/:toolId` | Get tool detail + schema |
| `GET` | `/plugins` | List installed plugins |
| `POST` | `/plugins/install` | Install plugin |
| `DELETE` | `/plugins/:pluginId` | Remove plugin |

#### Reports

| Method | Endpoint | Deskripsi |
|--------|----------|-----------|
| `POST` | `/projects/:id/report/generate` | Generate report |
| `GET` | `/projects/:id/report/:format` | Download report (pdf/html) |

#### Settings

| Method | Endpoint | Deskripsi |
|--------|----------|-----------|
| `GET` | `/settings/llm` | Get LLM configuration |
| `PUT` | `/settings/llm` | Update LLM configuration |
| `POST` | `/settings/llm/test` | Test LLM connection |
| `GET` | `/audit/logs` | Get audit logs (paginated) |

### 5.3 WebSocket API

**Endpoint:** `ws://localhost:3001/ws`

**Events (Server → Client):**

| Event | Payload | Deskripsi |
|-------|---------|-----------|
| `scan:progress` | `{project_id, tool_id, percentage, status}` | Tool progress update |
| `scan:output` | `{project_id, tool_id, stream, line}` | Tool stdout/stderr line |
| `scan:complete` | `{project_id, tool_id, result_summary}` | Tool execution finished |
| `scan:error` | `{project_id, tool_id, error}` | Tool execution error |
| `graph:update` | `{project_id, nodes_added, edges_added}` | Graph data updated |
| `ai:message` | `{project_id, role, content}` | AI chat message |
| `ai:thinking` | `{project_id, status}` | AI processing indicator |

**Events (Client → Server):**

| Event | Payload | Deskripsi |
|-------|---------|-----------|
| `subscribe` | `{project_id}` | Subscribe to project updates |
| `unsubscribe` | `{project_id}` | Unsubscribe from project |
| `chat:send` | `{project_id, message}` | Send chat message |

### 5.4 MCP Tool Interface

Setiap security tool mengikuti standar MCP Tool schema:

```json
{
  "name": "ivy_<tool_name>",
  "description": "Tool description",
  "inputSchema": {
    "type": "object",
    "properties": {
      "target": {
        "type": "string",
        "description": "Target domain or IP"
      }
    },
    "required": ["target"]
  }
}
```

**MCP Tool Response Format:**

```json
{
  "tool_id": "ivy_amass",
  "status": "completed",
  "execution_time_ms": 45000,
  "findings": [
    {
      "type": "Subdomain",
      "data": {
        "name": "api.example.com",
        "source": "amass_passive"
      },
      "confidence": 0.95,
      "relationships": [
        {
          "type": "HAS_SUBDOMAIN",
          "from_type": "Domain",
          "from_key": "example.com"
        }
      ]
    }
  ],
  "raw_output": "...(full tool stdout)...",
  "errors": []
}
```

---

## 6. Data Requirements

### 6.1 Database Schema (PostgreSQL — Application Data)

```sql
-- Users
CREATE TABLE users (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username    VARCHAR(50) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at  TIMESTAMP DEFAULT NOW(),
    updated_at  TIMESTAMP DEFAULT NOW()
);

-- Projects
CREATE TABLE projects (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID REFERENCES users(id),
    name        VARCHAR(100) NOT NULL,
    description TEXT,
    target_domains TEXT[] NOT NULL,
    scope_config JSONB NOT NULL DEFAULT '{}',
    consent_given BOOLEAN DEFAULT FALSE,
    consent_at  TIMESTAMP,
    status      VARCHAR(20) DEFAULT 'created',
    created_at  TIMESTAMP DEFAULT NOW(),
    updated_at  TIMESTAMP DEFAULT NOW(),
    deleted_at  TIMESTAMP
);

-- Scan Sessions
CREATE TABLE scan_sessions (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id  UUID REFERENCES projects(id),
    status      VARCHAR(20) DEFAULT 'pending',
    intensity   VARCHAR(20) NOT NULL,
    execution_plan JSONB,
    started_at  TIMESTAMP,
    completed_at TIMESTAMP,
    created_at  TIMESTAMP DEFAULT NOW()
);

-- Tool Executions
CREATE TABLE tool_executions (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_id  UUID REFERENCES scan_sessions(id),
    tool_id     VARCHAR(50) NOT NULL,
    status      VARCHAR(20) DEFAULT 'queued',
    parameters  JSONB,
    result_summary JSONB,
    raw_output_path VARCHAR(500),
    started_at  TIMESTAMP,
    completed_at TIMESTAMP,
    error_message TEXT,
    container_id VARCHAR(100),
    created_at  TIMESTAMP DEFAULT NOW()
);

-- Chat Messages
CREATE TABLE chat_messages (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id  UUID REFERENCES projects(id),
    role        VARCHAR(20) NOT NULL,
    content     TEXT NOT NULL,
    tool_calls  JSONB,
    created_at  TIMESTAMP DEFAULT NOW()
);

-- Audit Logs
CREATE TABLE audit_logs (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID REFERENCES users(id),
    action      VARCHAR(100) NOT NULL,
    target_type VARCHAR(50),
    target_id   VARCHAR(100),
    details     JSONB,
    ip_address  INET,
    created_at  TIMESTAMP DEFAULT NOW()
);

-- LLM Configurations
CREATE TABLE llm_configs (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID REFERENCES users(id),
    provider    VARCHAR(50) NOT NULL,
    model_name  VARCHAR(100),
    api_key_encrypted BYTEA,
    endpoint_url VARCHAR(500),
    is_default  BOOLEAN DEFAULT FALSE,
    settings    JSONB DEFAULT '{}',
    created_at  TIMESTAMP DEFAULT NOW(),
    updated_at  TIMESTAMP DEFAULT NOW()
);
```

### 6.2 Graph Schema (Neo4j — Scan Data)

Lihat **FR-015** untuk detail node dan edge schema.

**Indexes yang diperlukan:**

```cypher
CREATE INDEX FOR (d:Domain) ON (d.name);
CREATE INDEX FOR (s:Subdomain) ON (s.name);
CREATE INDEX FOR (i:IPAddress) ON (i.address);
CREATE INDEX FOR (p:Port) ON (p.number);
CREATE INDEX FOR (v:Vulnerability) ON (v.cve_id);
CREATE INDEX FOR (e:Email) ON (e.address);
CREATE INDEX FOR (u:URL) ON (u.full_url);
```

### 6.3 Cache Schema (Redis)

| Key Pattern | Value | TTL |
|-------------|-------|-----|
| `session:{user_id}` | JWT metadata | 24h |
| `scan:progress:{session_id}` | Progress JSON | 1h |
| `tool:output:{execution_id}:buffer` | Last 1000 lines | 1h |
| `graph:stats:{project_id}` | Node/edge counts | 5min |
| `llm:rate_limit:{provider}` | Request counter | 1min |

---

## 7. Error Handling & Recovery

### 7.1 Error Categories

| Category | HTTP Code | Handling |
|----------|-----------|----------|
| **Validation Error** | 400 | Return field-level error messages |
| **Authentication Error** | 401 | Redirect to login |
| **Authorization Error** | 403 | Return "access denied" |
| **Resource Not Found** | 404 | Return "not found" message |
| **Tool Execution Error** | 500 | Log error, retry 1x, skip tool jika masih gagal |
| **LLM Provider Error** | 502 | Retry 3x with exponential backoff, fallback provider |
| **Container Timeout** | 504 | Kill container, log timeout, lanjut ke tool berikutnya |
| **Scope Violation** | 403 | Block execution, log violation, alert user |

### 7.2 Error Response Format

```json
{
  "error": {
    "code": "TOOL_EXECUTION_FAILED",
    "message": "Amass failed to execute within timeout",
    "details": {
      "tool_id": "ivy_amass",
      "timeout_seconds": 300,
      "container_id": "abc123"
    },
    "timestamp": "2026-07-12T12:00:00Z",
    "request_id": "req_xyz789"
  }
}
```

### 7.3 Recovery Strategies

| Scenario | Recovery |
|----------|----------|
| Tool crash | Auto-restart container (max 3x), lalu skip |
| Neo4j connection lost | Retry connection 5x (exponential backoff), buffer writes |
| Redis down | Fallback ke in-memory cache, degrade gracefully |
| LLM timeout | Retry 3x, fallback ke secondary provider |
| Docker daemon unreachable | Alert user, block scan operations |
| Disk full | Alert user, stop new scans, preserve existing data |

---

## 8. Deployment Requirements

### 8.1 Docker Compose Stack

```yaml
# docker-compose.yml (reference architecture)
services:
  ivy-backend:
    image: ivy/backend:latest
    ports: ["3001:3001"]
    depends_on: [neo4j, redis, postgres]
    environment:
      - DATABASE_URL=postgresql://ivy:password@postgres/ivy
      - NEO4J_URI=bolt://neo4j:7687
      - REDIS_URL=redis://redis:6379

  ivy-frontend:
    image: ivy/frontend:latest
    ports: ["3000:3000"]
    depends_on: [ivy-backend]

  neo4j:
    image: neo4j:5-community
    ports: ["7474:7474", "7687:7687"]
    volumes: ["neo4j_data:/data"]

  redis:
    image: redis:7-alpine
    ports: ["6379:6379"]

  postgres:
    image: postgres:16-alpine
    ports: ["5432:5432"]
    volumes: ["postgres_data:/var/lib/postgresql/data"]
```

### 8.2 Minimum System Requirements

| Resource | Minimum | Recommended |
|----------|---------|-------------|
| **CPU** | 4 cores | 8 cores |
| **RAM** | 8 GB | 16 GB |
| **Disk** | 50 GB SSD | 100 GB SSD |
| **OS** | Linux (Ubuntu 22.04+), macOS 13+, Windows 11 (WSL2) |
| **Docker** | Docker Engine 24+ | Docker Desktop / Podman |
| **Network** | Internet access untuk passive recon + LLM API |

### 8.3 Installation

```bash
# Quick start
git clone https://github.com/ivy-osint/ivy.git
cd ivy
cp .env.example .env          # Configure LLM API keys
docker compose up -d           # Start all services
# Open http://localhost:3000
```

---

## 9. Traceability Matrix

Mapping antara Functional Requirements → PRD Sections:

| FR ID | PRD Section | Priority | Phase |
|-------|-------------|----------|-------|
| FR-001 | 3 (Target Users) | High | 1 - MVP |
| FR-002 | 4.1 (User Input) | High | 1 - MVP |
| FR-003 | 9 (Security & Ethics) | Critical | 1 - MVP |
| FR-004 | 4.1 (User Input) | High | 1 - MVP |
| FR-005 | 7.1 (Scope Control) | High | 1 - MVP |
| FR-006 | 4.2 (AI Verification) | High | 1 - MVP |
| FR-007 | 14 (#1 LLM Provider) | High | 1 - MVP |
| FR-008 | 4.2 (AI Planning) | High | 1 - MVP |
| FR-009 | 4.3 (Recon Execution) | Critical | 1 - MVP |
| FR-010 | 7.4 (AI Analysis) | High | 2 - Core |
| FR-011 | 7.4 (NL Query) | Medium | 3 - Intel |
| FR-012 | 6.2 (MCP Tools) | High | 1 - MVP |
| FR-013 | 9.1 (Sandboxing) | Critical | 1 - MVP |
| FR-014 | 14 (#6 Plugin) | Medium | 3 - Intel |
| FR-015 | 4.4 (Graph Generation) | Critical | 1 - MVP |
| FR-016 | 8 (UI Design) | High | 1 - MVP |
| FR-017 | 8.2 (Node Inspector) | High | 2 - Core |
| FR-018 | 7.4 (NL Query) | Medium | 3 - Intel |
| FR-019 | 7.2 (Export) | Medium | 2 - Core |
| FR-020 | 7.2 (Reporting) | Medium | 2 - Core |
| FR-021 | 7.2 (Finding Class.) | High | 2 - Core |
| FR-022 | 8.2 (Tool Stream) | High | 2 - Core |
| FR-023 | 8.2 (Scan Progress) | High | 1 - MVP |
| FR-024 | 9.1 (Kill Switch) | Critical | 1 - MVP |
| FR-025 | 14 (#1 LLM Config) | High | 1 - MVP |
| FR-026 | 8.2 (Tool Config) | Medium | 2 - Core |
| FR-027 | 7.6 (Audit Log) | High | 2 - Core |

---

*Document Version: 1.0*
*Last Updated: 2026-07-12*
*Author: Ivy Development Team*
*Reference: [Ivy PRD v1.1](./Ivy_prd.md)*
