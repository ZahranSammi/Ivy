---
title: "[Dev A] Database: Design Neo4j Graph Queries"
labels: ["backend", "database"]
assignees: [] # Assign to Developer 1 (Zahra)
---

### Objective
Menulis query `Cypher` untuk Neo4j di dalam `ivy-db/src/neo4j/` guna menghubungkan (*correlate*) data hasil *reconnaissance* dari berbagai alat (tools) ke dalam struktur graph.

### Tasks
- [ ] Buat query Cypher untuk meng-*insert* Node baru (Domain, IP, Port).
- [ ] Buat query Cypher untuk meng-*insert* Edge / Relasi (contoh: `(Domain)-[:RESOLVES_TO]->(IP)`).
- [ ] Buat fungsi pencarian rute eksploitasi (contoh: path terpendek dari Domain ke CVE).

### Acceptance Criteria
- Query dapat berjalan di Neo4j tanpa error sintaks.
- Dapat merelasikan data subdomain (misal dari Amass) dengan IP (misal dari DNS).
