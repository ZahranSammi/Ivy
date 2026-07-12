---
title: "[Dev B] Infrastructure: Docker Sandboxing & Tool Wrappers"
labels: ["infrastructure", "docker"]
assignees: [] # Assign to Developer 2
---

### Objective
Mengimplementasikan engine eksekusi yang menggunakan **Bollard** (Docker API untuk Rust) untuk menjalankan tool OSINT di dalam container terisolasi, serta memparsing output terminalnya menjadi JSON (MCP schema).

### Tasks
- [ ] Implementasi fungsi `run_container` di `backend/ivy-docker/src/executor.rs` menggunakan crate `bollard` untuk men-spawn container dari image spesifik.
- [ ] Implementasikan fungsi streaming `stdout` dan `stderr` container ke Rust channel/streams (`backend/ivy-docker/src/output_stream.rs`).
- [ ] Buat Dockerfile untuk base image di `tools/_base/Dockerfile`.
- [ ] Buat wrapper script (`entrypoint.sh`) untuk salah satu tool pasif (misal: **Amass**) yang:
  - Membaca argumen JSON dari `stdin`.
  - Menjalankan perintah bash tool tersebut.
  - Memparsing hasilnya menjadi format JSON terstruktur ke `stdout`.

### Acceptance Criteria
- Dapat men-spawn container lewat kode Rust (memanggil API Docker daemon `unix:///var/run/docker.sock`).
- Tool Amass (sebagai *proof of concept*) dapat dijalankan di dalam container dan mengembalikan output JSON yang rapi (tidak berantakan dengan logging info).
- Memory dan CPU limits (cgroups) diterapkan pada container saat spawn.

### References
- [Ivy SRS (Non-Functional Security)](../Ivy_srs.md)
- Target folder: `backend/ivy-docker/` dan `tools/`
