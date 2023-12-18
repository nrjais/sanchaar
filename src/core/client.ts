import axios from "axios";
// @ts-ignore
import axiosTauriAdapter from "axios-tauri-adapter";

declare module "axios" {
  export interface AxiosRequestConfig {
    metadata?: {
      startTime: Date;
    };
  }

  export interface AxiosResponse<T = any> extends Promise<T> {
    metadata?: {
      endTime: Date;
    };
  }
}

const client = axios.create({ adapter: axiosTauriAdapter });

axios.interceptors.request.use(
  (config) => {
    config.metadata = { startTime: new Date() };
    return config;
  },
  (error) => Promise.reject(error)
);

axios.interceptors.response.use(
  (response) => {
    response.metadata = {
      ...response.metadata,
      endTime: new Date(),
    };
    return response;
  },
  (error) => Promise.reject(error)
);

export type RequestConfig = {
  url: string;
  method: "GET" | "POST" | "PUT" | "DELETE";
  data?: any;
  params?: any;
  headers?: any;
};

export enum ContentType {
  JSON = "application/json",
  FORM = "application/x-www-form-urlencoded",
  MULTIPART = "multipart/form-data",
  UNKNOWN = "unknown",
}

export const getContentType = (headers: Map<string, string>): ContentType => {
  if (headers.has("content-type")) {
    const contentType = headers.get("content-type")?.toLowerCase();
    if (contentType?.includes("json")) {
      return ContentType.JSON;
    } else if (contentType?.includes("form")) {
      return ContentType.FORM;
    } else if (contentType?.includes("multipart")) {
      return ContentType.MULTIPART;
    }
  }

  return ContentType.UNKNOWN;
};

export type Response = {
  data: any;
  contentLength: number;
  headers: Map<string, string>;
  status: number;
  statusText: string;
  contentType: ContentType;
  latency: number;
};

export const execute = async (reqConfig: RequestConfig): Promise<Response> => {
  const { url, method, data, params, headers } = reqConfig;

  const response = await client.request({
    url,
    method,
    data,
    params,
    headers,
  });

  const headersMap = new Map<string, string>();
  Object.keys(response.headers).forEach((key) => {
    headersMap.set(key, response.headers[key]);
  });

  const latency =
    response.metadata?.endTime.getTime() -
    response.metadata?.startTime.getTime();

  return {
    data: response.data,
    contentLength: response.headers["content-length"],
    headers: headersMap,
    status: response.status,
    statusText: response.statusText,
    contentType: getContentType(headersMap),
    latency: latency,
  };
};
