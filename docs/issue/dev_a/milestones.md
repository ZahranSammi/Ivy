# 🎯 Dev A (Zahra): Milestones & FR Tracker

Dokumen ini memetakan ke-13 Functional Requirements (FR) yang menjadi tanggung jawab **Dev A** ke dalam urutan pengerjaan (Milestones) yang logis. Setiap milestone harus selesai dan bisa di-test (setidaknya via unit test) sebelum lanjut ke milestone berikutnya.

---

## 🏁 Milestone 1: Core Foundations & Safety
**Fokus**: Membangun aturan main, struktur data, dan pengaman (safety guard) di `ivy-core`.
**Status**: 🔴 Not Started

| FR ID | Deskripsi FR | Issue Terkait | Status |
|-------|--------------|---------------|--------|
| **FR-006** | Domain Validation (DNS Resolution) | `A1_core_models.md` | `TODO` |
| **FR-005** | Scope Configuration & Enforcement | `A1_core_models.md` | `TODO` |
| **FR-003** | Legal Disclaimer Consent (Core Logic) | `A1_core_models.md` | `TODO` |

*Goal M1*: `ivy-core` memiliki logika validasi yang kuat. Sistem bisa menolak IP di luar scope dan memastikan consent tervalidasi di level struct/domain model.

---

## 🏁 Milestone 2: Graph Database Architecture
**Fokus**: Membangun lapisan penyimpanan data di `ivy-db/neo4j` menggunakan Cypher.
**Status**: 🔴 Not Started

| FR ID | Deskripsi FR | Issue Terkait | Status |
|-------|--------------|---------------|--------|
| **FR-015** | Graph Data Storage (Schema & Adapter) | `A2_neo4j_adapter.md` | `TODO` |
| **FR-018** | Graph Query (Raw Cypher query func) | `A2_neo4j_adapter.md` | `TODO` |

*Goal M2*: Kamu punya modul Rust yang bisa melakukan `MERGE` node (Domain, IP, Subdomain) dan `CREATE` relasi secara deduplikatif ke Neo4j.

---

## 🏁 Milestone 3: The AI Brain (Orchestrator)
**Fokus**: Menghidupkan AI menggunakan MCP di `ivy-mcp`.
**Status**: 🔴 Not Started

| FR ID | Deskripsi FR | Issue Terkait | Status |
|-------|--------------|---------------|--------|
| **FR-007** | LLM Provider Config (Gemini/Claude) | `A3_ai_orchestrator.md` | `TODO` |
| **FR-008** | AI Recon Planning (Prompt & JSON Output) | `A3_ai_orchestrator.md` | `TODO` |
| **FR-009** | MCP Tool Execution (Mock protocol loop) | `A3_ai_orchestrator.md` | `TODO` |
| **FR-010** | AI Result Correlation (Graph mapping) | `A3_ai_orchestrator.md` | `TODO` |

*Goal M3*: AI bisa menerima input domain, merencanakan eksekusi, memanggil mock executor, mengkorelasikan hasil, dan menyimpannya ke layer Neo4j (gabungan M2 dan M3).

---

## 🏁 Milestone 4: Polish & Advanced Features
**Fokus**: Fitur pelengkap dan sistem darurat.
**Status**: 🔴 Not Started

| FR ID | Deskripsi FR | Issue Terkait | Status |
|-------|--------------|---------------|--------|
| **FR-021** | Finding Classification (Auto-scoring) | `A1_core_models.md` | `TODO` |
| **FR-024** | Kill Switch (Cancellation Token/Signal) | `A3_ai_orchestrator.md` | `TODO` |
| **FR-011** | Natural Language Interaction (Chat to Graph) | `TBD` | `TODO` |

*Goal M4*: AI bisa diajak ngobrol pakai bahasa manusia, penemuan (findings) diberi label *Critical/High*, dan proses AI bisa dibatalkan secara paksa di tengah jalan.
