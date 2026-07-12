# 🎯 Dev B (Naufal): Milestones & FR Tracker

Dokumen ini memetakan ke-15 Functional Requirements (FR) yang menjadi tanggung jawab **Dev B** (Platform & Infrastructure) ke dalam urutan pengerjaan (Milestones) yang logis. Setiap milestone membangun infrastruktur dari dasar (API) hingga UI interaktif.

---

## 🏁 Milestone 1: Server Foundations & API
**Fokus**: Membangun backend HTTP server dengan Axum dan PostgreSQL, serta API dasar untuk manajemen project.
**Status**: 🔴 Not Started

| FR ID | Deskripsi FR | Issue Terkait | Status |
|-------|--------------|---------------|--------|
| **FR-001** | User Authentication (JWT) | `B1_axum_server.md` | `TODO` |
| **FR-002** | Project Management CRUD | `B1_axum_server.md` | `TODO` |
| **FR-004** | Domain Input API (Menerima target) | `B1_axum_server.md` | `TODO` |
| **FR-027** | Audit Log (Middleware Logging) | `B1_axum_server.md` | `TODO` |

*Goal M1*: Server Axum sudah berjalan di port 3001, nyambung ke database Postgres, dan bisa melayani request CRUD untuk project melalui Postman/cURL.

---

## 🏁 Milestone 2: Docker Sandbox Engine (Otot)
**Fokus**: Membangun sistem yang bisa mengeksekusi *tools* keamanan di dalam container Docker.
**Status**: 🔴 Not Started

| FR ID | Deskripsi FR | Issue Terkait | Status |
|-------|--------------|---------------|--------|
| **FR-012** | Tool Registry (Daftar image & schema) | `B3_docker_sandbox.md` | `TODO` |
| **FR-013** | Tool Sandboxing (Eksekusi Docker via API) | `B3_docker_sandbox.md` | `TODO` |

*Goal M2*: Kode Rust di `ivy-docker` bisa men-spawn container (misal: jalankan `nmap` atau `whois`), menangkap outputnya (stdout), lalu mematikan containernya dengan aman.

---

## 🏁 Milestone 3: Dashboard & Graph Visualization (Wajah)
**Fokus**: Membangun tampilan utama (Next.js) dan kanvas interaktif.
**Status**: 🔴 Not Started

| FR ID | Deskripsi FR | Issue Terkait | Status |
|-------|--------------|---------------|--------|
| **FR-016** | Graph Visualization (Canvas Cytoscape.js) | `B2_nextjs_ui.md` | `TODO` |
| **FR-017** | Node Inspector (Side panel properti) | `B2_nextjs_ui.md` | `TODO` |

*Goal M3*: Frontend Next.js sudah jadi. Saat user membuka halaman detail project, muncul tampilan canvas graf (walaupun datanya masih dari endpoint API mock/statis).

---

## 🏁 Milestone 4: Real-time UI & Settings
**Fokus**: Menghubungkan WebSocket untuk data real-time, dan menambah menu konfigurasi.
**Status**: 🔴 Not Started

| FR ID | Deskripsi FR | Issue Terkait | Status |
|-------|--------------|---------------|--------|
| **FR-022** | Real-time Tool Output Streaming (WS) | `TBD` | `TODO` |
| **FR-023** | Scan Progress Tracking (Progress Bar UI) | `TBD` | `TODO` |
| **FR-025** | LLM Provider Settings (API + Form UI) | `TBD` | `TODO` |
| **FR-026** | Tool Configuration (API + Form UI) | `TBD` | `TODO` |

*Goal M4*: Di UI terlihat live-terminal yang menampilkan output tool yang sedang berjalan secara *real-time*. Progress bar bergerak naik, dan user bisa menyimpan API Key Gemini/Claude di halaman Settings.

---

## 🏁 Milestone 5: Output & Extensibility (Polish)
**Fokus**: Fitur pelaporan, export data, dan sistem plugin untuk eksploitasi.
**Status**: 🔴 Not Started

| FR ID | Deskripsi FR | Issue Terkait | Status |
|-------|--------------|---------------|--------|
| **FR-019** | Graph Export (JSON, CSV) | `TBD` | `TODO` |
| **FR-020** | Auto-Generated PDF/HTML Report | `TBD` | `TODO` |
| **FR-014** | Plugin System (Sistem modul exploit) | `TBD` | `TODO` |

*Goal M5*: Sistem bisa mencetak PDF laporan penemuan otomatis. Ada arsitektur standar (plugin) jika tim red team ingin memasukkan script eksploitasi tambahan (SQLMap, dll) di masa depan.
