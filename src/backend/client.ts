import { ContentType, KeyValue, getContentTypeHeader } from "@/models/common";
import { RequestConfig, RequestBody } from "@/models/request";
import { ResponseBody, ResponseDetails } from "@/models/response";
import { getReasonPhrase } from "http-status-codes";
import { replacePathParams } from "./replacer";

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

const encodeBody = (body: RequestBody): BodyInit | undefined => {
  switch (body.type) {
    case ContentType.JSON:
    case ContentType.XML:
    case ContentType.TEXT:
      return body.data;
    case ContentType.URL_ENCODED:
      const urlEncoded = new URLSearchParams();
      const params = body.data
        .filter((param) => param.enabled)
        .filter((param) => param.key);
      for (const data of params) {
        urlEncoded.append(data.key, data.value as string);
      }
      return urlEncoded;
    case ContentType.MUTLIPART_FORM:
      const formData = new FormData();
      for (const data of body.data) {
        formData.append(data.key, data.value as string);
      }
      return formData;
    case ContentType.BLOB:
      return body.data;
    case ContentType.NONE:
      return undefined;
  }
};

export const execute = async (
  reqConfig: RequestConfig,
  options: { signal?: AbortSignal }
): Promise<ResponseDetails> => {
  const { address, method } = reqConfig;
  const queryParams = reqConfig.query
    .filter((query) => query.enabled)
    .filter((query) => query.key)
    .map((query) => [query.key, query.value] as [string, string]);

  const withPathParams = replacePathParams(reqConfig.params, address);

  const url = new URL(withPathParams);
  for (const query of queryParams) {
    url.searchParams.append(query[0], query[1]);
  }

  return sendRequest(url, {
    method: method.toString(),
    cache: "no-cache",
    headers: buildRequestHeaders(reqConfig),
    signal: options.signal,
    body: encodeBody(reqConfig.body),
  });
};

const sendRequest = async (url: URL, requestConfig: RequestInit) => {
  const startTime = Date.now();
  const response = await fetch(url, requestConfig);
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
  };
};

const buildRequestHeaders = (
  reqConfig: RequestConfig
): HeadersInit | undefined => {
  const headers = reqConfig.headers
    .filter((header) => header.enabled)
    .filter((header) => header.key)
    .map((header) => [header.key, header.value] as [string, string]);

  const headersInit = new Headers();
  for (const header of headers) {
    headersInit.append(header[0], header[1]);
  }

  const contentType = getContentTypeHeader(reqConfig.body.type);
  if (contentType) {
    headersInit.append("Content-Type", contentType);
  }

  return headersInit;
};
