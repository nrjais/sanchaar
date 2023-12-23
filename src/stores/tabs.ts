import { Ref, reactive, ref, watch } from "vue";
import { Request, RequestDetails } from "../core/request";
import { Methods } from "@/core/methods";

const nextTabId = ((start: number) => {
  return () => `tab-${start++}`;
})(0);

export type TabContent = {
  type: "request";
  value: RequestDetails;
};

export interface Tab {
  id: string;
  content: TabContent;
}

const useTabStore = () => {
  const openTabs: Tab[] = reactive([]);
  const activeTab = ref<string>();
  const activeTabContent = ref<TabContent>();

  watch(activeTab, (id) => {
    const tab = openTabs.find((tab) => tab.id === id);
    if (tab) {
      activeTabContent.value = tab.content;
    }
  });

  const openRequestTab = (name: string) => {
    const id = nextTabId();
    activeTab.value = id;

    openTabs.push({
      id: id,
      content: {
        type: "request",
        value: {
          name: name,
          config: {
            name: name,
            method: Methods.GET,
            domain: "",
            path: "",
            headers: [],
            params: [],
            query: [],
            contentType: "none",
          },
        },
      },
    });
  };

  const removeTab = (id: string) => {
    const index = openTabs.findIndex((tab) => tab.id === id);
    if (index === -1) {
      return;
    }
    openTabs.splice(index, 1);
    if (activeTab.value === id) {
      activeTab.value = openTabs[Math.max(0, index - 1)]?.id;
    }
  };

  const getTabs = () => {
    return openTabs;
  };

  const tabTitle = (id: string): { name: string; prefix?: string } => {
    const tab = openTabs.find((tab) => tab.id === id);
    if (tab) {
      switch (tab.content.type) {
        case "request":
          return {
            prefix: tab.content.value.config.method,
            name: tab.content.value.name,
          };
      }
    }
    return { name: "Untitled" };
  };

  const updateRequest = (fn: (r: RequestDetails) => void) => {
    const tabContent = activeTabContent.value;
    if (tabContent?.type === "request") {
      fn(tabContent.value);
    }
  };

  const getRequestConfig = (): Request | undefined => {
    const tabContent = activeTabContent.value;
    if (tabContent?.type === "request") {
      return tabContent.value.config;
    }
  };

  return {
    activeTab,
    openTabs,
    activeTabContent,
    openRequestTab,
    removeTab,
    getTabs,
    tabTitle,
    updateRequest,
    getRequestConfig,
  };
};

export default useTabStore;
