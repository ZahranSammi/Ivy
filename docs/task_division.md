# 🚀 Ivy: Development Task Division (2 Developers)

Karena ada **2 Developer** yang bekerja di proyek ini (dan masing-masing dibantu oleh AI/Gemini), kita membagi repository monorepo ini menjadi dua pilar utama berdasarkan **Architectural Boundary (Otak vs Otot)**. 

Pendekatan ini memastikan beban kerja seimbang, mencegah konflik Git, dan memastikan Developer 1 (Lead) tetap memegang kendali pada komponen yang paling kritis dan kompleks. Pembagian ini memetakan seluruh **27 Functional Requirements (FR)** dari dokumen `Ivy_srs.md`.

---

## 🧠 Developer 1: Lead Engineer / Core & AI (Zahra)

Fokus pada "Otak" aplikasi: arsitektur utama, kecerdasan buatan, database graf, keamanan, dan aturan bisnis OSINT. Kode yang disentuh murni Rust (`ivy-core`, `ivy-mcp`, `ivy-db/neo4j`).

**Total Beban: 12 FR**

### 1. AI Orchestration & MCP (`ivy-mcp`)
Ini adalah "otak" dari Ivy. AI yang menganalisis hasil dari satu tool, lalu memutuskan tool apa yang harus dieksekusi selanjutnya.
- **FR-007**: LLM Provider Configuration
- **FR-008**: AI Recon Planning
- **FR-009**: MCP Tool Execution (Logika pemanggilan via protokol)
- **FR-010**: AI Result Correlation
- **FR-011**: Natural Language Interaction

### 2. Graph Database Architecture (`ivy-db/neo4j`)
Mendesain query Cypher yang kompleks untuk menghubungkan data recon.
- **FR-015**: Graph Data Storage (Neo4j Adapter & Schema)
- **FR-018**: Graph Query (Cypher & NLP to Cypher)

### 3. Core Business Logic & Safety (`ivy-core`)
Jantung aplikasi yang memastikan keamanan dan akurasi logika OSINT.
- **FR-003**: Legal Disclaimer Consent (Logika validasi gatekeeper)
- **FR-005**: Scope Configuration & Enforcement (Safety-Critical filter)
- **FR-006**: Domain Validation (DNS Resolution)
- **FR-021**: Finding Classification (Auto-scoring & Severity)
- **FR-024**: Kill Switch (Logika core pembatalan token/thread)

---

## 🦾 Developer 2: Platform & Infrastructure Engineer (Naufal)

Fokus pada "Otot" dan "Wajah" aplikasi: infrastruktur web, API server, UI/UX, dan pembungkus alat (Docker wrappers) agar sistem bisa dioperasikan. Kode yang disentuh meliputi Next.js (`frontend/`), Axum (`ivy-server`), dan Docker (`ivy-docker`, `tools/`).

**Total Beban: 15 FR**

### 1. HTTP Server & Integrasi Backend (`ivy-server` & `ivy-db/postgres`)
Server yang melayani HTTP REST API dan real-time WebSocket.
- **FR-001**: User Authentication
- **FR-002**: Project Management CRUD
- **FR-004**: Domain Input (API Handler)
- **FR-022**: Real-time Tool Output Streaming (WebSocket implementation)
- **FR-025**: LLM Provider Settings (API Handler)
- **FR-026**: Tool Configuration (API Handler)
- **FR-027**: Audit Log (Middleware)
- **FR-019**: Graph Export (Format to JSON/CSV/PDF)
- **FR-020**: Auto-Generated Report

### 2. Frontend Dashboard (`frontend/`)
Membangun UI/UX menggunakan Next.js dan TailwindCSS.
- **FR-016**: Graph Visualization (Cytoscape.js canvas)
- **FR-017**: Node Inspector (Side panel UI)
- **FR-023**: Scan Progress Tracking (UI progress bars)

### 3. Docker & Tool Sandboxing (`ivy-docker` & `tools/`)
Menulis kode Rust untuk men-spawn dan mematikan Docker container secara dinamis lewat API (`bollard`) serta membungkus command OSINT.
- **FR-012**: Tool Registry (Manajemen image dan schema)
- **FR-013**: Tool Sandboxing (Docker execution & egress filter)
- **FR-014**: Plugin System (Sistem instalasi exploit tools)

---

## 🔄 Cara Kolaborasi Git (V-Shape Workflow)

Karena arsitektur Rust kita sudah dipisah menjadi **Crates (Workspace)**, konflik kode saat melakukan `git merge` akan sangat minim. Pekerjaan dapat dilakukan secara paralel (Dev 1 membangun Otak, Dev 2 membangun Platform), lalu bertemu di tahap integrasi.

Area kerja sangat terisolasi:
- **Area Dev 1 (Zahra)**: `backend/ivy-core/`, `backend/ivy-mcp/`, `backend/ivy-db/src/neo4j/`
- **Area Dev 2 (Naufal)**: `backend/ivy-server/`, `frontend/`, `backend/ivy-docker/`, `tools/`
