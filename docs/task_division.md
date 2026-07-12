# 🚀 Ivy: Development Task Division (2 Developers)

Karena ada **2 Developer** yang bekerja di proyek ini (dan masing-masing dibantu oleh AI/Gemini), kita membagi repository monorepo ini menjadi dua pilar utama. 

**Developer 1 (Zahra/Lead)** memegang komponen yang **paling critical** dan menjadi *core-brain* (Otak) dari platform ini. **Developer 2 (Co-Developer)** akan membangun *platform & infrastructure* (Otot/Tubuh) agar otak tersebut bisa berjalan dan dilihat oleh user.

---

## 👨‍💻 Developer 1: Lead Engineer / Core & AI (Zahra)

Fokus pada arsitektur utama, kecerdasan buatan, keamanan, dan struktur data OSINT. 

### 1. AI & MCP Orchestrator (`ivy-mcp`)
Ini adalah "otak" dari Ivy.
- Mengembangkan logic LLM Provider (koneksi ke Gemini, Claude, Ollama).
- **Planner & Correlator**: Bagaimana AI menganalisis hasil dari satu tool, lalu memutuskan tool apa yang harus dieksekusi selanjutnya.
- Mendefinisikan schema JSON (MCP) untuk input/output semua tools.

### 2. Core Business Logic (`ivy-core`)
Jantung dari aplikasi.
- Mendefinisikan Domain Models (struktur `Project`, `Finding`, `GraphNode`, `GraphEdge`).
- Mendefinisikan Trait (interface) utama untuk eksekusi scan.
- Memastikan flow eksekusi OSINT aman dan akurat.

### 3. Graph Database Architecture (`ivy-db/neo4j`)
- Mendesain **Cypher queries** yang kompleks untuk menghubungkan data recon.
- Memastikan bagaimana sebuah IP, Domain, Port, dan Vulnerability berelasi dan dikorelasikan di dalam Graph Database.

---

## 👨‍💻 Developer 2: Platform & Infrastructure Engineer

Fokus pada infrastruktur web, API, koneksi, UI, dan pembungkus alat (Docker wrappers) agar sistem bisa digunakan dengan mudah.

### 1. HTTP Server & Database Postgres (`ivy-server` & `ivy-db/postgres`)
- Setup Axum routing, controller, dan middleware (Auth JWT, Rate limiting, CORS).
- CRUD aplikasi biasa (Projects, Users) menggunakan Postgres & SQLx.
- **WebSocket Server**: Setup streaming data real-time ke frontend saat scan berjalan.

### 2. Frontend Dashboard (Next.js)
- Membangun UI/UX menggunakan Next.js dan TailwindCSS.
- Integrasi Visualisasi Graph (menggunakan Cytoscape.js atau vis.js) untuk menampilkan node hasil scan.
- Integrasi API REST dan WebSocket client.

### 3. Docker & Tool Wrappers (`ivy-docker` & `tools/`)
- Menulis kode Rust untuk men-spawn dan mematikan Docker container secara dinamis lewat API (`bollard`).
- Membuat Dockerfile dan Bash parsing script (entrypoint) untuk 15+ tool OSINT (Amass, Subfinder, Nmap, dll) agar output terminal berubah menjadi format JSON terstruktur yang bisa dibaca AI.

---

## 🔄 Cara Kolaborasi Git

Karena arsitektur Rust kita sudah dipisah menjadi **Crates (Workspace)**, konflik kode saat melakukan `git merge` akan sangat minim. Area kerja sangat terisolasi:

- **Area Dev 1 (Otak)**: `backend/ivy-core/`, `backend/ivy-mcp/`, `backend/ivy-db/src/neo4j/`
- **Area Dev 2 (Otot)**: `backend/ivy-server/`, `frontend/`, `backend/ivy-docker/`, `tools/`
