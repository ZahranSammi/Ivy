# Graph Report - .  (2026-07-28)

## Corpus Check
- Corpus is ~11,106 words - fits in a single context window. You may not need a graph.

## Summary
- 228 nodes · 204 edges · 46 communities (25 shown, 21 thin omitted)
- Extraction: 94% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 11 edges (avg confidence: 0.81)
- Token cost: 107,381 input · 0 output

## Community Hubs (Navigation)
- TypeScript App Config
- Frontend Build Tooling
- TypeScript Node Config
- Frontend Package Manifest
- Graph Type Definitions
- Safety & Legal Principles
- Graph SVG Component
- Project Identity & API Contract
- API Client Functions
- Graph Canvas View
- System Architecture & MVP Status
- Recon Tools & MCP Registry
- Scan API Types
- Architecture Decisions
- Graph Mock Data
- Node Inspector Component
- Scope Config & App Flow
- Graph API Types
- LLM Config Types
- Target API Types
- Vite Env Types
- Root TS Config
- Graph Legend Component
- Nav Rail Component
- Error Response Schema
- Target Graph Endpoint
- Scan Session Schema
- Pagination & NFRs
- Config Options Rule
- Docker Network Layer Rule
- Goal-Driven Execution Rule
- Surgical Changes Rule
- Branch Naming Convention
- Create Target Endpoint
- Get LLM Config Endpoint
- Get Scan Status Endpoint
- Get Target Endpoint
- Graph Data Schema
- Start Active Scan Endpoint
- Update LLM Config Endpoint
- Problem Statement

## God Nodes (most connected - your core abstractions)
1. `compilerOptions` - 18 edges
2. `compilerOptions` - 11 edges
3. `request()` - 9 edges
4. `System Architecture (Frontend / AI Orchestration / Tool Execution / Data layers)` - 7 edges
5. `Ivy API OpenAPI Spec (v0.1.0)` - 6 edges
6. `scripts` - 5 edges
7. `Ivy (AI-orchestrated OSINT/recon tool)` - 5 edges
8. `MCP Tools Registry (ivy_whois, ivy_dns, ivy_amass, ... ivy_hydra)` - 5 edges
9. `emit` - 4 edges
10. `lib` - 4 edges

## Surprising Connections (you probably didn't know these)
- `Ivy (AI-orchestrated OSINT/recon tool)` --semantically_similar_to--> `Ivy Product (PRD vision)`  [INFERRED] [semantically similar]
  .agents/rules/overview.md → docs/Ivy_prd.md
- `Ivy Core Flow (8 steps: input domain -> plan -> passive -> consent -> active -> graph)` --semantically_similar_to--> `Ivy Application Flow (Phase 0-4)`  [INFERRED] [semantically similar]
  .agents/rules/overview.md → docs/Ivy_prd.md
- `Scope Enforcement Principle` --semantically_similar_to--> `Scope & Legal Boundary Control (gap analysis §7.1)`  [INFERRED] [semantically similar]
  .agents/rules/overview.md → docs/Ivy_prd.md
- `Human-in-the-Loop Principle (no auto-escalation passive->active->exploit)` --semantically_similar_to--> `Security & Ethical Considerations / Mandatory Safeguards (§9)`  [INFERRED] [semantically similar]
  .agents/rules/overview.md → docs/Ivy_prd.md
- `Container Sandboxing (deferred, tools run as host subprocess)` --semantically_similar_to--> `Docker Sandboxed Tool Execution`  [INFERRED] [semantically similar]
  .agents/rules/overview.md → docs/Ivy_prd.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **API Contract Documentation Set** — docs_api_readme_api_contract, docs_api_openapi_spec, docs_api_redocly_lint_config [EXTRACTED 1.00]
- **Ivy v1 MVP Deferred Scope** — agents_rules_overview_mvp_status, agents_rules_overview_container_sandboxing, agents_rules_overview_exploitation_module, docs_ivy_prd_redis [INFERRED 0.75]
- **MCP Orchestration Layer Components** — docs_ivy_prd_mcp, docs_ivy_prd_ai_engine, docs_ivy_prd_tool_runner, docs_ivy_prd_neo4j [EXTRACTED 1.00]

## Communities (46 total, 21 thin omitted)

### Community 0 - "TypeScript App Config"
Cohesion: 0.08
Nodes (25): compilerOptions, allowImportingTsExtensions, baseUrl, isolatedModules, jsx, lib, module, moduleDetection (+17 more)

### Community 1 - "Frontend Build Tooling"
Cohesion: 0.13
Nodes (15): devDependencies, tailwindcss, @tailwindcss/vite, @types/node, typescript, vite, @vitejs/plugin-vue, vue-tsc (+7 more)

### Community 2 - "TypeScript Node Config"
Cohesion: 0.13
Nodes (14): compilerOptions, allowImportingTsExtensions, isolatedModules, lib, module, moduleDetection, moduleResolution, noEmit (+6 more)

### Community 3 - "Frontend Package Manifest"
Cohesion: 0.15
Nodes (12): dependencies, vue, name, private, scripts, build, dev, preview (+4 more)

### Community 4 - "Graph Type Definitions"
Cohesion: 0.17
Nodes (11): AccentTokens, AssetType, ConnectionSummary, FactRow, Finding, FindingSummary, GraphEdge, GraphNode (+3 more)

### Community 5 - "Safety & Legal Principles"
Cohesion: 0.20
Nodes (11): Engineering Judgment Principle, Git Workflow Policy (branch then PR, never commit to main), Container Sandboxing (deferred, tools run as host subprocess), Human-in-the-Loop Principle (no auto-escalation passive->active->exploit), Scope Enforcement Principle, CLAUDE.md Import Root, POST /targets/{targetId}/scan/passive (startPassiveScan), Docker Sandboxed Tool Execution (+3 more)

### Community 6 - "Graph SVG Component"
Cohesion: 0.25
Nodes (10): emit, lines, neighbors, nodesById, onEnter(), onKeydown(), onLeave(), props (+2 more)

### Community 7 - "Project Identity & API Contract"
Cohesion: 0.27
Nodes (10): Scope Per Branch Rule (one branch = one concern), Ivy (AI-orchestrated OSINT/recon tool), Ivy Graph Canvas App Entry (Vite HTML entry point), LlmConfig / LlmConfigUpdate Schema (api_key write-only, never echoed), Ivy API OpenAPI Spec (v0.1.0), Ivy API Contract (Frontend<->Backend), utoipa Spec-from-Rust-Code Option (not yet adopted), Redocly Lint Config for openapi.yaml (+2 more)

### Community 8 - "API Client Functions"
Cohesion: 0.38
Nodes (9): createTarget(), getLlmConfig(), getScanStatus(), getTarget(), getTargetGraph(), request(), startActiveScan(), startPassiveScan() (+1 more)

### Community 9 - "Graph Canvas View"
Cohesion: 0.20
Nodes (8): activeTab, connections, facts, findings, hoveredId, selectedId, selectedNode, selection

### Community 10 - "System Architecture & MVP Status"
Cohesion: 0.22
Nodes (9): v1/MVP Status (all stub, no Postgres/Neo4j/MCP client wiring yet), AI Engine / LLM Orchestrator, Backend: Rust + Axum, Frontend: Vue 3 + Vite + TypeScript, MCP (Model Context Protocol) Server, Neo4j Graph Database, Redis Cache Layer, System Architecture (Frontend / AI Orchestration / Tool Execution / Data layers) (+1 more)

### Community 11 - "Recon Tools & MCP Registry"
Cohesion: 0.36
Nodes (8): MCP Tool Scope Rule (don't wrap tools outside MVP roadmap), ToolExecution Schema (tool_id references MCP tool registry), Active Reconnaissance Tools (RustScan, Nmap, httpx, Nuclei, etc.), Development Roadmap (Phase 1-4, §11), Phase 4: Exploitation (Optional, explicit user consent required), ivy_amass MCP Tool (example schema), MCP Tools Registry (ivy_whois, ivy_dns, ivy_amass, ... ivy_hydra), Passive Reconnaissance Tools (WHOIS, DNS, Amass, Subfinder, crt.sh, etc.)

### Community 12 - "Scan API Types"
Cohesion: 0.29
Nodes (6): ScanPhase, ScanSession, ScanStatus, ScanStatusResponse, ToolExecution, ToolStatus

### Community 13 - "Architecture Decisions"
Cohesion: 0.33
Nodes (6): Provider/Plugin Abstraction Rule (no generic trait until 2nd provider exists), Exploitation Module (deferred, planned as separate plugin with double-consent), Finalized Architecture Decisions (§14), Exploit Module as Separate Opt-in Plugin Decision, LLM Provider-Agnostic Decision (Gemini/Claude/Ollama via abstraction layer), Self-Hosted Deployment Decision

### Community 14 - "Graph Mock Data"
Cohesion: 0.33
Nodes (4): ACCENTS, EDGES, NODES, SEVERITY

### Community 15 - "Node Inspector Component"
Cohesion: 0.40
Nodes (3): emit, props, tabs

### Community 16 - "Scope Config & App Flow"
Cohesion: 0.67
Nodes (4): Ivy Core Flow (8 steps: input domain -> plan -> passive -> consent -> active -> graph), ScopeConfig Schema (intensity only, in/out-of-scope not yet modeled), ScopeConfig In/Out-of-Scope Field Gap (unresolved), Ivy Application Flow (Phase 0-4)

### Community 17 - "Graph API Types"
Cohesion: 0.50
Nodes (3): GraphData, GraphEdge, GraphNode

### Community 18 - "LLM Config Types"
Cohesion: 0.50
Nodes (3): LlmConfig, LlmConfigUpdate, LlmProvider

### Community 19 - "Target API Types"
Cohesion: 0.50
Nodes (3): Intensity, ScopeConfig, Target

## Ambiguous Edges - Review These
- `Scope Per Branch Rule (one branch = one concern)` → `Ivy (AI-orchestrated OSINT/recon tool)`  [AMBIGUOUS]
  .agents/rules/git-workflow.md · relation: references

## Knowledge Gaps
- **132 isolated node(s):** `name`, `version`, `private`, `type`, `dev` (+127 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **21 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **What is the exact relationship between `Scope Per Branch Rule (one branch = one concern)` and `Ivy (AI-orchestrated OSINT/recon tool)`?**
  _Edge tagged AMBIGUOUS (relation: references) - confidence is low._
- **Why does `devDependencies` connect `Frontend Build Tooling` to `Frontend Package Manifest`?**
  _High betweenness centrality (0.010) - this node is a cross-community bridge._
- **Why does `Ivy (AI-orchestrated OSINT/recon tool)` connect `Project Identity & API Contract` to `Safety & Legal Principles`?**
  _High betweenness centrality (0.004) - this node is a cross-community bridge._
- **What connects `name`, `version`, `private` to the rest of the system?**
  _132 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `TypeScript App Config` be split into smaller, more focused modules?**
  _Cohesion score 0.07692307692307693 - nodes in this community are weakly interconnected._
- **Should `Frontend Build Tooling` be split into smaller, more focused modules?**
  _Cohesion score 0.13333333333333333 - nodes in this community are weakly interconnected._
- **Should `TypeScript Node Config` be split into smaller, more focused modules?**
  _Cohesion score 0.13333333333333333 - nodes in this community are weakly interconnected._