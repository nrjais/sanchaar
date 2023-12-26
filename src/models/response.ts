import { ContentType, KeyValue } from "./request";

export interface ResponseDetails {
  // data: string;
  contentType: ContentType;
  headers: KeyValue[];
  status: number;
  contentLength: number;
  statusText: string;
  latency: number;
}
