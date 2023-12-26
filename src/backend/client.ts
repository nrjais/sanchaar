import { ContentType, KeyValue, Request } from "@/models/request";
import { ResponseDetails } from "@/models/response";

export const getContentType = (headers: Headers): ContentType => {
  if (headers.has("content-type")) {
    const contentType = headers.get("content-type")?.toLowerCase();
    const headerValueType = typeof contentType;
    if (!contentType || headerValueType !== "string") {
      return ContentType.BYTES;
    }
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

  const response = await fetch(url, {
    method: method.toString(),
    cache: "no-cache",
    headers: headers,
    signal: options.signal,
  });

  const responseHeaders = [] as KeyValue[];
  Object.entries(response.headers).forEach(([key, values]) => {
    values.forEach((value: string) => {
      responseHeaders.push({ key, value });
    });
  });

  const latency = 5;

  return {
    contentLength: Number(response.headers.get("content-length") || 0),
    headers: responseHeaders,
    status: response.status,
    statusText: response.statusText,
    contentType: getContentType(response.headers),
    latency: latency,
  };
};
