declare global {
  interface Window {
    __TAURI_INTERNALS__: any;
  }
}

export const isTauri = (): boolean => {
  return Boolean(window.__TAURI_INTERNALS__);
};

export const select = <T>(ifTauri: T, ifNotTauri: T): T => {
  return isTauri() ? ifTauri : ifNotTauri;
};

export * from "./fs";
