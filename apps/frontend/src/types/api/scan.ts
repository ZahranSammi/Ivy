export type ScanPhase = "passive" | "active";
export type ScanStatus = "pending" | "running" | "completed" | "failed";
export type ToolStatus = "queued" | "running" | "completed" | "failed";

export interface ToolExecution {
  id: string;
  tool_id: string;
  status: ToolStatus;
  result_summary: Record<string, unknown> | null;
  error_message: string | null;
}

export interface ScanSession {
  id: string;
  phase: ScanPhase;
  status: ScanStatus;
  created_at: string;
  completed_at: string | null;
  tool_executions: ToolExecution[];
  // Assumption: once a passive session completes, the backend includes the
  // active tools it wants to run next here so the consent gate has
  // something concrete to show. Not specified in the SRS — reconcile once
  // the backend response shape is real.
  proposed_active_tools?: string[];
}

export interface ScanStatusResponse {
  sessions: ScanSession[];
}
