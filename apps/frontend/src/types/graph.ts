export type AssetType = "domain" | "network" | "service" | "vuln" | "other";

export type Severity = "critical" | "high" | "medium" | "low" | "info";

export interface Finding {
  name: string;
  cve: string | null;
  sev: Severity;
  cvss: string;
}

export interface GraphNode {
  id: string;
  label: string;
  type: AssetType;
  value: string;
  x: number;
  y: number;
  r: number;
  active?: boolean;
  tool: string;
  seen: string;
  status: string;
  desc: string;
  sev?: Severity;
  findings?: Finding[];
}

/** [fromId, toId, active?] — active edges (1) get the highlighted "live scan" styling. */
export type GraphEdge = readonly [string, string, (0 | 1)?];

export interface AccentTokens {
  c: string;
  t: string;
  l: string;
}

export type InspectorTab = "overview" | "findings" | "connections";

export interface FindingSummary {
  name: string;
  cve: string;
  sevLabel: string;
  cvss: string;
  color: string;
  colorText: string;
}

export interface ConnectionSummary {
  id: string;
  label: string;
  typeLabel: string;
  color: string;
}

export interface FactRow {
  k: string;
  v: string;
}

export interface NodeSelection {
  node: GraphNode;
  typeLabel: string;
  accentColor: string;
  accentText: string;
  statusDotColor: string;
  connections: ConnectionSummary[];
  findings: FindingSummary[];
  facts: FactRow[];
}
