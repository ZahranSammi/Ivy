# 🌿 Ivy — Project Structure

> Panduan struktur folder dan organisasi kode untuk Ivy

---

## Overview

Ivy menggunakan **monorepo** structure karena terdiri dari beberapa komponen yang saling terhubung:
- Rust backend (Axum)
- Next.js frontend
- MCP tool wrappers (Docker)
- Plugin system

---

## Folder Structure

```
ivy/
├── .github/                          # GitHub CI/CD & templates
│   ├── workflows/
│   │   ├── ci.yml                    # Build + test + lint
│   │   ├── release.yml               # Build Docker images + release
│   │   └── security-audit.yml        # cargo audit + dependency check
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md
│   │   └── feature_request.md
│   └── PULL_REQUEST_TEMPLATE.md
│
├── docs/                             # Dokumentasi project
│   ├── Ivy_prd.md                    # Product Requirements Document
│   ├── Ivy_srs.md                    # Software Requirements Specification
│   ├── architecture.md               # Architecture Decision Records
│   ├── api-reference.md              # REST API documentation
│   ├── deployment-guide.md           # Self-hosted deployment guide
│   ├── plugin-development.md         # Guide membuat plugin
│   └── contributing.md               # Contribution guidelines
│
├── backend/                          # 🦀 Rust Backend (Axum)
│   ├── Cargo.toml                    # Workspace root manifest
│   ├── Cargo.lock
│   │
│   ├── ivy-server/                   # Main HTTP server binary
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs               # Entry point, server bootstrap
│   │       ├── config.rs             # Configuration loading (env + TOML)
│   │       ├── error.rs              # Global error types & handling
│   │       ├── state.rs              # Application shared state (AppState)
│   │       │
│   │       ├── routes/               # Axum route handlers
│   │       │   ├── mod.rs            # Router assembly
│   │       │   ├── auth.rs           # POST /auth/login, register, refresh
│   │       │   ├── projects.rs       # CRUD /projects
│   │       │   ├── scan.rs           # POST /scan/start, stop, status
│   │       │   ├── graph.rs          # GET /graph, query, export
│   │       │   ├── chat.rs           # POST /chat (AI interaction)
│   │       │   ├── tools.rs          # GET /tools, tool detail
│   │       │   ├── plugins.rs        # Plugin management endpoints
│   │       │   ├── reports.rs        # Report generation
│   │       │   ├── settings.rs       # LLM config, tool config
│   │       │   └── audit.rs          # GET /audit/logs
│   │       │
│   │       ├── middleware/           # Axum middleware layers
│   │       │   ├── mod.rs
│   │       │   ├── auth.rs           # JWT validation middleware
│   │       │   ├── audit.rs          # Request logging / audit trail
│   │       │   ├── rate_limit.rs     # API rate limiting
│   │       │   └── cors.rs           # CORS configuration
│   │       │
│   │       └── ws/                   # WebSocket handlers
│   │           ├── mod.rs
│   │           ├── handler.rs        # WS connection handler
│   │           ├── events.rs         # Event types (scan:progress, etc.)
│   │           └── broadcast.rs      # Channel-based event broadcasting
│   │
│   ├── ivy-core/                     # Core business logic library
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       │
│   │       ├── domain/              # Domain models & entities
│   │       │   ├── mod.rs
│   │       │   ├── project.rs        # Project entity
│   │       │   ├── scan.rs           # ScanSession, ToolExecution
│   │       │   ├── finding.rs        # Finding, Severity classification
│   │       │   ├── graph_node.rs     # Graph node types (Domain, IP, etc.)
│   │       │   ├── graph_edge.rs     # Graph edge types (HAS_SUBDOMAIN, etc.)
│   │       │   └── user.rs           # User entity
│   │       │
│   │       ├── services/            # Business logic / use cases
│   │       │   ├── mod.rs
│   │       │   ├── auth_service.rs   # Authentication logic (Argon2id, JWT)
│   │       │   ├── project_service.rs# Project CRUD + consent
│   │       │   ├── scan_service.rs   # Scan orchestration & lifecycle
│   │       │   ├── graph_service.rs  # Graph queries & mutations
│   │       │   ├── report_service.rs # Report generation (PDF/HTML)
│   │       │   ├── audit_service.rs  # Audit log recording
│   │       │   └── plugin_service.rs # Plugin install/remove/list
│   │       │
│   │       └── traits/              # Trait abstractions (ports)
│   │           ├── mod.rs
│   │           ├── repository.rs     # Database repository traits
│   │           ├── graph_store.rs    # Graph DB trait (Neo4j)
│   │           ├── cache.rs          # Cache trait (Redis)
│   │           └── tool_executor.rs  # Tool execution trait
│   │
│   ├── ivy-mcp/                     # MCP Protocol implementation
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── client.rs             # MCP client (connects to LLM)
│   │       ├── server.rs             # MCP server (exposes tools to LLM)
│   │       ├── protocol.rs           # MCP message types & serialization
│   │       │
│   │       ├── llm/                 # LLM provider abstraction
│   │       │   ├── mod.rs
│   │       │   ├── provider.rs       # LlmProvider trait
│   │       │   ├── gemini.rs         # Google Gemini implementation
│   │       │   ├── claude.rs         # Anthropic Claude implementation
│   │       │   ├── openai.rs         # OpenAI implementation
│   │       │   └── ollama.rs         # Ollama (local) implementation
│   │       │
│   │       ├── orchestrator/        # AI recon orchestration
│   │       │   ├── mod.rs
│   │       │   ├── planner.rs        # AI execution planning
│   │       │   ├── executor.rs       # Plan execution engine
│   │       │   └── correlator.rs     # Cross-tool result correlation
│   │       │
│   │       └── tools/               # MCP tool definitions
│   │           ├── mod.rs
│   │           ├── registry.rs       # Tool registry & discovery
│   │           └── schema.rs         # Tool input/output schema types
│   │
│   ├── ivy-db/                      # Database layer (adapters)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       │
│   │       ├── postgres/            # PostgreSQL (application data)
│   │       │   ├── mod.rs
│   │       │   ├── pool.rs           # Connection pool (sqlx)
│   │       │   ├── user_repo.rs      # User repository
│   │       │   ├── project_repo.rs   # Project repository
│   │       │   ├── scan_repo.rs      # Scan session repository
│   │       │   ├── chat_repo.rs      # Chat message repository
│   │       │   ├── audit_repo.rs     # Audit log repository
│   │       │   └── llm_config_repo.rs# LLM config repository
│   │       │
│   │       ├── neo4j/               # Neo4j (graph data)
│   │       │   ├── mod.rs
│   │       │   ├── client.rs         # Neo4j Bolt client connection
│   │       │   ├── graph_repo.rs     # Graph CRUD operations
│   │       │   ├── queries.rs        # Cypher query templates
│   │       │   └── mapper.rs         # Result → domain model mapping
│   │       │
│   │       ├── redis/               # Redis (cache)
│   │       │   ├── mod.rs
│   │       │   ├── client.rs         # Redis connection
│   │       │   └── cache_impl.rs     # Cache trait implementation
│   │       │
│   │       └── migrations/          # SQL migrations (sqlx)
│   │           ├── 001_create_users.sql
│   │           ├── 002_create_projects.sql
│   │           ├── 003_create_scan_sessions.sql
│   │           ├── 004_create_tool_executions.sql
│   │           ├── 005_create_chat_messages.sql
│   │           ├── 006_create_audit_logs.sql
│   │           └── 007_create_llm_configs.sql
│   │
│   ├── ivy-docker/                  # Docker container management
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── manager.rs            # Container lifecycle management
│   │       ├── sandbox.rs            # Security sandbox configuration
│   │       ├── executor.rs           # Tool execution in containers
│   │       ├── output_stream.rs      # Container stdout/stderr streaming
│   │       └── scope_filter.rs       # Network egress filtering by scope
│   │
│   └── ivy-cli/                     # CLI binary (optional)
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs               # CLI entry point
│           └── commands/
│               ├── mod.rs
│               ├── scan.rs            # ivy scan example.com
│               ├── plugin.rs          # ivy plugin install/remove
│               ├── config.rs          # ivy config set/get
│               └── export.rs          # ivy export --format json
│
├── frontend/                         # ⚛️ Next.js Frontend
│   ├── package.json
│   ├── next.config.ts
│   ├── tsconfig.json
│   ├── tailwind.config.ts            # (jika pakai Tailwind, optional)
│   │
│   ├── public/
│   │   ├── favicon.ico
│   │   ├── ivy-logo.svg
│   │   └── fonts/
│   │
│   ├── src/
│   │   ├── app/                     # Next.js App Router
│   │   │   ├── layout.tsx            # Root layout
│   │   │   ├── page.tsx              # Dashboard (/)
│   │   │   ├── login/
│   │   │   │   └── page.tsx          # Login page
│   │   │   ├── project/
│   │   │   │   └── [id]/
│   │   │   │       ├── page.tsx      # Project workspace (graph+chat+tools)
│   │   │   │       ├── findings/
│   │   │   │       │   └── page.tsx  # Findings table view
│   │   │   │       └── report/
│   │   │   │           └── page.tsx  # Report view
│   │   │   ├── settings/
│   │   │   │   ├── page.tsx          # General settings
│   │   │   │   ├── llm/
│   │   │   │   │   └── page.tsx      # LLM provider config
│   │   │   │   └── plugins/
│   │   │   │       └── page.tsx      # Plugin manager
│   │   │   └── audit/
│   │   │       └── page.tsx          # Audit log viewer
│   │   │
│   │   ├── components/              # React components
│   │   │   ├── ui/                  # Base UI primitives
│   │   │   │   ├── Button.tsx
│   │   │   │   ├── Input.tsx
│   │   │   │   ├── Modal.tsx
│   │   │   │   ├── Table.tsx
│   │   │   │   ├── Badge.tsx
│   │   │   │   ├── Toast.tsx
│   │   │   │   └── ProgressBar.tsx
│   │   │   │
│   │   │   ├── layout/              # Layout components
│   │   │   │   ├── Sidebar.tsx       # Project list + scan progress
│   │   │   │   ├── Header.tsx        # Top bar + kill switch
│   │   │   │   └── TabNav.tsx        # Graph/Findings/Terminal/Report tabs
│   │   │   │
│   │   │   ├── graph/               # Graph visualization
│   │   │   │   ├── GraphCanvas.tsx   # Main Cytoscape.js canvas
│   │   │   │   ├── NodeInspector.tsx # Node detail side panel
│   │   │   │   ├── GraphToolbar.tsx  # Filter, search, layout controls
│   │   │   │   ├── GraphLegend.tsx   # Node type color legend
│   │   │   │   └── graph-styles.ts   # Cytoscape.js style config
│   │   │   │
│   │   │   ├── chat/                # AI chat interface
│   │   │   │   ├── ChatPanel.tsx     # Chat container
│   │   │   │   ├── ChatMessage.tsx   # Individual message bubble
│   │   │   │   └── ChatInput.tsx     # Message input + send
│   │   │   │
│   │   │   ├── scan/                # Scan management
│   │   │   │   ├── ScanProgress.tsx  # Overall scan progress
│   │   │   │   ├── ToolStatus.tsx    # Individual tool status card
│   │   │   │   ├── TerminalOutput.tsx# Real-time tool output terminal
│   │   │   │   └── KillSwitch.tsx    # Emergency stop button
│   │   │   │
│   │   │   ├── project/             # Project components
│   │   │   │   ├── ProjectCard.tsx   # Project list card
│   │   │   │   ├── NewProjectModal.tsx# Create project + consent
│   │   │   │   ├── ScopeConfig.tsx   # Scope configuration form
│   │   │   │   └── LegalDisclaimer.tsx# Legal consent dialog
│   │   │   │
│   │   │   ├── findings/            # Findings table
│   │   │   │   ├── FindingsTable.tsx # Sortable/filterable table
│   │   │   │   ├── FindingRow.tsx    # Individual finding row
│   │   │   │   └── SeverityBadge.tsx # Severity color badge
│   │   │   │
│   │   │   └── settings/            # Settings forms
│   │   │       ├── LlmConfigForm.tsx # LLM provider configuration
│   │   │       ├── ToolConfigForm.tsx# Tool parameter defaults
│   │   │       └── PluginCard.tsx    # Plugin install/remove card
│   │   │
│   │   ├── hooks/                   # Custom React hooks
│   │   │   ├── useWebSocket.ts       # WebSocket connection & events
│   │   │   ├── useGraph.ts           # Graph data fetching & state
│   │   │   ├── useScan.ts            # Scan control & progress
│   │   │   ├── useChat.ts            # AI chat interaction
│   │   │   ├── useAuth.ts            # Authentication state
│   │   │   └── useProject.ts         # Project CRUD
│   │   │
│   │   ├── lib/                     # Utility libraries
│   │   │   ├── api.ts                # REST API client (fetch wrapper)
│   │   │   ├── ws.ts                 # WebSocket client singleton
│   │   │   ├── auth.ts               # JWT storage & refresh
│   │   │   └── constants.ts          # App-wide constants
│   │   │
│   │   ├── types/                   # TypeScript type definitions
│   │   │   ├── project.ts            # Project, ScopeConfig
│   │   │   ├── scan.ts               # ScanSession, ToolExecution
│   │   │   ├── graph.ts              # GraphNode, GraphEdge
│   │   │   ├── finding.ts            # Finding, Severity
│   │   │   ├── chat.ts               # ChatMessage
│   │   │   ├── tool.ts               # Tool, ToolSchema
│   │   │   ├── ws-events.ts          # WebSocket event payloads
│   │   │   └── api.ts                # API response types
│   │   │
│   │   └── styles/                  # Global styles
│   │       ├── globals.css           # CSS reset + variables
│   │       └── graph-theme.css       # Graph visualization theme
│   │
│   └── tests/
│       ├── components/               # Component tests
│       └── e2e/                      # End-to-end tests (Playwright)
│
├── tools/                            # 🔧 MCP Tool Wrappers (Docker)
│   ├── README.md                     # How tools work
│   │
│   ├── _base/                       # Base Docker image for all tools
│   │   └── Dockerfile                # Common dependencies
│   │
│   ├── ivy-whois/
│   │   ├── Dockerfile
│   │   ├── tool.json                 # MCP tool schema definition
│   │   └── entrypoint.sh             # Parse args → run tool → output JSON
│   │
│   ├── ivy-dns/
│   │   ├── Dockerfile
│   │   ├── tool.json
│   │   └── entrypoint.sh
│   │
│   ├── ivy-amass/
│   │   ├── Dockerfile
│   │   ├── tool.json
│   │   └── entrypoint.sh
│   │
│   ├── ivy-subfinder/
│   │   ├── Dockerfile
│   │   ├── tool.json
│   │   └── entrypoint.sh
│   │
│   ├── ivy-crtsh/
│   │   ├── Dockerfile
│   │   ├── tool.json
│   │   └── entrypoint.sh
│   │
│   ├── ivy-theharvester/
│   │   ├── Dockerfile
│   │   ├── tool.json
│   │   └── entrypoint.sh
│   │
│   ├── ivy-wayback/
│   │   ├── Dockerfile
│   │   ├── tool.json
│   │   └── entrypoint.sh
│   │
│   ├── ivy-shodan/
│   │   ├── Dockerfile
│   │   ├── tool.json
│   │   └── entrypoint.sh
│   │
│   ├── ivy-rustscan/
│   │   ├── Dockerfile
│   │   ├── tool.json
│   │   └── entrypoint.sh
│   │
│   ├── ivy-nmap/
│   │   ├── Dockerfile
│   │   ├── tool.json
│   │   └── entrypoint.sh
│   │
│   ├── ivy-httpx/
│   │   ├── Dockerfile
│   │   ├── tool.json
│   │   └── entrypoint.sh
│   │
│   ├── ivy-nuclei/
│   │   ├── Dockerfile
│   │   ├── tool.json
│   │   └── entrypoint.sh
│   │
│   ├── ivy-ffuf/
│   │   ├── Dockerfile
│   │   ├── tool.json
│   │   └── entrypoint.sh
│   │
│   ├── ivy-whatweb/
│   │   ├── Dockerfile
│   │   ├── tool.json
│   │   └── entrypoint.sh
│   │
│   └── ivy-nikto/
│       ├── Dockerfile
│       ├── tool.json
│       └── entrypoint.sh
│
├── plugins/                          # 🔌 Plugin packages
│   └── ivy-exploit/                  # Exploit plugin (installed separately)
│       ├── manifest.json             # Plugin metadata & tool list
│       ├── README.md
│       ├── tools/
│       │   ├── ivy-sqlmap/
│       │   │   ├── Dockerfile
│       │   │   ├── tool.json
│       │   │   └── entrypoint.sh
│       │   ├── ivy-xsstrike/
│       │   │   ├── Dockerfile
│       │   │   ├── tool.json
│       │   │   └── entrypoint.sh
│       │   ├── ivy-commix/
│       │   │   ├── Dockerfile
│       │   │   ├── tool.json
│       │   │   └── entrypoint.sh
│       │   └── ivy-hydra/
│       │       ├── Dockerfile
│       │       ├── tool.json
│       │       └── entrypoint.sh
│       └── install.sh                # Plugin installation script
│
├── deploy/                           # 🚀 Deployment configurations
│   ├── docker-compose.yml            # Full stack (backend+frontend+neo4j+redis+pg)
│   ├── docker-compose.dev.yml        # Development overrides
│   ├── Dockerfile.backend            # Multi-stage Rust build
│   ├── Dockerfile.frontend           # Next.js production build
│   ├── nginx.conf                    # Reverse proxy config
│   └── .env.example                  # Environment variables template
│
├── scripts/                          # 🛠️ Development & utility scripts
│   ├── setup.sh                      # First-time dev environment setup
│   ├── build-tools.sh                # Build all tool Docker images
│   ├── dev.sh                        # Start dev environment
│   ├── test.sh                       # Run all tests
│   ├── seed-db.sh                    # Seed database with sample data
│   └── reset-db.sh                   # Reset databases (dev only)
│
├── tests/                            # 🧪 Integration & E2E tests
│   ├── integration/
│   │   ├── test_scan_flow.rs         # Full scan flow integration test
│   │   ├── test_graph_operations.rs  # Graph CRUD tests
│   │   └── test_mcp_tools.rs         # MCP tool execution tests
│   └── fixtures/
│       ├── sample_amass_output.json   # Sample tool outputs for testing
│       ├── sample_nmap_output.json
│       └── sample_graph.cypher        # Neo4j test fixtures
│
├── .env.example                      # Environment variables template
├── .gitignore
├── .dockerignore
├── Cargo.toml                        # Rust workspace root
├── LICENSE                           # Open source license (MIT / Apache-2.0)
├── README.md                         # Project overview & quick start
├── CHANGELOG.md                      # Version history
├── SECURITY.md                       # Security policy & reporting
└── Makefile                          # Common dev commands
```

---

## Cargo Workspace Configuration

```toml
# Cargo.toml (workspace root)
[workspace]
resolver = "2"
members = [
    "backend/ivy-server",
    "backend/ivy-core",
    "backend/ivy-mcp",
    "backend/ivy-db",
    "backend/ivy-docker",
    "backend/ivy-cli",
]

[workspace.dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres", "uuid", "chrono"] }
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
tracing = "0.1"
tracing-subscriber = "0.3"
anyhow = "1"
thiserror = "2"
jsonwebtoken = "9"
argon2 = "0.5"
bollard = "0.18"                     # Docker API client
neo4rs = "0.8"                       # Neo4j Bolt driver
redis = { version = "0.27", features = ["tokio-comp"] }
reqwest = { version = "0.12", features = ["json"] }
tokio-tungstenite = "0.26"           # WebSocket
tower-http = { version = "0.6", features = ["cors", "trace"] }
```

---

## Key Design Decisions

### Backend Crate Responsibilities

```
                        ┌─────────────────┐
                        │   ivy-server    │  HTTP server, routes,
                        │   (binary)      │  middleware, WebSocket
                        └───────┬─────────┘
                                │ depends on
                ┌───────────────┼───────────────┐
                │               │               │
        ┌───────┴──────┐ ┌─────┴──────┐ ┌──────┴───────┐
        │  ivy-core    │ │  ivy-mcp   │ │  ivy-docker  │
        │  (library)   │ │  (library) │ │  (library)   │
        │              │ │            │ │              │
        │ Domain models│ │ MCP proto  │ │ Container    │
        │ Services     │ │ LLM abstrc │ │ management   │
        │ Business     │ │ AI orch.   │ │ Sandbox      │
        │ logic        │ │ Tool defs  │ │ Streaming    │
        └───────┬──────┘ └────────────┘ └──────────────┘
                │ depends on
        ┌───────┴──────┐
        │  ivy-db      │
        │  (library)   │
        │              │
        │ PostgreSQL   │
        │ Neo4j        │
        │ Redis        │
        │ Migrations   │
        └──────────────┘
```

### Tool Wrapper Pattern

Setiap tool mengikuti pola yang sama:

```
tools/ivy-<toolname>/
├── Dockerfile           # Install tool + dependencies
├── tool.json            # MCP tool schema (name, inputSchema, etc.)
└── entrypoint.sh        # 1. Parse JSON input from stdin
                         # 2. Run the actual tool
                         # 3. Parse output → structured JSON
                         # 4. Write result JSON to stdout
```

**Contoh `tool.json`:**
```json
{
  "name": "ivy_amass",
  "description": "Subdomain enumeration using OWASP Amass",
  "category": "passive",
  "inputSchema": {
    "type": "object",
    "properties": {
      "target": { "type": "string", "description": "Target domain" },
      "mode": { "enum": ["passive", "active"], "default": "passive" },
      "timeout": { "type": "integer", "default": 300 }
    },
    "required": ["target"]
  },
  "outputSchema": {
    "type": "object",
    "properties": {
      "findings": { "type": "array" },
      "raw_output": { "type": "string" }
    }
  },
  "docker": {
    "image": "ivy/tool-amass:latest",
    "memory_limit": "2g",
    "cpu_limit": "1.0",
    "network_mode": "ivy-scan-net"
  }
}
```

### Plugin Manifest

```json
{
  "name": "ivy-exploit",
  "version": "1.0.0",
  "description": "Exploitation tools plugin for Ivy",
  "author": "Ivy Community",
  "license": "MIT",
  "requires_consent": true,
  "consent_message": "This plugin contains exploitation tools. Use only with authorization.",
  "tools": [
    "tools/ivy-sqlmap/tool.json",
    "tools/ivy-xsstrike/tool.json",
    "tools/ivy-commix/tool.json",
    "tools/ivy-hydra/tool.json"
  ]
}
```

---

*Document Version: 1.0*
*Last Updated: 2026-07-12*
*Reference: [Ivy PRD](./Ivy_prd.md) | [Ivy SRS](./Ivy_srs.md)*
