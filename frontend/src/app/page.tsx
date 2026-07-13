import { Search, Terminal, AlertTriangle, ShieldCheck } from 'lucide-react';

export default function Dashboard() {
  return (
    <div className="max-w-6xl mx-auto space-y-6">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-slate-800 tracking-tight">Overview</h1>
        <p className="text-slate-500 mt-1">Start a new scan or monitor active ones.</p>
      </div>

      {/* Target Input Section */}
      <div className="bg-white p-6 rounded-xl border border-slate-200 shadow-sm">
        <h2 className="text-lg font-semibold text-slate-800 mb-4 flex items-center gap-2">
          <Search className="w-5 h-5 text-emerald-500" /> New Target
        </h2>
        <div className="flex gap-4">
          <input
            type="text"
            placeholder="example.com"
            className="flex-1 px-4 py-2 border border-slate-300 rounded-md focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent font-mono"
            defaultValue="example.com"
          />
          <button className="bg-slate-900 hover:bg-slate-800 text-white px-6 py-2 rounded-md font-medium transition-colors">
            Start Scan
          </button>
        </div>
        <p className="text-xs text-slate-500 mt-3 flex items-center gap-1">
          <ShieldCheck className="w-4 h-4 text-slate-400" /> 
          By starting a scan, you agree to the legal disclaimer and confirm authorization.
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Left Column: Stats & Progress */}
        <div className="space-y-6">
          <div className="bg-white p-6 rounded-xl border border-slate-200 shadow-sm">
            <h2 className="text-lg font-semibold text-slate-800 mb-4">Scan Progress</h2>
            
            <div className="space-y-4">
              <div>
                <div className="flex justify-between text-sm mb-1">
                  <span className="font-medium text-slate-700">Phase 1: Subdomain Enumeration</span>
                  <span className="text-emerald-600 font-medium">100%</span>
                </div>
                <div className="w-full bg-slate-100 h-2 rounded-full overflow-hidden">
                  <div className="bg-emerald-500 h-full w-full"></div>
                </div>
              </div>
              
              <div>
                <div className="flex justify-between text-sm mb-1">
                  <span className="font-medium text-slate-700">Phase 2: Port Scanning (nmap)</span>
                  <span className="text-blue-600 font-medium">65%</span>
                </div>
                <div className="w-full bg-slate-100 h-2 rounded-full overflow-hidden">
                  <div className="bg-blue-500 h-full w-[65%]"></div>
                </div>
              </div>
              
              <div>
                <div className="flex justify-between text-sm mb-1">
                  <span className="font-medium text-slate-400">Phase 3: Vulnerability Scanning</span>
                  <span className="text-slate-400 font-medium">Pending</span>
                </div>
                <div className="w-full bg-slate-100 h-2 rounded-full overflow-hidden">
                  <div className="bg-slate-300 h-full w-0"></div>
                </div>
              </div>
            </div>
          </div>

          <div className="bg-white p-6 rounded-xl border border-slate-200 shadow-sm">
             <h2 className="text-lg font-semibold text-slate-800 mb-4">Quick Stats</h2>
             <div className="grid grid-cols-2 gap-4">
               <div className="p-4 bg-slate-50 rounded-lg border border-slate-100">
                 <div className="text-sm text-slate-500 mb-1">Subdomains</div>
                 <div className="text-2xl font-bold text-slate-800">42</div>
               </div>
               <div className="p-4 bg-slate-50 rounded-lg border border-slate-100">
                 <div className="text-sm text-slate-500 mb-1">Open Ports</div>
                 <div className="text-2xl font-bold text-slate-800">12</div>
               </div>
               <div className="p-4 bg-red-50 rounded-lg border border-red-100 col-span-2 flex items-center justify-between">
                 <div>
                   <div className="text-sm text-red-600 font-medium mb-1 flex items-center gap-1">
                     <AlertTriangle className="w-4 h-4" /> High Vulns
                   </div>
                   <div className="text-2xl font-bold text-red-700">3</div>
                 </div>
                 <button className="text-sm bg-red-100 text-red-700 px-3 py-1 rounded-md font-medium hover:bg-red-200 transition-colors">
                   View Graph
                 </button>
               </div>
             </div>
          </div>
        </div>

        {/* Right Column: Live Terminal */}
        <div className="lg:col-span-2 bg-slate-950 rounded-xl border border-slate-800 shadow-xl overflow-hidden flex flex-col">
          <div className="bg-slate-900 border-b border-slate-800 p-3 flex items-center gap-2">
            <Terminal className="w-4 h-4 text-slate-400" />
            <span className="text-sm font-medium text-slate-300">Live Execution Log (Mock WS)</span>
            <div className="ml-auto flex gap-2">
               <span className="w-3 h-3 rounded-full bg-red-500"></span>
               <span className="w-3 h-3 rounded-full bg-yellow-500"></span>
               <span className="w-3 h-3 rounded-full bg-emerald-500"></span>
            </div>
          </div>
          <div className="p-4 flex-1 overflow-auto font-mono text-xs md:text-sm text-slate-300 space-y-1.5 h-[500px]">
            <div className="text-emerald-400">[*] Starting Ivy AI Orchestrator...</div>
            <div>[+] Target configured: example.com</div>
            <div>[*] Checking scope enforcements... OK</div>
            <div className="text-blue-400">[*] Prompting LLM (Claude-3.5) for recon plan...</div>
            <div>[+] Plan received. Next tool: amass</div>
            <br />
            <div className="text-slate-500"># Executing: amass enum -d example.com</div>
            <div>[amass] Found subdomain: api.example.com</div>
            <div>[amass] Found subdomain: dev.example.com</div>
            <div>[amass] Found subdomain: admin.example.com</div>
            <div className="text-emerald-400">[+] Subdomain enum complete (24 nodes added to Neo4j)</div>
            <br />
            <div className="text-blue-400">[*] Re-evaluating with AI...</div>
            <div>[+] Next tool: nmap on discovered IPs</div>
            <div className="text-slate-500"># Executing: nmap -sV -p- 93.184.216.34</div>
            <div>[nmap] PORT   STATE SERVICE VERSION</div>
            <div>[nmap] 22/tcp open  ssh     OpenSSH 8.2p1</div>
            <div>[nmap] 80/tcp open  http    nginx 1.18.0</div>
            <div className="animate-pulse text-yellow-400">_</div>
          </div>
        </div>
      </div>
    </div>
  );
}
