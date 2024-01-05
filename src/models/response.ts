import { ContentType, KeyValue } from "./common";

export type ResponseBody =
  | {
      type: ContentType.JSON;
      data: string;
    }
  | {
      type: ContentType.XML;
      data: string;
    }
  | {
      type: ContentType.TEXT;
      data: string;
    }
  | {
      type: ContentType.URL_ENCODED;
      data: string;
    }
  | {
      type: ContentType.BLOB;
      data: Blob;
    }
  | {
      type: ContentType.NONE;
    };

export interface ResponseDetails {
  content: ResponseBody;
  headers: KeyValue[];
  status: number;
  contentLength: number;
  statusText: string;
  latency: number;
  requestHeaders: KeyValue[];
}
