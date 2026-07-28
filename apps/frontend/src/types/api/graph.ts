// Raw Neo4j-shaped graph response from the backend — distinct from
// `@/types/graph`, which is the Graph Canvas view model (fixed layout,
// design-driven categories). A future layout step maps one to the other.

export interface GraphNode {
  id: string;
  labels: string[];
  properties: Record<string, unknown>;
}

export interface GraphEdge {
  id: string;
  source_id: string;
  target_id: string;
  type: string;
  properties: Record<string, unknown>;
}

export interface GraphData {
  nodes: GraphNode[];
  edges: GraphEdge[];
}
