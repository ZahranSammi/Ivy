export default function Home() {
  return (
    <div className="h-full flex flex-col">
      <div className="mb-6">
        <h1 className="text-2xl font-bold text-slate-900">Dashboard</h1>
        <p className="text-slate-500">Welcome to Ivy OSINT Platform.</p>
      </div>
      
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
        <div className="bg-white p-6 rounded-xl border border-slate-200 shadow-sm">
          <h3 className="text-slate-500 text-sm font-medium mb-1">Total Nodes</h3>
          <p className="text-3xl font-bold text-slate-800">1,204</p>
        </div>
        <div className="bg-white p-6 rounded-xl border border-slate-200 shadow-sm">
          <h3 className="text-slate-500 text-sm font-medium mb-1">Active Scans</h3>
          <p className="text-3xl font-bold text-slate-800">1</p>
        </div>
        <div className="bg-white p-6 rounded-xl border border-slate-200 shadow-sm">
          <h3 className="text-slate-500 text-sm font-medium mb-1">Vulnerabilities</h3>
          <p className="text-3xl font-bold text-red-600">3</p>
        </div>
      </div>
      
      <div className="flex-1 bg-white rounded-xl border border-slate-200 shadow-sm flex items-center justify-center text-slate-400 flex-col gap-4">
        <span className="text-4xl">🕸️</span>
        <p>Graph visualization will be rendered here</p>
      </div>
    </div>
  );
}
