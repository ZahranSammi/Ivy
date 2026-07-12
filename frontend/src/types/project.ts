export interface Project {
  id: string;
  name: string;
  description?: string;
  target_domains: string[];
  status: 'created' | 'active' | 'archived';
  created_at: string;
  updated_at: string;
}
