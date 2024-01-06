import { ContentType, KeyValue } from "./common";
import Environment from "./environment";
import { Methods } from "./methods";

export type RequestBody =
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
      data: KeyValue[];
    }
  | {
      type: ContentType.MUTLIPART_FORM;
      data: KeyValue[];
    }
  | {
      type: ContentType.BLOB;
      data: Blob;
    }
  | {
      type: ContentType.NONE;
    };

export interface RequestConfig {
  name: string;
  method: Methods;
  address: string;
  headers: KeyValue[];
  params: KeyValue[];
  query: KeyValue[];
  body: RequestBody;
  environment: Environment;
}

export interface RequestDetails {
  name: string;
  description?: string;
  config: RequestConfig;
}
