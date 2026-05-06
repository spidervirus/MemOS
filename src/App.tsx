import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Database, Brain, Loader2 } from "lucide-react";
import SearchBar from "./components/SearchBar";
import ResultsList from "./components/ResultsList";
import StatusBar from "./components/StatusBar";
import FolderSelector from "./components/FolderSelector";

export interface SearchResult {
  id: string;
  content: string;
  source: string;
  score: number;
}

function App() {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchResult[]>([]);
  const [isSearching, setIsSearching] = useState(false);
  const [indexedCount, setIndexedCount] = useState(0);
  const [isIndexing, setIsIndexing] = useState(false);

  const fetchCount = async () => {
    try {
      const count = await invoke<number>("get_indexed_count");
      setIndexedCount(count);
    } catch (e) {
      console.error(e);
    }
  };

  useEffect(() => {
    fetchCount();
    const interval = setInterval(fetchCount, 5000);
    return () => clearInterval(interval);
  }, []);

  const handleSearch = async (q: string) => {
    if (!q.trim()) return;
    setQuery(q);
    setIsSearching(true);
    try {
      const res = await invoke<SearchResult[]>("search_memories", { query: q, topK: 10 });
      setResults(res);
    } catch (e) {
      console.error(e);
    } finally {
      setIsSearching(false);
    }
  };

  const handleFolderSelect = async (path: string) => {
    setIsIndexing(true);
    try {
      await invoke("start_indexing", { path });
      // We don't wait for indexing to finish as it's backgrounded
    } catch (e) {
      console.error(e);
    } finally {
      // Small delay to show feedback
      setTimeout(() => setIsIndexing(false), 2000);
    }
  };

  return (
    <div className="min-h-screen bg-background text-white flex flex-col">
      {/* Header */}
      <header className="border-b border-white/5 p-4 flex items-center justify-between bg-surface/50 backdrop-blur-md sticky top-0 z-10">
        <div className="flex items-center gap-2">
          <div className="p-2 bg-accent/10 rounded-lg">
            <Brain className="w-6 h-6 text-accent" />
          </div>
          <h1 className="text-xl font-bold tracking-tight">MemOS <span className="text-white/40 font-normal">Solo</span></h1>
        </div>
        <div className="flex items-center gap-4">
          <FolderSelector onSelect={handleFolderSelect} />
        </div>
      </header>

      {/* Main Content */}
      <main className="flex-1 max-w-4xl mx-auto w-full p-6 flex flex-col gap-8">
        <div className="flex flex-col gap-4 mt-8">
          <h2 className="text-3xl font-bold text-center bg-gradient-to-r from-white to-white/60 bg-clip-text text-transparent">
            Search your second brain
          </h2>
          <SearchBar onSearch={handleSearch} isLoading={isSearching} />
        </div>

        <div className="flex-1">
          {isSearching ? (
            <div className="flex flex-col items-center justify-center py-20 gap-4">
              <Loader2 className="w-8 h-8 text-accent animate-spin" />
              <p className="text-white/40">Thinking...</p>
            </div>
          ) : results.length > 0 ? (
            <ResultsList results={results} query={query} />
          ) : query ? (
            <div className="text-center py-20">
              <p className="text-white/40 text-lg">No memories found for "{query}"</p>
            </div>
          ) : (
            <div className="flex flex-col items-center justify-center py-20 text-center gap-4 border border-dashed border-white/10 rounded-2xl bg-white/[0.02]">
              <Database className="w-12 h-12 text-white/10" />
              <div>
                <p className="text-white/60 text-lg font-medium">Your brain is empty</p>
                <p className="text-white/40 max-w-xs mx-auto mt-2">
                  Add a folder with Markdown, TXT, or PDF files to start indexing your knowledge.
                </p>
              </div>
            </div>
          )}
        </div>
      </main>

      {/* Status Bar */}
      <StatusBar count={indexedCount} isIndexing={isIndexing} />
    </div>
  );
}

export default App;
