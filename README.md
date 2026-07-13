# Ivy

AI-orchestrated OSINT recon: kasih target domain, AI (via MCP) jalanin passive recon,
minta konfirmasi sebelum active recon, hasilnya dikorelasikan ke graph database (Neo4j).

Spesifikasi lengkap: [`docs/Ivy_srs.md`](docs/Ivy_srs.md).

## Run (dev)

Neo4j dan PostgreSQL diinstall & dijalankan native di host (tidak pakai Docker di v1).

```bash
cp .env.example .env   # isi LLM_API_KEY + connection string Neo4j/Postgres lokal

cd backend && cargo run      # :3001
cd frontend && npm run dev   # :3000
```

Neo4j Browser: http://localhost:7474 · Backend: http://localhost:3001/api/v1 ·
Frontend: http://localhost:3000
