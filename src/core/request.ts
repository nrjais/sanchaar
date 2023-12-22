export enum ContentType {
  JSON = "json",
  URL_ENCODED = "url_encoded",
  TEXT = "text",
  XML = "xml",
  BYTES = "bytes",
  NONE = "none",
}

export interface RequestBody {
  type: ContentType;
  body: string;
}

export interface Request {
  name: string;
  method: string;
  domain: string;
  path: string;
  headers: Map<string, string>;
  params: Map<string, string>;
  query: Map<string, string>;
  body?: string;
  contentType: string;
}

export interface RequestDetails {
  name: string;
  description: string;
  request: Request;
}
