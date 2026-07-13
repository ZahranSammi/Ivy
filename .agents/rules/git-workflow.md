# Git Workflow

Wajib branch dulu, PR kemudian. **Jangan pernah commit atau push langsung ke `main`** —
kalau lagi di `main`/`master`, bikin branch baru dulu sebelum ubah file apapun.

## Alur

1. Cek branch aktif. Kalau di `main`, `git checkout -b <branch-name>` dulu.
2. Kerjain perubahan di branch itu.
3. Push branch, buka PR ke `main` (pakai `.github/PULL_REQUEST_TEMPLATE.md` yang udah ada).
4. Jangan self-merge PR kecuali diminta eksplisit — biarin di-review dulu.

## Naming convention

`<type>/<issue-number>-<slug-singkat>`

Type: `feat`, `fix`, `chore`, `docs`, `refactor`, `test`.

Contoh: `feat/12-mcp-tool-registry`, `fix/8-scope-validator-bypass`.

Kalau gak ada issue number yang relevan, boleh skip nomor: `chore/update-cargo-deps`.

## Scope per branch

- Satu branch = satu concern (satu issue / satu task). Jangan gabungin perubahan gak
  berhubungan dalam satu PR — bikin review susah dan boundary Dev A/Dev B jadi kabur
  (lihat `overview.md`).
- Kalau di tengah kerja ternyata perlu nyentuh file di luar scope branch/issue saat ini
  (apalagi lintas area Dev A/Dev B), stop dan flag dulu ke user sebelum lanjut — jangan
  diam-diam nambahin ke branch yang sama.
