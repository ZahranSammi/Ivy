---
trigger: always_on
---

# Engineering Judgment

Sebelum nambah sesuatu (dependency, abstraction layer, config knob, tool baru, plugin baru,
pattern baru), pikir dulu sebagai senior engineer: **apakah ini benar-benar dibutuhkan untuk
requirement yang sedang dikerjakan sekarang?** Kalau tidak, jangan dipakai.

## Cara mikirnya

1. **Cek requirement aktual dulu.** Bukan "mungkin nanti butuh", tapi "diminta/dibutuhkan
   sekarang atau tidak". Kalau speculative — skip.
2. **Solusi paling sederhana yang memenuhi requirement, menang.** Jangan tambah fleksibilitas,
   configurability, atau error handling untuk skenario yang gak diminta dan gak mungkin terjadi
   di scope Ivy saat ini.
3. **Kalau ragu, state assumption-nya, jangan diam-diam milih.** Kalau ada opsi lebih simpel,
   bilang — termasuk push back ke saya kalau saya sendiri minta sesuatu yang overkill.
4. **Setiap dependency/crate/npm package baru harus dijustifikasi 1 kalimat**: kenapa ini,
   kenapa bukan yang udah ada di workspace.

## Terapkan khusus ke area-area ini (rawan overengineering di project OSINT/MCP kayak Ivy)

- **Provider/plugin abstraction** — jangan bikin trait/interface generik untuk "future provider"
  kalau baru ada 1 implementasi konkret dipakai. Tambahkan abstraksi pas provider ke-2 beneran ada.
- **MCP tool baru** — jangan wrap tool yang belum ada di scope MVP (lihat `docs/Ivy_prd.md`
  roadmap phase) cuma karena "sekalian aja".
- **Config options** — jangan expose env var / config knob yang belum ada use case-nya. Hardcode
  dulu, extract jadi config kalau memang butuh diubah-ubah.
- **Docker/network layer** — jangan tambah layer isolasi/proxy baru di luar yang udah didefinisikan
  (per-job network + egress filter) tanpa alasan konkret yang bisa dijelaskan.

## Yang TETAP wajib walau simple

Ini bukan alasan buat skip safety-critical stuff di `overview.md` (scope enforcement, sandboxing,
consent gate). "Sederhana" berarti gak nambah kompleksitas yang gak perlu — bukan ngirit di bagian
yang emang butuh robust karena alasan keamanan.

## Sebelum mulai coding

- Kalau ada lebih dari satu cara wajar untuk interpretasi request, tampilkan opsinya — jangan
  diam-diam pilih satu.
- Kalau ada yang ambigu/gak jelas, berhenti dan tanya. Jangan nebak terus jalan.

## Surgical changes

- Pas edit kode existing: sentuh cuma yang diminta. Jangan "sambil benerin" kode/komentar/format
  di sekitarnya yang gak diminta.
- Jangan refactor sesuatu yang gak rusak.
- Ikutin style kode yang udah ada di file itu, walau kamu punya preferensi lain.
- Kalau nemu dead code gak terkait, sebutkan aja — jangan langsung dihapus.
- Kalau perubahanmu bikin import/variable jadi unused, itu boleh dibersihin (karena itu akibat
  langsung dari perubahanmu).

## Goal-driven execution

Ubah task jadi kriteria yang bisa diverifikasi, bukan "pokoknya jalan":
- "Tambah validasi" → tulis test untuk invalid input dulu, baru bikin lolos.
- "Fix bug" → tulis test yang reproduce bug-nya dulu, baru fix sampai test-nya hijau.
- Untuk task multi-step, kasih plan singkat: `1. [step] → verify: [cek apa]`.
