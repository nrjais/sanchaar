declare global {
  interface Window {
    __TAURI__: any;
  }
}

export const isTauri = (): boolean => {
  return Boolean(window.__TAURI__);
};

export const select = <T>(ifTauri: T, ifNotTauri: T): T => {
  return isTauri() ? ifTauri : ifNotTauri;
};
