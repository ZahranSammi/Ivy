import type { Intensity, Target } from "@/types/api/target";
import type { ScanSession, ScanStatusResponse } from "@/types/api/scan";
import type { GraphData } from "@/types/api/graph";
import type { LlmConfig, LlmConfigUpdate } from "@/types/api/llm";

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL ?? "http://localhost:3001";

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  const res = await fetch(`${API_BASE_URL}${path}`, {
    ...init,
    headers: { "Content-Type": "application/json", ...init?.headers },
  });

  if (!res.ok) {
    const body = await res.text().catch(() => "");
    throw new Error(`${init?.method ?? "GET"} ${path} failed: ${res.status} ${body}`);
  }

  if (res.status === 204) return undefined as T;
  return (await res.json()) as T;
}

export function createTarget(input: { domain: string; intensity: Intensity }): Promise<Target> {
  return request<Target>("/targets", {
    method: "POST",
    body: JSON.stringify({
      domain: input.domain,
      scope_config: { intensity: input.intensity },
    }),
  });
}

export function getTarget(id: string): Promise<Target> {
  return request<Target>(`/targets/${id}`);
}

export function startPassiveScan(targetId: string): Promise<ScanSession> {
  return request<ScanSession>(`/targets/${targetId}/scan/passive`, {
    method: "POST",
  });
}

export function startActiveScan(targetId: string): Promise<ScanSession> {
  return request<ScanSession>(`/targets/${targetId}/scan/active`, {
    method: "POST",
  });
}

export function getScanStatus(targetId: string): Promise<ScanStatusResponse> {
  return request<ScanStatusResponse>(`/targets/${targetId}/scan/status`);
}

export function getTargetGraph(targetId: string): Promise<GraphData> {
  return request<GraphData>(`/targets/${targetId}/graph`);
}

export function getLlmConfig(): Promise<LlmConfig> {
  return request<LlmConfig>("/settings/llm");
}

export function updateLlmConfig(input: LlmConfigUpdate): Promise<LlmConfig> {
  return request<LlmConfig>("/settings/llm", {
    method: "PUT",
    body: JSON.stringify(input),
  });
}
