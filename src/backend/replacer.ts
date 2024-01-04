import { KeyValue } from "@/models/common";

export const getPathParams = (url: string): KeyValue[] => {
  const pathParams: KeyValue[] = [];
  const urlParts = url.split("/");
  for (let part of urlParts) {
    if (part.startsWith(":") && part.length > 1) {
      pathParams.push({ key: part.substring(1), value: "" });
    }
  }
  return pathParams;
};
