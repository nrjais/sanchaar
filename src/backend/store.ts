import { Store } from "@tauri-apps/plugin-store";
import { select } from "./api";

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

class LocalStorage implements Storage {
  async save<T>(key: string, value: T): Promise<void> {
    localStorage.setItem(key, JSON.stringify(value));
  }

  async load<T>(key: string): Promise<T | null> {
    const value = localStorage.getItem(key);
    if (value) {
      return JSON.parse(value);
    }
    return null;
  }
}

export const storage = select(new TauriStorage(), new LocalStorage());
