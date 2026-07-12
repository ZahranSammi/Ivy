---
title: "[Dev A] Core: Define Domain Models & Traits"
labels: ["backend", "core"]
assignees: [] # Assign to Developer 1 (Zahra)
---

### Objective
Membangun fondasi logika bisnis utama (Domain-Driven Design) di `ivy-core` untuk memastikan struktur data OSINT dan antarmuka tool valid.

### Tasks
- [ ] Buat *struct* di `backend/ivy-core/src/domain/project.rs` (Project, Target).
- [ ] Buat *struct* struktur graph (OSINT) di `backend/ivy-core/src/domain/graph_node.rs` (IP, Domain, Port, CVE).
- [ ] Definisikan trait (abstraksi antarmuka) untuk `Repository` dan `ToolExecutor` di `backend/ivy-core/src/traits/`.

### Acceptance Criteria
- Semua tipe data OSINT terdefinisi dengan jelas dan di-derive `Serialize/Deserialize` dari Serde.
- `ivy-core` dapat dicompile tanpa dependensi eksternal (murni logika bisnis).
