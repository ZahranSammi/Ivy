import React, { useState, useEffect, useRef } from 'react';
import cytoscape from 'cytoscape';
import { 
  Shield, 
  Terminal, 
  Activity, 
  Play, 
  CheckCircle, 
  XCircle, 
  Clock, 
  Database,
  Search,
  AlertTriangle
} from 'lucide-react';

export default function App() {
  const [domains, setDomains] = useState('target.com');
  const [ips, setIps] = useState('192.168.1.0/24');
  const [sessionLabel, setSessionLabel] = useState('bugbounty-target');
  const [scopeId, setScopeId] = useState('');
  
  const [scanDomain, setScanDomain] = useState('target.com');
  const [scanStatus, setScanStatus] = useState('');
  const [scanResult, setScanResult] = useState(null);
  
  const [approvals, setApprovals] = useState([]);
  const [logs, setLogs] = useState([]);
  
  const cyRef = useRef(null);
  const cyInstance = useRef(null);

  // Fetch approvals and logs periodically
  useEffect(() => {
    fetchData();
    const interval = setInterval(fetchData, 3000);
    return () => clearInterval(interval);
  }, []);

  const fetchData = async () => {
    try {
      const appResp = await fetch('http://localhost:5000/api/approvals');
      const appData = await appResp.json();
      setApprovals(appData.approvals || []);

      const logsResp = await fetch('http://localhost:5000/api/logs');
      const logsData = await logsResp.json();
      setLogs(logsData.logs || []);

      // Reload graph
      loadGraph();
    } catch (e) {
      console.error("Error fetching updates:", e);
    }
  };

  const loadGraph = async () => {
    try {
      const resp = await fetch('http://localhost:5000/api/graph');
      const data = await resp.json();
      
      if (!cyInstance.current && cyRef.current) {
        cyInstance.current = cytoscape({
          container: cyRef.current,
          style: [
            {
              selector: 'node',
              style: {
                'background-color': '#8a2be2',
                'label': 'data(label)',
                'color': '#fff',
                'font-size': '10px',
                'text-valign': 'center',
                'text-halign': 'round-rectangle',
                'width': '35px',
                'height': '35px'
              }
            },
            {
              selector: 'node[type="IPAddress"]',
              style: { 'background-color': '#00f2fe' }
            },
            {
              selector: 'node[type="Vulnerability"]',
              style: { 
                'background-color': '#ff1744',
                'shape': 'triangle',
                'width': '45px',
                'height': '45px'
              }
            },
            {
              selector: 'node[type="Service"]',
              style: { 'background-color': '#00e676' }
            },
            {
              selector: 'edge',
              style: {
                'width': 2,
                'line-color': '#2e2e4a',
                'target-arrow-color': '#2e2e4a',
                'target-arrow-shape': 'triangle',
                'curve-style': 'bezier'
              }
            }
          ],
          elements: []
        });
      }

      if (cyInstance.current) {
        const elements = [];
        // Map nodes
        (data.nodes || []).forEach(n => {
          elements.push({
            data: { 
              id: n.id, 
              label: n.properties.name || n.properties.ip || n.properties.type || 'Node',
              type: n.labels[0]
            }
          });
        });
        // Map edges
        (data.edges || []).forEach(e => {
          elements.push({
            data: { 
              id: e.id, 
              source: e.start, 
              target: e.end 
            }
          });
        });

        cyInstance.current.elements().remove();
        cyInstance.current.add(elements);
        cyInstance.current.layout({ name: 'cose' }).run();
      }
    } catch (e) {
      console.error(e);
    }
  };

  const handleSetScope = async () => {
    try {
      const resp = await fetch('http://localhost:5000/api/scope', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          domains: domains.split(',').map(d => d.trim()),
          ips: ips.split(',').map(ip => ip.trim()),
          session_label: sessionLabel
        })
      });
      const data = await resp.json();
      if (data.scope_id) {
        setScopeId(data.scope_id);
        alert(`Scope set successfully! Scope ID: ${data.scope_id}`);
      } else {
        alert(`Error: ${JSON.stringify(data.details)}`);
      }
    } catch (e) {
      alert("Error setting scope");
    }
  };

  const handleRunRecon = async () => {
    if (!scopeId) {
      alert("Please set scope first");
      return;
    }
    setScanStatus('RUNNING');
    try {
      const resp = await fetch('http://localhost:5000/api/passive', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          scope_id: scopeId,
          domain: scanDomain
        })
      });
      const data = await resp.json();
      setScanStatus('COMPLETED');
      setScanResult(data);
      loadGraph();
    } catch (e) {
      setScanStatus('FAILED');
      alert("Scan failed");
    }
  };

  const handleApprove = async (reqId, approve) => {
    try {
      const resp = await fetch('http://localhost:5000/api/approve', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          approval_request_id: reqId,
          approve
        })
      });
      const data = await resp.json();
      alert(`Approval response: ${data.status}`);
      fetchData();
    } catch (e) {
      alert("Error processing approval");
    }
  };

  return (
    <div className="app-container">
      <div className="sidebar">
        <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
          <Shield size={28} color="#8a2be2" />
          <h2 style={{ margin: 0, fontSize: '20px', letterSpacing: '1px' }}>GraphCon</h2>
        </div>

        {/* Scope Panel */}
        <div className="glass-card">
          <h3 style={{ margin: '0 0 15px 0', fontSize: '15px', color: '#00f2fe' }}>1. Scope Configuration</h3>
          <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
            <label style={{ fontSize: '12px', color: '#b0b0cc' }}>Target Domains (comma sep)</label>
            <input 
              type="text" 
              value={domains} 
              onChange={e => setDomains(e.target.value)} 
              style={{ background: '#1a1a2e', border: '1px solid #2e2e4a', padding: '8px', color: '#fff', borderRadius: '4px' }}
            />
            <label style={{ fontSize: '12px', color: '#b0b0cc' }}>Target IPs (comma sep)</label>
            <input 
              type="text" 
              value={ips} 
              onChange={e => setIps(e.target.value)} 
              style={{ background: '#1a1a2e', border: '1px solid #2e2e4a', padding: '8px', color: '#fff', borderRadius: '4px' }}
            />
            <button className="btn-primary" onClick={handleSetScope}>Initialize Scope</button>
          </div>
        </div>

        {/* Recon Panel */}
        <div className="glass-card">
          <h3 style={{ margin: '0 0 15px 0', fontSize: '15px', color: '#00f2fe' }}>2. Passive Recon</h3>
          <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
            <input 
              type="text" 
              value={scanDomain} 
              onChange={e => setScanDomain(e.target.value)} 
              placeholder="target.com"
              style={{ background: '#1a1a2e', border: '1px solid #2e2e4a', padding: '8px', color: '#fff', borderRadius: '4px' }}
            />
            <button className="btn-primary" style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', gap: '8px' }} onClick={handleRunRecon}>
              <Play size={16} /> Run Recon
            </button>
            {scanStatus && (
              <div style={{ fontSize: '13px', display: 'flex', alignItems: 'center', gap: '6px' }}>
                <Activity size={14} className={scanStatus === 'RUNNING' ? 'spin' : ''} />
                Status: <span style={{ color: scanStatus === 'COMPLETED' ? '#00e676' : '#ff1744' }}>{scanStatus}</span>
              </div>
            )}
          </div>
        </div>

        {/* Approvals Panel */}
        <div className="glass-card" style={{ flexGrow: 1, overflowY: 'auto' }}>
          <h3 style={{ margin: '0 0 15px 0', fontSize: '15px', color: '#00f2fe' }}>3. Pending Approvals</h3>
          {approvals.length === 0 ? (
            <p style={{ fontSize: '12px', color: '#b0b0cc' }}>No pending active scans.</p>
          ) : (
            approvals.map(app => (
              <div key={app.approval_request_id} style={{ border: '1px solid #2e2e4a', padding: '10px', borderRadius: '6px', marginBottom: '10px', background: '#1a1a2e' }}>
                <div style={{ fontSize: '12px', fontWeight: 'bold' }}>Req ID: {app.approval_request_id.slice(0, 8)}</div>
                <div style={{ fontSize: '11px', color: '#b0b0cc' }}>Targets: {JSON.parse(app.targets_json).join(', ')}</div>
                <div style={{ fontSize: '11px', color: '#b0b0cc' }}>Types: {JSON.parse(app.test_types_json).join(', ')}</div>
                <div style={{ display: 'flex', gap: '5px', marginTop: '10px' }}>
                  <button className="btn-primary" style={{ padding: '4px 8px', fontSize: '11px', background: '#00e676' }} onClick={() => handleApprove(app.approval_request_id, true)}>Approve</button>
                  <button className="btn-secondary" style={{ padding: '4px 8px', fontSize: '11px', color: '#ff1744' }} onClick={() => handleApprove(app.approval_request_id, false)}>Reject</button>
                </div>
              </div>
            ))
          )}
        </div>
      </div>

      <div className="main-content">
        <div className="top-bar">
          <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
            <Activity size={20} color="#00f2fe" />
            <h1 style={{ margin: 0, fontSize: '18px', fontWeight: 600 }}>Visual Surface Graph</h1>
          </div>
          <div style={{ display: 'flex', alignItems: 'center', gap: '20px' }}>
            <div style={{ fontSize: '13px', color: '#b0b0cc' }}>Audit Logs: <span style={{ color: '#fff' }}>{logs.length} entries</span></div>
          </div>
        </div>

        <div className="graph-viewport">
          <div id="cy" ref={cyRef}></div>
        </div>
      </div>
    </div>
  );
}
