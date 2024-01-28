import { execute } from "@/backend/client";
import { ContentType } from "@/models/common";
import Environment from "@/models/environment";
import { Methods } from "@/models/methods";
import { RequestConfig } from "@/models/request";
import { ResponseDetails } from "@/models/response";
import { defineStore } from "pinia";
import { ref, toRaw } from "vue";

export type ReqTitle = {
  method?: Methods;
  name: string;
};

export type OpenRequest = {
  name: string;
  config: RequestConfig;
};

export type ExecutionState =
  | { state: "idle" }
  | { state: "running"; abort: () => void }
  | { state: "cancelled" }
  | { state: "error"; error: Error }
  | { state: "completed"; response: ResponseDetails };

export const useRequestStore = defineStore("RequestStore", () => {
  const requests = ref<ObjectMap<OpenRequest>>({});
  const executions = ref<ObjectMap<ExecutionState>>({});

  const getRequestTitle = (tabId: string): ReqTitle => {
    const request = requests.value[tabId];
    if (request) {
      return {
        method: request.config.method,
        name: request.name,
      };
    }

    return { name: "Unknown" };
  };

  const updateRequest = (tabId: string, fn: (r: RequestConfig) => void) => {
    const request = requests.value[tabId];
    if (request) {
      fn(request.config);
    }
  };

  const getRequest = (tabId: string): RequestConfig | undefined => {
    return requests.value[tabId].config;
  };

  const getRequestAddress = (tabId: string): string => {
    return requests.value[tabId]?.config.address || "";
  };

  const addNewRequest = (tabId: string) => {
    const req = <RequestConfig>{
      method: Methods.POST,
      address: "https://echo.nrjais.com",
      headers: [],
      params: [],
      query: [],
      body: { type: ContentType.NONE },
      environment: Environment.request(),
    };
    addRequest(tabId, { config: req, name: "Untitled" });
  };

  const addRequest = (tabId: string, request: OpenRequest) => {
    requests.value[tabId] = request;
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
      const options = { signal: abort.signal };
      const response = await execute(toRaw(request.config), options);
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
    addRequest,
    removeRequest,
    updateRequest,
    getRequest,
    executeRequest,
    getExecutionResult,
    getRequestAddress,
  };
});
