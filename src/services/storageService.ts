import { invoke } from "@tauri-apps/api/core";
import type { CapturedContent } from "../types/content";

export async function getAllContent(
  limit?: number,
  offset?: number
): Promise<CapturedContent[]> {
  return invoke("get_all_content", { limit, offset });
}

export async function deleteContent(id: string): Promise<void> {
  return invoke("delete_content", { id });
}
