import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

export interface MarkdownResult {
  content: string;
  filename: string;
}

// interact with rust backend
// get markdown file content
export async function openMarkdownFile(): Promise<MarkdownResult | null> {
  try {
    // Open file dialog
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "Markdown",
          extensions: ["md", "markdown"],
        },
      ],
    });

    if (selected === null) {
      return null;
    }

    const filePath = Array.isArray(selected) ? selected[0] : selected;

    // Read file content
    const result: MarkdownResult = await invoke("read_markdown_file", {
      path: filePath,
    });
    return result;
  } catch (error) {
    console.error("Error reading markdown file:", error);
    throw error;
  }
}
