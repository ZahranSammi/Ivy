"use client";

import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { LayoutDashboard, Network, Settings, ShieldAlert } from 'lucide-react';

export default function Sidebar() {
  const pathname = usePathname();

  const getLinkClasses = (path: string) => {
    const isActive = pathname === path;
    return `flex items-center gap-3 px-3 py-2 rounded-md text-sm font-medium transition-colors ${
      isActive
        ? 'bg-slate-900 text-emerald-400 border border-slate-800'
        : 'hover:bg-slate-900 text-slate-300 border border-transparent'
    }`;
  };

  return (
    <aside className="w-64 bg-slate-950 text-slate-300 border-r border-slate-800 h-screen flex flex-col p-4">
      <div className="flex items-center gap-3 mb-8 px-2">
        <ShieldAlert className="w-7 h-7 text-emerald-500" />
        <h1 className="text-xl font-bold tracking-tight text-white">Ivy OSINT</h1>
      </div>
      
      <div className="flex-1 space-y-6">
        <div>
          <h2 className="text-xs uppercase text-slate-500 font-semibold mb-3 tracking-wider px-2">Menu</h2>
          <ul className="space-y-1">
            <li>
              <Link href="/" className={getLinkClasses('/')}>
                <LayoutDashboard className="w-4 h-4" /> Dashboard
              </Link>
            </li>
            <li>
              <Link href="/graph" className={getLinkClasses('/graph')}>
                <Network className="w-4 h-4" /> Graph View
              </Link>
            </li>
            <li>
              <Link href="/settings" className={getLinkClasses('/settings')}>
                <Settings className="w-4 h-4" /> Settings
              </Link>
            </li>
          </ul>
        </div>

        <div>
          <h2 className="text-xs uppercase text-slate-500 font-semibold mb-3 tracking-wider px-2">Active Target</h2>
          <div className="px-3 py-2 bg-slate-900/50 border border-slate-800 rounded-md text-sm font-mono text-slate-400">
            example.com
          </div>
        </div>
      </div>
      
      <div className="mt-auto">
        <h2 className="text-xs uppercase text-slate-500 font-semibold mb-3 tracking-wider px-2">Scan Progress</h2>
        <div className="bg-slate-900 border border-slate-800 p-3 rounded-md">
          <div className="flex justify-between text-xs mb-2">
            <span className="text-slate-300">nmap (running)</span>
            <span className="text-emerald-400">65%</span>
          </div>
          <div className="w-full bg-slate-800 h-1.5 rounded-full overflow-hidden">
            <div className="bg-emerald-500 h-full w-[65%] shadow-[0_0_10px_rgba(16,185,129,0.5)]"></div>
          </div>
        </div>
      </div>
    </aside>
  );
}

