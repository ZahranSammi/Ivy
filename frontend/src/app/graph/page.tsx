"use client";

import { useEffect, useRef, useState } from 'react';
import dynamic from 'next/dynamic';
import { Download, Info, Server, Activity, ArrowRight, X, Network, AlertTriangle } from 'lucide-react';

// CytoscapeJS throws errors if imported on the server side
const CytoscapeComponent = dynamic(() => import('react-cytoscapejs'), { ssr: false });

const DUMMY_ELEMENTS = [
  // Nodes
  { data: { id: 'domain', label: 'example.com', type: 'domain' } },
  { data: { id: 'ip1', label: '93.184.216.34', type: 'ip' } },
  { data: { id: 'port80', label: 'Port 80 (HTTP)', type: 'port' } },
  { data: { id: 'port22', label: 'Port 22 (SSH)', type: 'port' } },
  { data: { id: 'vuln1', label: 'CVE-2021-1234', type: 'vuln' } },
  
  // Edges
  { data: { source: 'domain', target: 'ip1', label: 'resolves_to' } },
  { data: { source: 'ip1', target: 'port80', label: 'has_port' } },
  { data: { source: 'ip1', target: 'port22', label: 'has_port' } },
  { data: { source: 'port80', target: 'vuln1', label: 'vulnerable_to' } },
];

export default function GraphView() {
  const [selectedNode, setSelectedNode] = useState<any>(null);
  
  return (
    <div className="flex flex-col h-[calc(100vh-6rem)]">
      <div className="flex justify-between items-center mb-4">
        <div>
          <h1 className="text-2xl font-bold text-slate-800 tracking-tight">Graph Visualization</h1>
          <p className="text-slate-500 text-sm">Interactive Neo4j graph representation of active recon targets.</p>
        </div>
        <div className="flex gap-3">
          <button className="flex items-center gap-2 bg-white border border-slate-300 px-4 py-2 rounded-md text-sm font-medium text-slate-700 hover:bg-slate-50 transition-colors shadow-sm">
            <Download className="w-4 h-4" /> Export JSON
          </button>
          <button className="flex items-center gap-2 bg-indigo-600 text-white px-4 py-2 rounded-md text-sm font-medium hover:bg-indigo-700 transition-colors shadow-sm">
            <Download className="w-4 h-4" /> Export Report (PDF)
          </button>
        </div>
      </div>

      <div className="flex flex-1 gap-6 min-h-0">
        {/* Graph Canvas */}
        <div className="flex-1 bg-white border border-slate-200 rounded-xl shadow-sm overflow-hidden relative">
          <CytoscapeComponent 
            elements={DUMMY_ELEMENTS} 
            style={{ width: '100%', height: '100%' }}
            layout={{ name: 'cose', padding: 50 }}
            stylesheet={[
              {
                selector: 'node',
                style: {
                  'background-color': '#10b981', // emerald
                  'label': 'data(label)',
                  'color': '#1e293b', // slate 800
                  'text-valign': 'bottom',
                  'text-halign': 'center',
                  'text-margin-y': 5,
                  'font-size': '12px',
                  'font-weight': 'bold',
                }
              },
              {
                selector: 'node[type="domain"]',
                style: { 'background-color': '#3b82f6', 'width': 40, 'height': 40 } // blue
              },
              {
                selector: 'node[type="vuln"]',
                style: { 'background-color': '#ef4444', 'shape': 'diamond' } // red
              },
              {
                selector: 'edge',
                style: {
                  'width': 2,
                  'line-color': '#cbd5e1', // slate 300
                  'target-arrow-color': '#cbd5e1',
                  'target-arrow-shape': 'triangle',
                  'curve-style': 'bezier',
                  'label': 'data(label)',
                  'font-size': '10px',
                  'color': '#64748b', // slate 500
                  'text-rotation': 'autorotate'
                }
              }
            ]}
            cy={(cy) => {
              cy.on('tap', 'node', (evt) => {
                setSelectedNode(evt.target.data());
              });
              cy.on('tap', (evt) => {
                if (evt.target === cy) {
                  setSelectedNode(null);
                }
              });
            }}
          />
        </div>

        {/* Node Inspector Panel */}
        {selectedNode && (
          <div className="w-80 bg-white border border-slate-200 rounded-xl shadow-sm flex flex-col animate-in slide-in-from-right-4 duration-200">
            <div className="p-4 border-b border-slate-200 flex justify-between items-center bg-slate-50 rounded-t-xl">
              <h3 className="font-semibold text-slate-800 flex items-center gap-2">
                <Info className="w-4 h-4 text-slate-500" /> Inspector
              </h3>
              <button 
                onClick={() => setSelectedNode(null)}
                className="text-slate-400 hover:text-slate-600"
              >
                <X className="w-4 h-4" />
              </button>
            </div>
            
            <div className="p-5 flex-1 overflow-auto">
              <div className="mb-6 text-center">
                <div className="w-16 h-16 rounded-full bg-slate-100 flex items-center justify-center mx-auto mb-3 border-2 border-slate-200">
                  {selectedNode.type === 'domain' && <Server className="w-8 h-8 text-blue-500" />}
                  {selectedNode.type === 'ip' && <Network className="w-8 h-8 text-emerald-500" />}
                  {selectedNode.type === 'port' && <Activity className="w-8 h-8 text-indigo-500" />}
                  {selectedNode.type === 'vuln' && <AlertTriangle className="w-8 h-8 text-red-500" />}
                </div>
                <h4 className="text-lg font-bold text-slate-800">{selectedNode.label}</h4>
                <span className="inline-block px-2 py-1 mt-1 bg-slate-100 text-slate-600 text-xs font-semibold uppercase tracking-wider rounded">
                  {selectedNode.type}
                </span>
              </div>
              
              <div className="space-y-4">
                <div>
                  <h5 className="text-xs font-semibold text-slate-400 uppercase tracking-wider mb-2">Properties</h5>
                  <div className="bg-slate-50 border border-slate-200 rounded-md p-3 text-sm space-y-2">
                    <div className="flex justify-between">
                      <span className="text-slate-500">ID</span>
                      <span className="font-mono text-slate-800">{selectedNode.id}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-slate-500">Discovered</span>
                      <span className="text-slate-800">Just now</span>
                    </div>
                  </div>
                </div>
                
                {selectedNode.type === 'vuln' && (
                  <div>
                    <h5 className="text-xs font-semibold text-slate-400 uppercase tracking-wider mb-2">Details</h5>
                    <div className="bg-red-50 border border-red-100 rounded-md p-3 text-sm text-red-800">
                      High severity vulnerability found. Exploitation could lead to unauthorized access.
                    </div>
                  </div>
                )}
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
