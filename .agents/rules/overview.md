---
trigger: always_on
---

# 🌿 Ivy — Overview

## Apa ini

Ivy adalah tool OSINT/recon berbasis AI-orchestration untuk bug bounty & pentest. User kasih
target domain, AI (lewat MCP) yang memutuskan tools apa yang dijalankan dan urutannya, hasilnya
dikorelasi otomatis ke graph database (Neo4j) buat visualisasi attack surface. AI bertindak
sebagai "operator" yang menjalankan banyak tools OSINT terpisah secara terkoordinasi — bukan
sekadar dipanggil manual satu-satu.

Self-hosted, single-user, no auth di v1.

## Masalah yang diselesaikan

Recon manual pakai banyak tools terpisah (Amass, Subfinder, Nmap, dll) itu:

- **Fragmented** — output tiap tool beda format, harus digabung manual
- **Repetitive** — user harus tahu tools mana yang relevan buat tiap tahap
- **Sulit dikorelasi** — susah lihat relasi domain → subdomain → IP → port → service → vuln
  tanpa graph
- **Time-consuming** — workflow-nya sekuensial dan manual

Ivy nyoba nyelesain ini: AI yang plan & eksekusi tools, hasil di-dedup dan disimpan sebagai
graph (bukan tabel flat), jadi attack surface bisa dilihat relasinya, bukan cuma dibaca sebagai
list.

## Flow

1. User input target domain + scope (in-scope/out-of-scope rules)
2. AI verifikasi domain & bikin execution plan (via MCP)
3. Passive recon dijalankan (WHOIS, DNS, subdomain enum, dst)
4. Hasil ditampilkan ke user
5. **Consent eksplisit** dari user sebelum lanjut ke active recon — tidak ada auto-escalation
6. Active recon dijalankan (port scan, service detection, dst)
7. AI korelasi semua hasil → merge ke Neo4j graph (dedup, incremental, bukan replace)
8. User lihat graph interaktif

Exploitation module (di luar v1) rencananya jadi plugin terpisah dengan double-consent —
sengaja dipisah dari core recon karena beda risiko legal.

## Prinsip yang tidak boleh dilonggarkan

- **Scope enforcement** — tool tidak boleh menyentuh target di luar scope yang didefinisikan
  user. Cek ini harus jalan sebelum tool apapun dieksekusi.
- **Human-in-the-loop** — tidak ada eskalasi otomatis dari passive ke active recon, apalagi ke
  exploitation. User harus konfirmasi eksplisit di tiap tahap yang lebih invasif.

## Status saat ini (v1 / MVP)

Semua masih stub — belum ada koneksi Postgres/Neo4j, belum ada MCP client, belum ada eksekusi
tool sungguhan. Yang ada baru skeleton modul.

Beberapa hal dari visi awal sengaja **di-defer**, bukan hilang dari roadmap:

- Tidak ada container sandboxing (tools jalan sebagai host subprocess langsung) — ini risiko
  yang sudah didokumentasikan, bukan oversight
- Tidak ada multi-user/auth
- Tidak ada Redis cache
- Exploit tools (SQLMap, Nuclei, dll) belum di-wire ke sistem
- Tidak ada Docker Compose — Neo4j & Postgres jalan native di host


sedang dikerjakan sekarang — jangan dijadikan acuan scope aktif.