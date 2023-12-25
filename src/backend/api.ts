import { CreateAxiosDefaults } from "axios";
import { bOptions } from "./browser/axios";
import { tOptions } from "./tauri/axios";

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

export const axiosOptions = select<CreateAxiosDefaults>(tOptions, bOptions);
