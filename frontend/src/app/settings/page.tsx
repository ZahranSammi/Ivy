"use client";

import { Save, Brain, CheckCircle2, Shield, Network } from 'lucide-react';
import { useState } from 'react';

export default function SettingsView() {
  const [llmProvider, setLlmProvider] = useState('claude');
  const [apiKey, setApiKey] = useState('sk-ant-api03-xxxxxxxxxxxxxxxxxxxxxxxxx');

  return (
    <div className="max-w-4xl mx-auto space-y-6">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-slate-800 tracking-tight">Settings</h1>
        <p className="text-slate-500 mt-1">Configure AI orchestrator and tool integrations.</p>
      </div>

      {/* AI Orchestrator Settings */}
      <div className="bg-white p-6 rounded-xl border border-slate-200 shadow-sm">
        <div className="flex items-center gap-2 mb-6">
          <Brain className="w-5 h-5 text-indigo-500" />
          <h2 className="text-lg font-semibold text-slate-800">AI Orchestrator (LLM)</h2>
        </div>
        
        <div className="space-y-5 max-w-2xl">
          <div>
            <label className="block text-sm font-medium text-slate-700 mb-1">Active Provider</label>
            <select 
              className="w-full px-4 py-2 bg-slate-50 border border-slate-300 rounded-md focus:outline-none focus:ring-2 focus:ring-emerald-500"
              value={llmProvider}
              onChange={(e) => setLlmProvider(e.target.value)}
            >
              <option value="claude">Anthropic Claude (Recommended)</option>
              <option value="gemini">Google Gemini</option>
              <option value="openai">OpenAI GPT-4</option>
              <option value="ollama">Ollama (Local/Self-hosted)</option>
            </select>
          </div>

          <div>
            <label className="block text-sm font-medium text-slate-700 mb-1">API Key</label>
            <input
              type="password"
              value={apiKey}
              onChange={(e) => setApiKey(e.target.value)}
              className="w-full px-4 py-2 bg-slate-50 border border-slate-300 rounded-md focus:outline-none focus:ring-2 focus:ring-emerald-500 font-mono"
            />
            <p className="text-xs text-slate-500 mt-1">Key is encrypted at rest (AES-256-GCM) via PostgreSQL backend.</p>
          </div>

          {llmProvider === 'ollama' && (
            <div>
              <label className="block text-sm font-medium text-slate-700 mb-1">Ollama Endpoint URL</label>
              <input
                type="text"
                placeholder="http://localhost:11434"
                className="w-full px-4 py-2 bg-slate-50 border border-slate-300 rounded-md focus:outline-none focus:ring-2 focus:ring-emerald-500 font-mono"
              />
            </div>
          )}

          <div className="pt-4 flex gap-3">
            <button className="flex items-center gap-2 bg-slate-900 text-white px-5 py-2 rounded-md font-medium hover:bg-slate-800 transition-colors">
              <Save className="w-4 h-4" /> Save Configuration
            </button>
            <button className="flex items-center gap-2 bg-emerald-50 text-emerald-700 border border-emerald-200 px-5 py-2 rounded-md font-medium hover:bg-emerald-100 transition-colors">
              <CheckCircle2 className="w-4 h-4" /> Test Connection
            </button>
          </div>
        </div>
      </div>

      {/* Sandboxing & Egress Settings */}
      <div className="bg-white p-6 rounded-xl border border-slate-200 shadow-sm opacity-75">
        <div className="flex items-center gap-2 mb-6">
          <Shield className="w-5 h-5 text-red-500" />
          <h2 className="text-lg font-semibold text-slate-800">Sandboxing & Egress Control</h2>
        </div>
        <p className="text-sm text-slate-600 mb-4">Configure Docker container limits and network access policies for security tools.</p>
        
        <div className="space-y-4 max-w-2xl">
          <div className="flex items-center justify-between p-4 bg-slate-50 border border-slate-200 rounded-lg">
            <div>
              <div className="font-medium text-slate-800">Strict Egress Filtering</div>
              <div className="text-xs text-slate-500">Only allow tools to communicate with the defined target scope.</div>
            </div>
            <div className="relative inline-block w-10 mr-2 align-middle select-none transition duration-200 ease-in">
              <input type="checkbox" name="toggle" id="toggle1" defaultChecked className="toggle-checkbox absolute block w-5 h-5 rounded-full bg-white border-4 appearance-none cursor-pointer border-emerald-500 translate-x-5" />
              <label htmlFor="toggle1" className="toggle-label block overflow-hidden h-5 rounded-full bg-emerald-500 cursor-pointer"></label>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
