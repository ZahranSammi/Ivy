---
title: "[Dev B] Frontend: Build Next.js Dashboard UI"
labels: ["frontend", "ui"]
assignees: [] # Assign to Developer 2
---

### Objective
Membangun User Interface utama menggunakan **Next.js (App Router)** dan **TailwindCSS**, dengan visualisasi data dasar dan state management.

### Tasks
- [ ] Buat layout utama dashboard (Sidebar navigation & Header) di `frontend/src/app/layout.tsx`.
- [ ] Buat halaman Home (`/`) yang berisi summary/statistik (jumlah node, vulnerabilities).
- [ ] Buat halaman Project Detail (`/project/[id]`) dengan tabulasi (Graph View, Findings Table, Logs).
- [ ] Integrasikan library Visualisasi Graph (contoh: **Cytoscape.js** atau **vis.js**) pada komponen `frontend/src/components/graph/GraphCanvas.tsx`.
- [ ] (Optional) Buat stub tabel "Findings" yang bersifat sortable/filterable.

### Acceptance Criteria
- UI responsif dan sesuai dengan prinsip *modern web design* (seperti glassmorphism/dark mode jika disepakati).
- Navbar dan Sidebar berfungsi (minimal navigasi ke home dan project example).
- Terdapat kanvas graph yang bisa me-render *dummy nodes/edges*.

### References
- [Ivy Project Structure](../Ivy_project_structure.md)
- Target folder: `frontend/`
