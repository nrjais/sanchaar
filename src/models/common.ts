export enum ContentType {
  JSON = "json",
  URL_ENCODED = "url_encoded",
  MUTLIPART_FORM = "multipart_form",
  TEXT = "text",
  XML = "xml",
  BLOB = "blob",
  NONE = "none",
}

export const getContentTypeHeader = (
  contentType: ContentType
): string | null => {
  switch (contentType) {
    case ContentType.JSON:
      return "application/json";
    case ContentType.URL_ENCODED:
      return "application/x-www-form-urlencoded";
    case ContentType.MUTLIPART_FORM:
      return "multipart/form-data";
    case ContentType.TEXT:
      return "text/plain";
    case ContentType.XML:
      return "application/xml";
    case ContentType.BLOB:
      return "application/octet-stream";
    case ContentType.NONE:
      return null;
  }
};

export type KeyValueRaw<T> = {
  enabled?: boolean;
  key: string;
  value: T;
  description?: string;
};

export type FormBlob = {
  filename: string;
  data: Blob;
};

export type KeyValue = KeyValueRaw<string>;
export type KeyValueForm = KeyValueRaw<FormBlob | string>;
