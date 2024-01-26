import { Store } from "@tauri-apps/plugin-store";

export interface Storage {
  save: <T>(key: string, value: T) => Promise<void>;
  load: <T>(key: string) => Promise<T | null>;
}

class TauriStorage implements Storage {
  private store = new Store("./store.dat");
  async save<T>(key: string, value: T): Promise<void> {
    await this.store.set(key, value);
  }

  async load<T>(key: string): Promise<T | null> {
    return await this.store.get<T>(key);
  }
}

export const storage = new TauriStorage();
