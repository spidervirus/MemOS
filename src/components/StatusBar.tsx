import { Database, RefreshCw } from "lucide-react";

interface StatusBarProps {
  count: number;
  isIndexing: boolean;
}

export default function StatusBar({ count, isIndexing }: StatusBarProps) {
  return (
    <footer className="border-t border-white/5 p-3 px-6 flex items-center justify-between bg-surface/80 backdrop-blur-md text-[11px] text-white/40 uppercase font-bold tracking-widest">
      <div className="flex items-center gap-4">
        <div className="flex items-center gap-1.5">
          <Database className="w-3 h-3" />
          <span>{count} Documents Indexed</span>
        </div>
        {isIndexing && (
          <div className="flex items-center gap-1.5 text-accent animate-pulse">
            <RefreshCw className="w-3 h-3 animate-spin" />
            <span>Indexing...</span>
          </div>
        )}
      </div>
      <div>
        <span>MemOS Solo Alpha v0.1.0</span>
      </div>
    </footer>
  );
}
