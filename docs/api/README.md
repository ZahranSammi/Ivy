# Ivy API — kontrak frontend ↔ backend

Backend (`backend/`) masih kosong/stub sejak commit restart, jadi belum ada
kontrak resmi dari sisi Rust. Frontend (`apps/frontend`) sudah lebih dulu
punya API client + types yang ke-port dari implementasi sebelumnya
(`apps/frontend/src/lib/api.ts`, `apps/frontend/src/types/api/*.ts`).

Dokumen ini + `openapi.yaml` di folder ini adalah kontrak itu, ditulis ulang
dalam bentuk OpenAPI supaya backend dev implementasi persis sama bentuknya
tanpa harus baca kode TypeScript. **Sumber kebenaran saat ini adalah kode
frontend** — kalau butuh ubah bentuk request/response, ubah dulu di
`types/api/*.ts` + `lib/api.ts`, baru sinkronkan `openapi.yaml`.

## Cara lihat dokumentasi Swagger-nya

`openapi.yaml` adalah spec OpenAPI 3.0.3 biasa, belum di-host di mana-mana.
Cara tercepat buat lihat sebagai Swagger UI tanpa install permanen:

- **Frontend Dev Server (`bun run dev` / `npm run dev`)**:
  Buka `http://localhost:3000/docs` atau `http://localhost:3000/swagger` saat server frontend aktif.
- **Standalone CLI (npx)**:
  ```bash
  npx --yes swagger-ui-watcher docs/api/openapi.yaml
  ```
- **Online**: Copy-paste isi `openapi.yaml` ke https://editor.swagger.io
- **VS Code**: Extension "OpenAPI (Swagger) Editor" bisa render preview langsung dari file `.yaml` ini.

Belum ditambahkan swagger-ui sebagai dependency permanen di repo karena
belum ada backend yang perlu serve dokumentasinya secara live — kalau nanti
Axum sudah jalan, opsi paling umum adalah mount `utoipa-swagger-ui` (kalau
pakai `utoipa` buat generate spec dari kode Rust langsung) supaya
`openapi.yaml` ini gak perlu di-maintain manual dua tempat.

### Validasi spec habis diedit

`redocly.yaml` di folder ini nyimpen rule lint (2 rule dari preset
"recommended" dimatiin karena gak relevan buat internal contract doc — lihat
komentar di file-nya). Sebelum commit perubahan ke `openapi.yaml`:

```bash
npx --yes @redocly/cli lint docs/api/openapi.yaml --config docs/api/redocly.yaml
```

Harus keluar "Woohoo! Your API description is valid." — kalau ada error
(bukan warning), itu tandanya schema-nya emang salah, bukan cuma gaya
penulisan.

## No auth (v1)

Self-hosted, single-user — gak ada Authorization header di request manapun.
Lihat `.agents/rules/overview.md`.

## Daftar endpoint

| Method | Path | Deskripsi |
|---|---|---|
| POST | `/targets` | Buat target baru |
| GET | `/targets/{targetId}` | Ambil detail target |
| POST | `/targets/{targetId}/scan/passive` | Mulai passive recon |
| POST | `/targets/{targetId}/scan/active` | Mulai active recon (butuh consent user duluan di sisi frontend) |
| GET | `/targets/{targetId}/scan/status` | List semua scan session |
| GET | `/targets/{targetId}/graph` | Ambil graph attack-surface |
| GET | `/settings/llm` | Ambil config LLM aktif |
| PUT | `/settings/llm` | Update config LLM |

Detail request/response body ada di `openapi.yaml`.

## Yang belum final — jangan diimplementasikan diam-diam

Ini bukan bug di spec, ini memang belum diputuskan. Kalau backend dev nemu
salah satu dari ini, stop dan konfirmasi ke frontend dev dulu, jangan nebak:

1. **Bentuk error response** — frontend cuma baca response body sebagai
   text buat pesan error (`request()` di `lib/api.ts` gak parse JSON error).
   `openapi.yaml` kasih saran schema `{ error, message }` tapi itu belum
   disepakati.
2. **Scope in-scope/out-of-scope rules** — flow di `overview.md` bilang
   user define scope rules (bukan cuma intensity), tapi `ScopeConfig` di
   kode cuma punya field `intensity`. Field buat domain include/exclude
   list belum ada tempatnya.
3. **`proposed_active_tools` di `ScanSession`** — ini asumsi dari frontend
   session sebelumnya (dikomentari langsung di `types/api/scan.ts`), belum
   dikonfirmasi backend mau kirim dalam bentuk apa setelah passive scan
   selesai.
4. **Pagination/filtering graph** — PRD nyebut target performa graph
   sampai 10.000 node (`docs/Ivy_prd.md` §10), tapi `GET /graph` di sini
   masih return semua node/edge sekaligus, gak ada query param apa pun.
   Belum masalah buat MVP, tapi bakal jadi masalah kalau graph beneran
   gede.

## Kalau backend mau generate spec dari kode Rust langsung

Alternatif jangka panjang biar `openapi.yaml` ini gak perlu di-maintain
manual: pakai crate `utoipa` (annotate handler Axum dengan derive macro,
spec ke-generate dari kode). Belum dilakukan sekarang karena backend-nya
sendiri belum ada — cuma dicatat sebagai opsi, bukan keputusan.
