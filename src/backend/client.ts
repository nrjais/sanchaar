import { ContentType, Request } from "@/models/request";
import { ResponseDetails } from "@/models/response";
import axios from "axios";
import { groupBy, mapValues } from "lodash";
import { axiosOptions } from "./api";

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

const client = axios.create(axiosOptions);

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

export const getContentType = (headers: Map<string, string>): ContentType => {
  if (headers.has("content-type")) {
    const contentType = headers.get("content-type")?.toLowerCase();
    if (contentType?.includes("json")) {
      return ContentType.JSON;
    } else if (contentType?.includes("urlencoded")) {
      return ContentType.URL_ENCODED;
    } else if (contentType?.includes("xml")) {
      return ContentType.XML;
    }
  }

  return ContentType.BYTES;
};

export const execute = async (reqConfig: Request): Promise<ResponseDetails> => {
  const { domain, path, method, body, headers, query } = reqConfig;
  const headerEntries = headers.map(
    (header) => [header.key, header.value] as [string, string]
  );

  const queryParams = mapValues(
    groupBy(query, (e) => e.key),
    (values) => values.map((e) => e.value)
  );

  const response = await client.request({
    url: `${domain}${path}`,
    method,
    data: body,
    params: queryParams,
    headers: Object.fromEntries(headerEntries),
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
