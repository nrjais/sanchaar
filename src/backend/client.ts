import { ContentType, KeyValue, Request } from "@/models/request";
import { ResponseBody, ResponseDetails } from "@/models/response";
import { getReasonPhrase } from "http-status-codes";

export const getContent = async (response: Response): Promise<ResponseBody> => {
  const headers = response.headers;
  if (headers.has("content-type")) {
    const contentType = headers.get("content-type")?.toLowerCase();
    const headerValueType = typeof contentType;
    if (!contentType || headerValueType !== "string") {
      return {
        type: ContentType.BYTES,
        data: await response.arrayBuffer(),
      };
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
  return {
    type: ContentType.BYTES,
    data: await response.arrayBuffer(),
  };
};

export const execute = async (
  reqConfig: Request,
  options: {
    signal?: AbortSignal;
  }
): Promise<ResponseDetails> => {
  const { address, method } = reqConfig;
  const headers = reqConfig.headers
    .filter((header) => header.enabled)
    .filter((header) => header.key)
    .map((header) => [header.key, header.value] as [string, string]);

  const queryParams = reqConfig.query
    .filter((query) => query.enabled)
    .filter((query) => query.key)
    .map((query) => [query.key, query.value] as [string, string]);

  const url = new URL(address);
  for (const query of queryParams) {
    url.searchParams.append(query[0], query[1]);
  }

  const startTime = Date.now();
  const response = await fetch(url, {
    method: method.toString(),
    cache: "no-cache",
    headers: headers,
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
    content: await getContent(response),
    latency: latency,
  };
};
