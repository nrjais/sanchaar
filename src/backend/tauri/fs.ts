import { readFile, writeTextFile, remove, mkdir } from "@tauri-apps/plugin-fs";
import { FS } from "../api";

class TauriFS implements FS {
  async readFile(path: string): Promise<string> {
    const content = await readFile(path);
    return content.toString();
  }

  async writeFile(path: string, data: string): Promise<void> {
    await writeTextFile(path, data);
  }

  async deleteFile(path: string, opts: { recusive: boolean }): Promise<void> {
    await remove(path, {
      recursive: opts.recusive,
    });
  }

  async createDirectory(path: string): Promise<void> {
    await mkdir(path, { recursive: true });
  }

  async deleteDirectory(path: string): Promise<void> {
    await remove(path, { recursive: true });
  }
}

export default new TauriFS();
