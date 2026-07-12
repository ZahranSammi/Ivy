---
title: "[Dev B] Backend: Setup Axum HTTP Server & PostgreSQL CRUD"
labels: ["backend", "enhancement"]
assignees: [] # Assign to Developer 2
---

### Objective
Membangun fondasi HTTP Server menggunakan **Axum** (`ivy-server`) dan mengimplementasikan operasi database CRUD dasar menggunakan **PostgreSQL & SQLx** (`ivy-db`).

### Tasks
- [ ] Inisialisasi routing dasar Axum di `backend/ivy-server/src/routes/`.
- [ ] Implementasi middleware:
  - CORS configuration.
  - JWT Authentication (stub atau integrasi dengan `argon2`).
- [ ] Setup koneksi database Postgres menggunakan `sqlx::PgPool` di `backend/ivy-db/src/postgres/pool.rs`.
- [ ] Buat endpoints CRUD sederhana:
  - `POST /projects` (Create project baru)
  - `GET /projects` (List semua project)
  - `GET /projects/:id` (Get detail project)

### Acceptance Criteria
- Server dapat berjalan di `localhost:3001` tanpa error.
- Hit ke endpoint `/projects` mengembalikan struktur JSON yang valid (atau array kosong jika belum ada data).
- Dependensi SQLx bisa terhubung ke database `ivy` di container Postgres lokal (via `docker-compose.dev.yml`).

### References
- [Ivy PRD](../Ivy_prd.md)
- [Ivy SRS](../Ivy_srs.md)
- Target folder: `backend/ivy-server/` dan `backend/ivy-db/src/postgres/`
