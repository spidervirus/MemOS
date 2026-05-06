import { FileText, ExternalLink } from "lucide-react";
import { SearchResult } from "../App";

interface ResultsListProps {
  results: SearchResult[];
  query: string;
}

export default function ResultsList({ results, query }: ResultsListProps) {
  const highlight = (text: string, q: string) => {
    if (!q) return text;
    const parts = text.split(new RegExp(`(${q})`, "gi"));
    return (
      <>
        {parts.map((part, i) => 
          part.toLowerCase() === q.toLowerCase() ? (
            <mark key={i} className="bg-accent/30 text-white rounded px-0.5">
              {part}
            </mark>
          ) : (
            part
          )
        )}
      </>
    );
  };

  return (
    <div className="flex flex-col gap-4 pb-12">
      <div className="flex items-center justify-between">
        <h3 className="text-sm font-semibold text-white/40 uppercase tracking-wider">
          Top Memories
        </h3>
        <span className="text-xs text-white/20">{results.length} results</span>
      </div>
      
      {results.map((result) => (
        <div 
          key={result.id} 
          className="group p-5 bg-surface border border-white/5 rounded-2xl hover:border-white/20 transition-all hover:shadow-xl hover:shadow-black/20"
        >
          <div className="flex items-start justify-between gap-4 mb-3">
            <div className="flex items-center gap-2">
              <FileText className="w-4 h-4 text-accent/60" />
              <span className="text-sm font-medium text-white/60 truncate max-w-[300px]">
                {result.source.split("/").pop()}
              </span>
            </div>
            <div className="text-[10px] px-2 py-1 bg-white/5 rounded uppercase font-bold text-white/40">
              Score: {(1 - result.score).toFixed(2)}
            </div>
          </div>
          
          <p className="text-white/80 leading-relaxed text-sm">
            {highlight(result.content.length > 300 ? result.content.slice(0, 300) + "..." : result.content, query)}
          </p>

          <div className="mt-4 pt-4 border-t border-white/5 flex items-center justify-between opacity-0 group-hover:opacity-100 transition-opacity">
            <div className="text-[10px] text-white/20 font-mono truncate max-w-[400px]">
              {result.source}
            </div>
            <button className="text-accent text-xs font-medium flex items-center gap-1 hover:underline">
              Open File <ExternalLink className="w-3 h-3" />
            </button>
          </div>
        </div>
      ))}
    </div>
  );
}
