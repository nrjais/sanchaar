import { Methods } from "./methods";

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

export interface KeyValue {
  enabled?: boolean;
  key: string;
  value: string;
  description?: string;
}

export interface Request {
  name: string;
  method: Methods;
  domain: string;
  path: string;
  headers: KeyValue[];
  params: KeyValue[];
  query: KeyValue[];
  body?: string;
  contentType: ContentType;
}

export interface RequestDetails {
  name: string;
  description?: string;
  config: Request;
}
