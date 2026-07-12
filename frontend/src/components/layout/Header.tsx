export default function Header() {
  return (
    <header className="h-14 border-b border-slate-200 bg-white flex items-center justify-between px-6">
      <div className="flex items-center gap-4">
        <h2 className="font-semibold text-slate-800">example.com</h2>
        <span className="px-2 py-0.5 rounded-full bg-emerald-100 text-emerald-700 text-xs font-medium">Active</span>
      </div>
      
      <div className="flex items-center gap-4">
        <button className="bg-red-500 hover:bg-red-600 text-white px-4 py-1.5 rounded-md text-sm font-medium transition-colors shadow-sm flex items-center gap-2">
          <span>🛑</span> Kill Switch
        </button>
      </div>
    </header>
  );
}
