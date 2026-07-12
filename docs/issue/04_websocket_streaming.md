---
title: "[Dev B] Backend/Frontend: Real-time WebSocket Streaming"
labels: ["backend", "frontend", "enhancement"]
assignees: [] # Assign to Developer 2
---

### Objective
Membangun komunikasi dua arah (*bidirectional*) via **WebSocket** antara server Axum dan frontend Next.js untuk menyalurkan log eksekusi secara real-time dan notifikasi progress scan.

### Tasks
- [ ] **Backend (`ivy-server`)**:
  - Implementasi rute `GET /ws` menggunakan `axum::extract::ws::WebSocketUpgrade`.
  - Setup mekanisme broadcasting (`tokio::sync::broadcast`) untuk mengirim event ke semua client yang terkoneksi pada project yang sama (`backend/ivy-server/src/ws/`).
  - Definisikan format JSON event (misal: `type: "scan_progress"`, `type: "tool_output"`).
- [ ] **Frontend (`frontend`)**:
  - Lengkapi hook `useWebSocket.ts` untuk melakukan koneksi, *reconnect*, dan menerima pesan.
  - Buat komponen `TerminalOutput.tsx` yang me-render log stream dari WebSocket secara berurutan.

### Acceptance Criteria
- Frontend berhasil terhubung ke `ws://localhost:3001/ws`.
- Saat backend membroadcast data log, frontend (`TerminalOutput`) dapat merender baris teks baru secara instan tanpa perlu reload halaman.

### References
- Target folder: `backend/ivy-server/src/ws/` dan `frontend/src/hooks/useWebSocket.ts`
