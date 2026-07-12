# 🌿 Ivy — Product Requirements Document (PRD)

> **Ivy** — *Information Verification & Yield*
> Seperti tanaman ivy yang merambat perlahan namun pasti, Ivy merayap ke setiap celah informasi dan menyusup membawa payload ke target.

---

## 1. Executive Summary

**Ivy** adalah software OSINT (Open Source Intelligence) berbasis AI yang memanfaatkan arsitektur **MCP (Model Context Protocol)** untuk melakukan reconnaissance, analisis kerentanan, dan eksploitasi opsional terhadap target domain. Hasil seluruh proses divisualisasikan dalam bentuk **Graph Database** yang interaktif, memberikan gambaran menyeluruh tentang attack surface target.

### 1.1 Problem Statement

Proses OSINT dan penetration testing saat ini membutuhkan penggunaan banyak tools terpisah (Amass, RustScan, SQLMap, dll.) secara manual, memakan waktu, dan sulit untuk menghubungkan temuan antar-tools. Security researcher membutuhkan:

- **Otomasi** — menjalankan banyak tools secara terkoordinasi
- **Korelasi** — menghubungkan temuan dari berbagai sumber menjadi satu graph
- **Kecerdasan** — AI yang bisa mengambil keputusan tools mana yang digunakan dan kapan
- **Visualisasi** — melihat hubungan antar-entitas (domain, subdomain, IP, port, service, vulnerability) secara visual

### 1.2 Proposed Solution

Ivy menggunakan AI sebagai **orchestrator** melalui MCP, di mana AI memilih, menjalankan, dan mengkorelasikan hasil dari berbagai security tools. Seluruh temuan disimpan dalam graph database dan divisualisasikan secara real-time.

---

## 2. Product Vision & Philosophy

```
🌿 Ivy merambat — perlahan, tenang, namun menjangkau setiap celah.
```

| Aspek | Deskripsi |
|-------|-----------|
| **Nama** | Ivy (Information Verification & Yield) |
| **Filosofi** | Seperti tanaman ivy yang merambat perlahan tapi pasti ke setiap permukaan, Ivy menyusuri setiap celah informasi target dan mampu menyisipkan payload ke aplikasi target |
| **Pendekatan** | AI-first — AI yang menentukan strategi recon, bukan user secara manual |
| **Output** | Graph database yang menunjukkan relasi antar-entitas keamanan |

---

## 3. Target Users

| Persona | Deskripsi |
|---------|-----------|
| **Penetration Tester** | Profesional yang melakukan pentest dan membutuhkan otomasi recon |
| **Bug Bounty Hunter** | Researcher yang perlu recon cepat dan menyeluruh pada target domain |
| **Red Team Operator** | Tim ofensif yang membutuhkan visualisasi attack surface |
| **Security Researcher** | Peneliti keamanan yang ingin mempelajari infrastruktur target |
| **SOC Analyst** | Analis yang membutuhkan threat intelligence dari domain tertentu |

---

## 4. Application Flow (Core Workflow)

```
┌─────────────────────────────────────────────────────────────┐
│                    🌿 IVY WORKFLOW                          │
└─────────────────────────────────────────────────────────────┘

  ┌───────────────────────┐
  │ 🧑 User Input Domain  │
  └───────────┬───────────┘
              │
              ▼
  ┌───────────────────────────────┐
  │ 🤖 AI Verification & Planning │
  └───────────────┬───────────────┘
                  │
                  ▼
  ┌─────────────────────────────────────┐
  │ 🔍 Phase 1: Passive Reconnaissance  │
  └─────────────────┬───────────────────┘
                    │
                    ▼
  ┌─────────────────────────────────────┐
  │ 🎯 Phase 2: Active Reconnaissance   │
  └─────────────────┬───────────────────┘
                    │
                    ▼
  ┌──────────────────────────────┐
  │ 📊 Phase 3: Graph Generation │
  └──────────────┬───────────────┘
                 │
                 ▼
         ┌───────────────┐
         │ 🧑 User Choice │
         └───┬───────┬───┘
             │       │
    Hanya Recon   Lanjut Payload
             │       │
             ▼       ▼
  ┌──────────┐   ┌────────────────────────┐
  │📋 Report │   │💉 Phase 4: Exploitation │
  └────┬─────┘   └───────────┬────────────┘
       │                     │
       │                     ▼
       │         ┌───────────────────────┐
       │         │📊 Update Graph+Report │
       │         └───────────┬───────────┘
       │                     │
       └──────────┬──────────┘
                  ▼
           ┌───────────┐
           │ 🌿 Done   │
           └───────────┘
```

### 4.1 Phase 0 — User Input

- User memasukkan **target domain** (contoh: `example.com`)
- User memilih **scope** dan **intensity level**:
  - 🟢 **Passive Only** — hanya passive recon, tidak menyentuh target langsung
  - 🟡 **Normal** — passive + active recon standar
  - 🔴 **Aggressive** — full recon + deep scanning
- User bisa memberikan **context tambahan** (e.g., "fokus pada API endpoints", "cari subdomain takeover")

### 4.2 Phase 1 — AI Verification & Planning

AI menerima input dan melakukan:

1. **Domain Validation** — memastikan domain valid dan resolvable
2. **Scope Analysis** — menganalisis scope dan membuat rencana recon
3. **Tool Selection** — AI memilih tools yang relevan berdasarkan target dan scope
4. **Execution Plan** — AI membuat urutan eksekusi tools yang optimal

> [!IMPORTANT]
> AI berperan sebagai **orchestrator** — ia menentukan tools mana yang dijalankan, dalam urutan apa, dan bagaimana mengkorelasikan hasilnya. Ini bukan sekadar wrapper script.

### 4.3 Phase 2 — Reconnaissance Execution

AI menjalankan tools secara terkoordinasi melalui MCP:

#### Passive Reconnaissance Tools

| Tool | Fungsi | Output |
|------|--------|--------|
| **WHOIS Lookup** | Informasi registrasi domain | Registrar, nameserver, dates |
| **DNS Enumeration** | Record DNS (A, AAAA, MX, TXT, NS, CNAME, SOA) | DNS records |
| **Amass (passive)** | Subdomain enumeration via passive sources | Subdomain list |
| **Subfinder** | Subdomain discovery dari banyak passive source | Subdomain list |
| **PDTM (ProjectDiscovery)** | Template-based scanning & management | Tool management |
| **theHarvester** | Email, nama, subdomain dari public sources | OSINT data |
| **Shodan/Censys API** | Informasi service yang terexpose | Service data |
| **crt.sh** | Certificate transparency logs | Subdomain dari SSL certs |
| **SecurityTrails** | Historical DNS & domain data | DNS history |
| **Wayback Machine** | Archived URLs dan endpoints | Historical URLs |
| **Google Dorking** | Search engine recon | Exposed files, endpoints |

#### Active Reconnaissance Tools

| Tool | Fungsi | Output |
|------|--------|--------|
| **RustScan** | Port scanning (ultra-fast) | Open ports |
| **Nmap** | Service & version detection | Service fingerprint |
| **httpx** | HTTP probing & technology detection | Live hosts, tech stack |
| **Nuclei** | Vulnerability scanning via templates | Vulnerabilities |
| **ffuf / Gobuster** | Directory & file brute-forcing | Hidden paths |
| **WhatWeb / Wappalyzer** | Technology fingerprinting | Tech stack |
| **Nikto** | Web server scanner | Server vulnerabilities |
| **SSL/TLS Analysis** | Certificate & cipher analysis | SSL issues |

### 4.4 Phase 3 — Graph Generation

Semua hasil dikumpulkan dan disimpan ke **Graph Database** (Neo4j):

#### Graph Nodes (Entities)

```
Domain → Subdomain → IP Address → Port → Service → Technology
                                    ↓
                              Vulnerability → CVE → Exploit
                                    ↓
                              Email → Person → Organization
```

| Node Type | Properties |
|-----------|------------|
| `Domain` | name, registrar, creation_date, expiry_date |
| `Subdomain` | name, source, first_seen |
| `IPAddress` | address, asn, geo_location, isp |
| `Port` | number, protocol, state |
| `Service` | name, version, banner |
| `Technology` | name, version, category |
| `Vulnerability` | id, severity, cvss, description |
| `Email` | address, source |
| `Certificate` | issuer, subject, expiry, san_list |
| `URL` | path, status_code, content_type |

#### Graph Edges (Relationships)

| Relationship | Description |
|-------------|-------------|
| `HAS_SUBDOMAIN` | Domain → Subdomain |
| `RESOLVES_TO` | Subdomain → IPAddress |
| `HAS_PORT` | IPAddress → Port |
| `RUNS_SERVICE` | Port → Service |
| `USES_TECHNOLOGY` | Service → Technology |
| `HAS_VULNERABILITY` | Service/Technology → Vulnerability |
| `ASSOCIATED_EMAIL` | Domain → Email |
| `HAS_CERTIFICATE` | Domain/Subdomain → Certificate |
| `DISCOVERED_URL` | Domain/Subdomain → URL |

### 4.5 Phase 4 — Exploitation (Optional)

> [!CAUTION]
> Phase ini **hanya dijalankan jika user secara eksplisit memilih "Ya"**. Ivy tidak akan pernah melakukan eksploitasi otomatis tanpa persetujuan user. Fitur ini dimaksudkan untuk **authorized penetration testing only**.

User memilih vulnerability dari graph, lalu AI menyarankan dan menjalankan payload:

| Tool | Fungsi | Target |
|------|--------|--------|
| **SQLMap** | SQL Injection automation | Database-backed endpoints |
| **XSStrike** | XSS detection & exploitation | Input fields, parameters |
| **Commix** | Command injection exploitation | OS command injection |
| **Metasploit (msfconsole)** | Multi-purpose exploitation framework | Various services |
| **Hydra** | Brute-force authentication | Login forms, services |
| **CrackMapExec** | Network exploitation suite | SMB, WinRM, LDAP |
| **Burp Suite (headless)** | Web application testing | HTTP traffic |

---

## 5. System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                      SYSTEM ARCHITECTURE                        │
└─────────────────────────────────────────────────────────────────┘

╔═══════════════════════ FRONTEND LAYER ════════════════════════╗
║  🖥️ Web Dashboard        📊 Graph Visualizer    💻 Terminal  ║
║  (React / Next.js)       (D3.js / Cytoscape)    (Live Stream)║
╚════════════╤══════════════════╤══════════════════╤════════════╝
             │                  │                  │
             ▼                  │                  │
╔═══════════════════ AI ORCHESTRATION LAYER ════════════════════╗
║  🔌 MCP Server ──► 🤖 AI Engine ──► 📋 Task Planner          ║
║  (Model Context      (LLM            (Execution              ║
║   Protocol)           Orchestrator)    Strategy)              ║
╚════════════════════════════╤══════════════════════════════════╝
                             │
                             ▼
╔═══════════════════ TOOL EXECUTION LAYER ══════════════════════╗
║                  ⚙️ Tool Runner (Sandboxed)                   ║
║          ┌──────────────┼──────────────┐                      ║
║          ▼              ▼              ▼                      ║
║  🔍 Passive Tools  🎯 Active Tools  💉 Exploit Tools          ║
║  (Amass,Subfinder) (RustScan,Nuclei)(SQLMap,XSStrike)         ║
╚═════╤═══════════════════╤══════════════════╤═════════════════╝
      │                   │                  │
      ▼                   ▼                  ▼
╔═══════════════════════ DATA LAYER ════════════════════════════╗
║  🗄️ Graph Database     ⚡ Cache Layer     📁 File Storage     ║
║  (Neo4j)               (Redis)           (Reports/Artifacts) ║
╚══════════════════════════════════════════════════════════════╝
```

### 5.1 Tech Stack

| Layer | Technology | Alasan |
|-------|-----------|--------|
| **Frontend** | Next.js + TypeScript | SSR, API routes, modern React |
| **Graph Visualization** | Cytoscape.js / D3.js | Interactive graph rendering |
| **AI/MCP** | MCP Protocol + LLM (Gemini/Claude) | AI orchestration via standardized protocol |
| **Backend** | Rust (Axum) | Memory safety, zero-cost abstractions, performa tinggi tanpa GC |
| **MCP SDK** | rust-mcp-sdk | Native Rust MCP implementation |
| **Graph Database** | Neo4j | Mature graph DB, Cypher query language |
| **Cache** | Redis | Session & result caching |
| **Task Queue** | Tokio (async tasks) | Native async runtime Rust, concurrent tool execution |
| **Container** | Docker | Sandboxed tool execution |
| **Realtime** | WebSocket (tokio-tungstenite) | Live streaming tool output, native async |

---

## 6. MCP Integration Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                  MCP INTEGRATION SEQUENCE                        │
└─────────────────────────────────────────────────────────────────┘

 🧑 User    🖥️ Frontend    🔌 MCP Server    🤖 AI Engine    ⚙️ Tool Runner    🗄️ Graph DB
   │              │               │               │               │               │
   │──input───►   │               │               │               │               │
   │ "example.com"│──request──►   │               │               │               │
   │              │               │──analyze──►   │               │               │
   │              │               │               │ Plan strategy │               │
   │              │               │   ◄──tools──  │               │               │
   │              │               │               │               │               │
   │              │         ┌─────────── FOR EACH TOOL ───────────┐               │
   │              │         │     │               │               │               │
   │              │         │     │──execute──►   │               │               │
   │              │         │     │               │  Run sandbox  │               │
   │              │         │     │   ◄──result── │               │               │
   │              │         │     │──result──►    │               │               │
   │              │         │     │               │──store──────────────────────►  │
   │  ◄──stream── │ ◄────── │     │               │               │               │
   │              │         └─────────────────────────────────────┘               │
   │              │               │               │               │               │
   │              │               │               │ Correlate all │               │
   │              │               │               │──finalize─────────────────►   │
   │  ◄──graph──  │ ◄──complete── │               │               │               │
   │              │               │               │               │               │
   │              │     ┌─── OPTIONAL: EXPLOITATION ───┐          │               │
   │──approve──►  │     │         │               │    │          │               │
   │              │──►  │         │──plan──►      │    │          │               │
   │              │     │         │               │──exploit──►   │               │
   │              │     │         │   ◄──result── │    │          │               │
   │              │     │         │──analyze──►   │    │          │               │
   │              │     │         │               │──update───────────────────►   │
   │  ◄──done──   │ ◄── │         │               │    │          │               │
   │              │     └──────────────────────────────┘          │               │
   │              │               │               │               │               │
```

### 6.1 MCP Tools Registry

Setiap security tool di-wrap sebagai **MCP Tool** dengan schema yang terstandarisasi:

```json
{
  "name": "ivy_amass",
  "description": "Subdomain enumeration using Amass",
  "inputSchema": {
    "type": "object",
    "properties": {
      "domain": { "type": "string", "description": "Target domain" },
      "mode": { "enum": ["passive", "active"], "default": "passive" },
      "timeout": { "type": "integer", "default": 300 }
    },
    "required": ["domain"]
  }
}
```

### 6.2 Daftar MCP Tools

| Tool Name | Category | Description |
|-----------|----------|-------------|
| `ivy_whois` | Passive | WHOIS domain lookup |
| `ivy_dns` | Passive | DNS record enumeration |
| `ivy_amass` | Passive/Active | Subdomain enumeration |
| `ivy_subfinder` | Passive | Subdomain discovery |
| `ivy_crtsh` | Passive | Certificate transparency |
| `ivy_theharvester` | Passive | Email & subdomain harvesting |
| `ivy_wayback` | Passive | Wayback Machine URL extraction |
| `ivy_shodan` | Passive | Shodan search |
| `ivy_rustscan` | Active | Fast port scanning |
| `ivy_nmap` | Active | Service detection |
| `ivy_httpx` | Active | HTTP probing |
| `ivy_nuclei` | Active | Vulnerability scanning |
| `ivy_ffuf` | Active | Directory brute-forcing |
| `ivy_whatweb` | Active | Technology fingerprinting |
| `ivy_nikto` | Active | Web server scanning |
| `ivy_sqlmap` | Exploit | SQL injection |
| `ivy_xsstrike` | Exploit | XSS exploitation |
| `ivy_commix` | Exploit | Command injection |
| `ivy_hydra` | Exploit | Brute-force attack |

---

## 7. Fitur yang Perlu Ditambahkan (Gap Analysis)

> [!NOTE]
> Berikut fitur-fitur yang **belum disebutkan** di alur awal tapi sangat penting untuk produk yang matang:

### 7.1 🔐 Scope & Legal Boundary Control

- **Scope Definition** — user harus bisa mendefinisikan scope (in-scope / out-of-scope) agar tools tidak menyerang target yang tidak diizinkan
- **Legal Disclaimer** — wajib ada persetujuan legal sebelum menjalankan active recon / exploitation
- **Rate Limiting** — kontrol kecepatan scanning agar tidak terdeteksi/merusak target

### 7.2 📊 Reporting & Export

- **Auto-generated Report** — PDF/HTML report profesional dengan executive summary
- **Export Graph** — export graph data ke format standar (GraphML, JSON, CSV)
- **Finding Classification** — severity rating (Critical/High/Medium/Low/Info) otomatis

### 7.3 🔄 Continuous Monitoring

- **Scheduled Scans** — jadwalkan recon berkala untuk mendeteksi perubahan
- **Change Detection** — notifikasi jika ada subdomain baru, port terbuka baru, atau vulnerability baru
- **Attack Surface Diff** — bandingkan graph antar-waktu

### 7.4 🧠 AI-Powered Analysis

- **Threat Scoring** — AI memberikan risk score berdasarkan kombinasi temuan
- **Attack Path Analysis** — AI menyarankan possible attack chain dari graph
- **False Positive Filtering** — AI memfilter false positive dari hasil scanner
- **Natural Language Query** — user bisa bertanya ke graph pakai bahasa natural ("Tunjukkan semua subdomain yang pakai Apache versi lama")

### 7.5 🛡️ Defensive Features

- **Blue Team Mode** — mode defensif untuk SOC analyst yang fokus pada hardening recommendations
- **Compliance Check** — cek apakah konfigurasi sesuai standar (CIS, OWASP, NIST)
- **Remediation Suggestions** — AI memberikan saran perbaikan untuk setiap temuan

### 7.6 👥 Collaboration

- **Multi-user Support** — multiple user bisa bekerja pada project yang sama
- **Shared Graph** — graph bisa di-share dan di-annotate oleh tim
- **Audit Log** — log semua aktivitas user untuk compliance

### 7.7 🔌 Integration

- **Jira/Ticketing Integration** — otomatis buat ticket dari temuan vulnerability
- **Slack/Discord Notification** — notifikasi real-time ke messaging platform
- **API Access** — REST API untuk integrasi dengan pipeline CI/CD

---

## 8. User Interface Design

### 8.1 Dashboard Overview

```
┌──────────────────────────────────────────────────────────────────┐
│  🌿 Ivy                                    [Profile] [Settings] │
├──────────┬───────────────────────────────────────────────────────┤
│          │                                                       │
│ Projects │         Interactive Graph Visualization               │
│          │                                                       │
│ ┌──────┐ │    ┌──┐      ┌──┐      ┌──┐                         │
│ │Proj 1│ │    │ D├──────►│SD├──────►│IP│                         │
│ └──────┘ │    └──┘      └─┬┘      └─┬┘                         │
│ ┌──────┐ │               │          │                           │
│ │Proj 2│ │             ┌─▼─┐     ┌──▼──┐                       │
│ └──────┘ │             │URL│     │PORT │                        │
│          │             └───┘     └──┬──┘                        │
│ ─────── │                        ┌─▼──┐                        │
│ Scans    │                        │VULN│                        │
│ Tools    │                        └────┘                        │
│ Reports  │                                                       │
│ Settings │                                                       │
│          ├───────────────────────────────────────────────────────┤
│          │  AI Chat Interface                                    │
│          │  ┌───────────────────────────────────────────────┐    │
│          │  │ 🤖 Found 23 subdomains, 5 have open ports...  │    │
│          │  │ 🤖 Vulnerability CVE-2024-xxxx detected on... │    │
│          │  │ 🧑 Scan port 443 on all subdomains            │    │
│          │  │ 🤖 Scanning... 15/23 complete                 │    │
│          │  └───────────────────────────────────────────────┘    │
│          │  [Type a message or command...]            [Send]     │
└──────────┴───────────────────────────────────────────────────────┘
```

### 8.2 Key UI Components

| Component | Deskripsi |
|-----------|-----------|
| **Graph Canvas** | Area utama menampilkan interactive graph (zoom, pan, filter by node type) |
| **AI Chat Panel** | Interface chat untuk berkomunikasi dengan AI — bisa command atau tanya |
| **Tool Output Stream** | Real-time terminal output dari tools yang sedang berjalan |
| **Node Inspector** | Panel detail saat klik node di graph (properties, related nodes) |
| **Scan Progress** | Progress bar dan status setiap tool yang sedang running |
| **Finding Table** | Tabel sortable/filterable semua temuan vulnerability |

---

## 9. Security & Ethical Considerations

> [!WARNING]
> Ivy adalah tool yang powerful dan **harus digunakan secara bertanggung jawab**. Berikut safeguard yang wajib diimplementasikan:

### 9.1 Mandatory Safeguards

| Safeguard | Implementasi |
|-----------|-------------|
| **Authorization Check** | User wajib konfirmasi bahwa mereka memiliki izin untuk scan target |
| **Scope Enforcement** | Tools tidak boleh keluar dari scope yang didefinisikan user |
| **Exploitation Consent** | Double confirmation sebelum menjalankan exploit tools |
| **Logging** | Semua aktivitas di-log dengan timestamp untuk audit trail |
| **Sandboxing** | Tools berjalan di Docker container yang terisolasi |
| **Kill Switch** | User bisa menghentikan semua tools sekaligus kapan saja |
| **Data Retention** | Kebijakan retensi data — auto-delete setelah X hari |

### 9.2 Legal Disclaimer

Setiap kali user memulai project baru, wajib muncul:

```
⚠️ LEGAL DISCLAIMER

Ivy is designed for authorized security testing ONLY.
You MUST have explicit written permission from the target owner
before running any scans or exploits.

Unauthorized access to computer systems is illegal under:
- UU ITE (Indonesia)
- CFAA (USA)
- Computer Misuse Act (UK)

By proceeding, you confirm that you have proper authorization.

[ ] I have authorization to test this target
[Cancel] [Proceed]
```

---

## 10. Non-Functional Requirements

| Requirement | Target | Metric |
|------------|--------|--------|
| **Performance** | Graph rendering < 2 detik untuk 10.000 nodes | Time to interactive |
| **Scalability** | Support hingga 100 concurrent scans | Load test |
| **Availability** | 99.5% uptime (self-hosted) | Uptime monitoring |
| **Security** | Zero trust — semua tools di sandbox | Container isolation |
| **Latency** | AI response < 3 detik | Response time |
| **Storage** | Efisien — incremental graph updates | DB size growth |
| **Compatibility** | Chrome, Firefox, Edge (latest 2 versions) | Cross-browser test |

---

## 11. Development Roadmap

### Phase 1 — MVP (Month 1-2)

- [ ] Core MCP server setup
- [ ] AI orchestration engine (basic planning)
- [ ] 5 passive recon tools (WHOIS, DNS, Amass, Subfinder, crt.sh)
- [ ] 2 active recon tools (RustScan, httpx)
- [ ] Neo4j graph database integration
- [ ] Basic graph visualization (Cytoscape.js)
- [ ] Web dashboard (Next.js) with AI chat
- [ ] Legal disclaimer & scope control

### Phase 2 — Core Features (Month 3-4)

- [ ] Full passive tool suite (theHarvester, Shodan, Wayback, etc.)
- [ ] Full active tool suite (Nmap, Nuclei, ffuf, WhatWeb, Nikto)
- [ ] Advanced graph visualization (filtering, search, clustering)
- [ ] AI correlation engine — cross-tool finding analysis
- [ ] PDF/HTML report generation
- [ ] Real-time WebSocket streaming
- [ ] Node inspector panel

### Phase 3 — Exploitation & Intelligence (Month 5-6)

- [ ] Exploitation module (SQLMap, XSStrike, Commix)
- [ ] AI attack path analysis
- [ ] Threat scoring system
- [ ] False positive filtering
- [ ] Natural language graph queries
- [ ] Blue team / defensive mode

### Phase 4 — Enterprise Features (Month 7+)

- [ ] Multi-user & team collaboration
- [ ] Scheduled/continuous monitoring
- [ ] Change detection & diffing
- [ ] Jira / Slack integration
- [ ] REST API for CI/CD pipeline
- [ ] Compliance checking (CIS, OWASP)
- [ ] On-premise enterprise deployment

---

## 12. Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| **Recon Coverage** | > 90% subdomain discovery vs manual | Benchmark test |
| **Time Savings** | 70% faster than manual multi-tool workflow | Time comparison |
| **Graph Accuracy** | < 5% false positive rate setelah AI filtering | Precision/Recall |
| **User Satisfaction** | > 4.5/5 rating dari pentesters | User survey |
| **Adoption** | 500+ active users dalam 6 bulan pertama | Analytics |

---

## 13. Risks & Mitigations

| Risk | Impact | Likelihood | Mitigation |
|------|--------|-----------|------------|
| Tool berjalan di luar scope | 🔴 High | Medium | Scope enforcement + proxy filtering |
| AI membuat keputusan berbahaya | 🔴 High | Low | Human-in-the-loop untuk semua exploit |
| Rate limiting oleh target | 🟡 Medium | High | Configurable delay + jitter |
| False positives membanjiri graph | 🟡 Medium | Medium | AI-powered filtering + manual verify |
| Legal issues dari misuse | 🔴 High | Medium | Disclaimer + logging + terms of service |
| Performance bottleneck pada graph besar | 🟡 Medium | Medium | Graph pagination + lazy loading |

---

## 14. Keputusan Arsitektur (Finalized)

> [!NOTE]
> Keputusan berikut sudah di-finalize:

| # | Keputusan | Jawaban | Alasan |
|---|-----------|---------|--------|
| 1 | **LLM Provider** | **Provider-Agnostic** (Gemini, Claude, Ollama, dll.) | Ivy mendukung semua LLM provider melalui abstraction layer. User bebas memilih provider sesuai kebutuhan — cloud (Gemini/Claude) untuk kemampuan terbaik, atau local (Ollama) untuk privasi maksimal |
| 2 | **Deployment Model** | **Self-Hosted** | User menjalankan Ivy di infrastruktur sendiri. Memberikan kontrol penuh atas data, privasi, dan keamanan. Cocok untuk pentest environment yang butuh isolasi |
| 3 | **Graph Database** | **Neo4j** (Community Edition) | Paling cocok untuk OSINT karena: (1) paling mature untuk relationship-heavy data, (2) Cypher query intuitif untuk security queries seperti "MATCH path dari domain ke vulnerability", (3) Community Edition gratis & open-source compatible, (4) ekosistem plugin & visualization terbaik |
| 4 | **Pricing Model** | **Open Source** (Free) | Project ini akan dirilis sebagai open-source. Komunitas bisa berkontribusi, audit kode, dan memperluas tools |
| 5 | **Tool Execution** | **Local Execution** | Tools dijalankan di mesin lokal user (dalam Docker sandbox). Tidak ada data yang dikirim ke cloud pihak ketiga. Sesuai dengan model self-hosted |
| 6 | **Exploit Module** | **Plugin Terpisah** | Modul eksploitasi (SQLMap, XSStrike, dll.) di-ship sebagai plugin terpisah yang harus di-install secara eksplisit. Ini memisahkan recon (legal) dari exploitation (butuh izin), dan mengurangi risiko legal |

### Diagram Keputusan Arsitektur

```
┌─────────────────────────────────────────────────────────────┐
│                    🌿 IVY ARCHITECTURE DECISIONS             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  LLM Layer (Provider-Agnostic)                              │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │ Gemini   │  │ Claude   │  │ Ollama   │  │ Others   │   │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘   │
│       └──────────────┴──────────────┴──────────────┘         │
│                          │                                   │
│                          ▼                                   │
│              ┌───────────────────────┐                       │
│              │ LLM Abstraction Layer │                       │
│              └───────────┬───────────┘                       │
│                          │                                   │
│                          ▼                                   │
│  Deployment: Self-Hosted (Docker Compose)                   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  ┌─────────┐  ┌──────────┐  ┌──────────────────┐    │   │
│  │  │ Ivy Core│  │ Neo4j    │  │ Redis            │    │   │
│  │  │ (Rust)  │  │ (Graph)  │  │ (Cache)          │    │   │
│  │  └─────────┘  └──────────┘  └──────────────────┘    │   │
│  └──────────────────────────────────────────────────────┘   │
│                          │                                   │
│  Execution: Local (Sandboxed Docker)                        │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  Tools berjalan di container terisolasi               │   │
│  └──────────────────────────────────────────────────────┘   │
│                          │                                   │
│  Plugins: Modular Architecture                              │
│  ┌──────────────────┐  ┌───────────────────────────────┐   │
│  │ 📦 Core (Recon)  │  │ 🔌 Plugin: Exploit Module     │   │
│  │ (built-in)       │  │ (install terpisah, opt-in)    │   │
│  └──────────────────┘  └───────────────────────────────┘   │
│                                                             │
│  License: Open Source                                       │
└─────────────────────────────────────────────────────────────┘
```

---

*Document Version: 1.1*
*Last Updated: 2026-07-12*
*Author: Ivy Development Team*