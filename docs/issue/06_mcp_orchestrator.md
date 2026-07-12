---
title: "[Dev A] AI: Implement MCP LLM Orchestrator"
labels: ["backend", "ai"]
assignees: [] # Assign to Developer 1 (Zahra)
---

### Objective
Membangun kecerdasan buatan (*otak*) di `ivy-mcp` yang akan menerima prompt, menganalisis temuan (*findings*), dan memutuskan tool OSINT apa yang perlu dijalankan berikutnya.

### Tasks
- [ ] Implementasikan `LlmProvider` trait untuk Gemini / Claude.
- [ ] Bangun logika `Planner` yang menerima JSON dari hasil tool sebelumnya dan meng-generate rencana (tool selanjutnya).
- [ ] Buat JSON schema (Model Context Protocol) untuk memastikan LLM mengembalikan format perintah eksekusi yang bisa di-*parse*.

### Acceptance Criteria
- Fungsi *Orchestrator* berhasil menerima input recon dan mengeluarkan response JSON terstruktur (menyarankan tool eksploit atau pasif berikutnya).
