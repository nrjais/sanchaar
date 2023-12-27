import { execute } from "@/backend/client";
import { Methods } from "@/models/methods";
import { ContentType, Request } from "@/models/request";
import { ResponseDetails } from "@/models/response";
import { defineStore } from "pinia";
import { ref } from "vue";

export type ReqTitle = {
  method?: Methods;
  name: string;
};

export type ExecutionState =
  | { state: "idle" }
  | { state: "running"; abort: () => void }
  | { state: "cancelled" }
  | { state: "error"; error: Error }
  | { state: "completed"; response: ResponseDetails };

export const useRequestStore = defineStore("RequestStore", () => {
  const requests = ref(new Map<string, Request>());
  const executions = ref(new Map<string, ExecutionState>());

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

  const getRequestAddress = (tabId: string): string => {
    const request = requests.value.get(tabId);
    return request?.address || "";
  };

  const addNewRequest = (tabId: string) => {
    const req = <Request>{
      name: "Untitled",
      method: Methods.GET,
      address: "https://echo.hoppscotch.io",
      headers: [],
      params: [],
      query: [],
      contentType: ContentType.NONE,
    };
    requests.value.set(tabId, req);
  };

  const executeRequest = async (tabId: string) => {
    const request = requests.value.get(tabId);
    if (!request) {
      return;
    }

    const abort = new AbortController();
    executions.value.set(tabId, {
      state: "running",
      abort: () => abort.abort(),
    });
    try {
      const response = await execute(request, { signal: abort.signal });
      executions.value.set(tabId, { state: "completed", response });
    } catch (error: any) {
      if (error?.name === "AbortError") {
        executions.value.set(tabId, { state: "cancelled" });
      } else {
        executions.value.set(tabId, { state: "error", error });
      }
    }
  };

  const getExecutionResult = (tabId: string): ExecutionState => {
    return executions.value.get(tabId) || { state: "idle" };
  };

  return {
    requests,
    executions,
    getRequestTitle,
    addNewRequest,
    updateRequest,
    getRequest,
    executeRequest,
    getExecutionResult,
    getRequestAddress,
  };
});
