import { KeyValue } from "@/models/common";

export const parsePathParams = (url: string): KeyValue[] => {
  const pathParams: KeyValue[] = [];
  const urlParts = url.split("/");
  for (let part of urlParts) {
    if (part.startsWith(":") && part.length > 1) {
      pathParams.push({ key: part.substring(1), value: "" });
    }
  }
  return pathParams;
};

export const replacePathParams = (params: KeyValue[], url: string): string => {
  let replaced = url;
  for (let param of params) {
    replaced = replaced.replace(`:${param.key}`, param.value);
  }
  return replaced;
};
