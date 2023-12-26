import { ContentType, KeyValue } from "./request";

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
      type: ContentType.BYTES;
      data: ArrayBuffer;
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
}
