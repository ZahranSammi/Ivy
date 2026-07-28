export type Intensity = "passive" | "normal" | "aggressive";

export interface ScopeConfig {
  intensity: Intensity;
}

export interface Target {
  id: string;
  domain: string;
  scope_config: ScopeConfig;
  created_at: string;
}
