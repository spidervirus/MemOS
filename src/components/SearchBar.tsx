import { Search, Loader2 } from "lucide-react";
import { useState, FormEvent } from "react";

interface SearchBarProps {
  onSearch: (query: string) => void;
  isLoading: boolean;
}

export default function SearchBar({ onSearch, isLoading }: SearchBarProps) {
  const [value, setValue] = useState("");

  const handleSubmit = (e: FormEvent) => {
    e.preventDefault();
    onSearch(value);
  };

  return (
    <form onSubmit={handleSubmit} className="relative group">
      <input
        type="text"
        value={value}
        onChange={(e) => setValue(e.target.value)}
        placeholder="What do you remember about..."
        className="w-full bg-surface border border-white/10 rounded-2xl py-4 pl-12 pr-4 text-lg focus:outline-none focus:ring-2 focus:ring-accent/50 focus:border-accent transition-all placeholder:text-white/20"
      />
      <div className="absolute left-4 top-1/2 -translate-y-1/2">
        {isLoading ? (
          <Loader2 className="w-5 h-5 text-accent animate-spin" />
        ) : (
          <Search className="w-5 h-5 text-white/20 group-focus-within:text-accent transition-colors" />
        )}
      </div>
      <button 
        type="submit"
        className="absolute right-3 top-1/2 -translate-y-1/2 px-4 py-1.5 bg-accent text-white text-sm font-medium rounded-lg opacity-0 group-focus-within:opacity-100 transition-opacity"
      >
        Search
      </button>
    </form>
  );
}
