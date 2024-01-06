import { ContentType, KeyValue } from "@/models/common";
import { RequestConfig } from "@/models/request";
import { ResponseBody, ResponseDetails } from "@/models/response";
import { getReasonPhrase } from "http-status-codes";
import { preprocess as preProcess } from "./preprocess";

export const decodeBody = async (response: Response): Promise<ResponseBody> => {
  const headers = response.headers;
  if (headers.has("content-type")) {
    const contentType = headers.get("content-type")?.toLowerCase();
    const headerValueType = typeof contentType;
    if (!contentType || headerValueType !== "string") {
      return { type: ContentType.BLOB, data: await response.blob() };
    }
    const data = await response.text();
    if (contentType?.includes("json")) {
      return { type: ContentType.JSON, data };
    } else if (contentType?.includes("urlencoded")) {
      return { type: ContentType.URL_ENCODED, data };
    } else if (contentType?.includes("xml")) {
      return { type: ContentType.XML, data };
    } else if (contentType?.includes("text")) {
      return { type: ContentType.TEXT, data };
    }
  }
  return { type: ContentType.BLOB, data: await response.blob() };
};

export const execute = async (
  reqConfig: RequestConfig,
  options: { signal?: AbortSignal }
): Promise<ResponseDetails> => {
  const processed = preProcess(reqConfig);

  const startTime = Date.now();
  const response = await fetch(processed.url, {
    ...processed.request,
    signal: options.signal,
  });

  const latency = Date.now() - startTime;

  const responseHeaders = [] as KeyValue[];
  for (const [key, value] of [...response.headers]) {
    responseHeaders.push({ key, value });
  }

  return {
    contentLength: Number(response.headers.get("content-length") || 0),
    headers: responseHeaders,
    status: response.status,
    statusText: getReasonPhrase(response.status),
    content: await decodeBody(response),
    latency: latency,
    requestHeaders: processed.headers,
  };
};
