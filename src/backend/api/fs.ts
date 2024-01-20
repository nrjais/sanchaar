export interface FS {
  readFile(path: string): Promise<string>;
  writeFile(path: string, data: string): Promise<void>;
  deleteFile(path: string, opts: { recusive: boolean }): Promise<void>;
  createDirectory(path: string): Promise<void>;
  deleteDirectory(path: string): Promise<void>;
}
