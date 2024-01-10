import { fetch as tauriFetch } from "@tauri-apps/plugin-http";

declare global {
  interface Window {
    __TAURI_INTERNALS__: any;
  }
}

export const isTauri = (): boolean => {
  return Boolean(window.__TAURI_INTERNALS__);
};

export const select = <T>(ifTauri: T, ifNotTauri: T): T => {
  console.log("isTauri", isTauri());

  return isTauri() ? ifTauri : ifNotTauri;
};

const browserFetch = fetch;
export const httpClient = select(tauriFetch, browserFetch);
