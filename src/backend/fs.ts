import { FS, select } from "./api";
import TauriFS from "./tauri/fs";

export const fs: FS = select(TauriFS, null!);
