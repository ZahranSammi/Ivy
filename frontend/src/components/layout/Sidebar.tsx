export default function Sidebar() {
  return (
    <aside className="w-64 bg-slate-900 text-white h-screen flex flex-col p-4">
      <div className="flex items-center gap-2 mb-8">
        <span className="text-2xl">🌿</span>
        <h1 className="text-xl font-bold tracking-tight">Ivy</h1>
      </div>
      
      <div className="flex-1">
        <h2 className="text-xs uppercase text-slate-400 font-semibold mb-3 tracking-wider">Projects</h2>
        <ul className="space-y-1">
          <li className="px-3 py-2 bg-slate-800 rounded-md text-sm font-medium">example.com</li>
          <li className="px-3 py-2 hover:bg-slate-800/50 rounded-md text-sm text-slate-300">test-target.org</li>
        </ul>
      </div>
      
      <div className="mt-auto">
        <h2 className="text-xs uppercase text-slate-400 font-semibold mb-3 tracking-wider">Active Scans</h2>
        <div className="bg-slate-800 p-3 rounded-md">
          <div className="flex justify-between text-xs mb-1">
            <span>amass</span>
            <span>40%</span>
          </div>
          <div className="w-full bg-slate-700 h-1.5 rounded-full overflow-hidden">
            <div className="bg-emerald-500 h-full w-[40%]"></div>
          </div>
        </div>
      </div>
    </aside>
  );
}
