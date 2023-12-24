import { Methods } from "@/core/methods";
import { ContentType, Request } from "@/core/request";
import { reactive } from "vue";

const requests = reactive(new Map<string, Request>());

export const addNewRequest = (tabId: string) => {
  const req = reactive(<Request>{
    name: "Untitled",
    method: Methods.GET,
    domain: "",
    path: "",
    headers: [],
    params: [],
    query: [],
    contentType: ContentType.NONE,
  });
  requests.set(tabId, req);
};

export type ReqTitle = {
  method?: Methods;
  name: string;
};

export const requestTitle = (tabId: string): ReqTitle => {
  const request = requests.get(tabId);
  if (request) {
    return {
      method: request.method,
      name: request.name,
    };
  }

  return {
    name: "Unknown",
  };
};

export const updateRequest = (tabId: string, fn: (r: Request) => void) => {
  const request = requests.get(tabId);
  if (request) {
    fn(request);
  }
};

export const getRequest = (tabId: string): Request | undefined => {
  return requests.get(tabId);
};
