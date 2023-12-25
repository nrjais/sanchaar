import { ContentType } from "./request";

export interface ResponseDetails {
  data: string;
  contentType: ContentType;
  headers: Map<string, string>;
  status: number;
  contentLength: number;
  statusText: string;
  latency: number;
}
