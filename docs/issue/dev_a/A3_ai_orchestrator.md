# Issue A3: The AI Brain (Orchestrator)

## Context
Tanggung jawab Developer A (Zahra) untuk menghidupkan AI menggunakan protokol MCP di modul `ivy-mcp`.

## FR Terkait
- **FR-007**: LLM Provider Config (Gemini/Claude)
- **FR-008**: AI Recon Planning (Prompt & JSON Output)
- **FR-009**: MCP Tool Execution (Mock protocol loop)
- **FR-010**: AI Result Correlation (Graph mapping)
- **FR-024**: Kill Switch (Cancellation Token/Signal)

## Tasks
- [ ] Implementasikan abstraksi provider LLM agar mudah berganti antara Gemini atau Claude (FR-007).
- [x] Buat prompt dan logika agentic untuk menyusun Recon Planning berdasarkan target awal (FR-008).
- [x] Integrasikan eksekusi *tool* menggunakan Model Context Protocol (MCP) (FR-009).
- [x] Buat sistem korelasi hasil eksekusi tool untuk memetakan temuan ke model graf (FR-010).
- [ ] Terapkan kapabilitas *Kill Switch* menggunakan cancellation token agar proses pemindaian dapat dihentikan (FR-024).

## Acceptance Criteria
- Agent AI dapat mengambil keputusan alat mana yang akan dijalankan berikutnya berdasarkan hasil sebelumnya.
- Pemanggilan LLM dan parsing respons bekerja dengan baik.
- Proses eksekusi dapat diinterupsi/dibatalkan kapan saja (Kill Switch berfungsi).
