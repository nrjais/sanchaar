import { ContentType, KeyValue, getContentTypeHeader } from "@/models/common";
import { RequestBody, RequestConfig } from "@/models/request";
import { replacePathParams } from "./pathParams";
import Environment from "@/models/environment";

const pattern = /\{\{(\w+)\}\}/g;

const resolveVars = (value: string, env: Environment): string => {
  return value.replace(pattern, (_match, name) => env.get(name) ?? "");
};

export const preprocess = (
  request: RequestConfig
): { url: URL; request: RequestInit; headers: KeyValue[] } => {
  const address = processAddress(request);
  const url = processURL(request.query, address, request.environment);

  const [headers, raw] = buildRequestHeaders(request);

  return {
    url,
    request: {
      method: request.method.toString(),
      cache: "no-cache",
      headers: headers,
      body: encodeBody(request.body),
    },
    headers: raw,
  };
};

const buildRequestHeaders = (
  request: RequestConfig
): [HeadersInit, KeyValue[]] => {
  const headers = request.headers
    .filter((header) => header.enabled)
    .filter((header) => header.key)
    .map((header) => ({
      key: header.key,
      value: resolveVars(header.value, request.environment),
    }));

  const contentType = getContentTypeHeader(request.body.type);
  if (contentType) {
    headers.push({ key: "Content-Type", value: contentType });
  }

  const headersInit = new Headers();
  for (const header of headers) {
    headersInit.append(header.key, header.value);
  }

  return [headersInit, headers];
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

const processAddress = (request: RequestConfig) => {
  const params = request.params.map((param) => ({
    key: param.key,
    value: resolveVars(param.value, request.environment),
  }));
  const rawAddress = resolveVars(request.address, request.environment);
  return replacePathParams(params, rawAddress);
};

const processURL = (
  queries: KeyValue[],
  address: string,
  env: Environment
): URL => {
  const queryParams = queries
    .filter((query) => query.enabled)
    .filter((query) => query.key);

  const url = new URL(address);
  for (const query of queryParams) {
    const value = resolveVars(query.value, env);
    url.searchParams.append(query.key, value);
  }

  return url;
};
