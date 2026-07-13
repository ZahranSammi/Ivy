import { Shield, Bell, User } from 'lucide-react';

export default function Header() {
  return (
    <header className="h-16 border-b border-slate-200 bg-white flex items-center justify-between px-8">
      <div className="flex items-center gap-4">
        <h2 className="text-lg font-semibold text-slate-800 tracking-tight">example.com</h2>
        <span className="px-2.5 py-1 rounded-full bg-emerald-100 text-emerald-700 text-xs font-semibold tracking-wide flex items-center gap-1.5">
          <span className="w-1.5 h-1.5 bg-emerald-500 rounded-full animate-pulse"></span>
          SCAN ACTIVE
        </span>
      </div>
      
      <div className="flex items-center gap-6">
        <button className="text-slate-400 hover:text-slate-600 transition-colors">
          <Bell className="w-5 h-5" />
        </button>
        <button className="bg-red-50 hover:bg-red-100 text-red-600 px-4 py-2 rounded-md text-sm font-semibold transition-colors flex items-center gap-2 border border-red-200">
          <Shield className="w-4 h-4" /> Kill Switch
        </button>
        <div className="w-8 h-8 rounded-full bg-slate-100 border border-slate-200 flex items-center justify-center text-slate-600">
          <User className="w-4 h-4" />
        </div>
      </div>
    </header>
  );
}
