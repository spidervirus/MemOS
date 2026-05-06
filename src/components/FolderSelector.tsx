import { open } from "@tauri-apps/plugin-dialog";
import { FolderOpen } from "lucide-react";

interface FolderSelectorProps {
  onSelect: (path: string) => void;
}

export default function FolderSelector({ onSelect }: FolderSelectorProps) {
  const handleOpen = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Select Folder to Index",
      });

      if (selected && typeof selected === "string") {
        onSelect(selected);
      }
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <button
      onClick={handleOpen}
      className="flex items-center gap-2 px-4 py-2 bg-white/5 hover:bg-white/10 border border-white/10 rounded-xl transition-all active:scale-95"
    >
      <FolderOpen className="w-4 h-4 text-white/60" />
      <span className="text-sm font-medium">Add Folder</span>
    </button>
  );
}
