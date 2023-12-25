import { Methods } from "@/models/methods";
import { ContentType, Request } from "@/models/request";
import { defineStore } from "pinia";
import { ref } from "vue";

export type ReqTitle = {
  method?: Methods;
  name: string;
};

export const useRequestStore = defineStore("RequestStore", () => {
  const requests = ref(new Map<string, Request>());

  const getRequestTitle = (tabId: string): ReqTitle => {
    const request = requests.value.get(tabId);
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

  const updateRequest = (tabId: string, fn: (r: Request) => void) => {
    const request = requests.value.get(tabId);
    if (request) {
      fn(request);
    }
  };

  const getRequest = (tabId: string): Request | undefined => {
    return requests.value.get(tabId);
  };

  const addNewRequest = (tabId: string) => {
    const req = <Request>{
      name: "Untitled",
      method: Methods.GET,
      domain: "",
      path: "",
      headers: [],
      params: [],
      query: [],
      contentType: ContentType.NONE,
    };
    requests.value.set(tabId, req);
  };

  return {
    requests,
    getRequestTitle,
    addNewRequest,
    updateRequest,
    getRequest,
  };
});
