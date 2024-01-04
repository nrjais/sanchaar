import { execute } from "@/backend/client";
import { ContentType, KeyValue } from "@/models/common";
import { Methods } from "@/models/methods";
import { RequestConfig } from "@/models/request";
import { ResponseDetails } from "@/models/response";
import { defineStore } from "pinia";
import { ref } from "vue";

type ObjectMap<T> = { [key: string]: T };

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
  const requests = ref<ObjectMap<RequestConfig>>({});
  const executions = ref<ObjectMap<ExecutionState>>({});

  const getRequestTitle = (tabId: string): ReqTitle => {
    const request = requests.value[tabId];
    if (request) {
      return {
        method: request.method,
        name: request.name,
      };
    }

    return { name: "Unknown" };
  };

  const updateRequest = (tabId: string, fn: (r: RequestConfig) => void) => {
    const request = requests.value[tabId];
    if (request) {
      fn(request);
    }
  };

  const updateRequestDeep = (
    tabId: string,
    updates: { address?: string; params?: KeyValue[] }
  ) => {
    console.log("updateRequestDeep", tabId, updates);

    const request = requests.value[tabId];
    if (request) {
      if (updates.address) {
        requests.value[tabId].address = updates.address;
      }
      if (updates.params) {
        requests.value[tabId].params = updates.params;
      }
    }
  };

  const getRequest = (tabId: string): RequestConfig | undefined => {
    return requests.value[tabId];
  };

  const getRequestAddress = (tabId: string): string => {
    return requests.value[tabId]?.address || "";
  };

  const addNewRequest = (tabId: string) => {
    const req = <RequestConfig>{
      name: "Untitled",
      method: Methods.POST,
      address: "https://echo.hoppscotch.io",
      headers: [],
      params: [],
      query: [],
      body: { type: ContentType.NONE },
    };
    requests.value[tabId] = req;
  };

  const removeRequest = (tabId: string) => {
    delete requests.value[tabId];
    delete executions.value[tabId];
  };

  const executeRequest = async (tabId: string) => {
    const request = requests.value[tabId];
    if (!request) {
      return;
    }

    const abort = new AbortController();
    executions.value[tabId] = {
      state: "running",
      abort: () => abort.abort(),
    };
    try {
      const response = await execute(request, { signal: abort.signal });
      executions.value[tabId] = { state: "completed", response };
    } catch (error: any) {
      if (error?.name === "AbortError") {
        executions.value[tabId] = { state: "cancelled" };
      } else {
        executions.value[tabId] = { state: "error", error };
      }
    }
  };

  const getExecutionResult = (tabId: string): ExecutionState => {
    return executions.value[tabId] || { state: "idle" };
  };

  return {
    requests,
    executions,
    getRequestTitle,
    addNewRequest,
    removeRequest,
    updateRequest,
    updateRequestDeep,
    getRequest,
    executeRequest,
    getExecutionResult,
    getRequestAddress,
  };
});
