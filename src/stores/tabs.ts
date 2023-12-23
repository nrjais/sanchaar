import { defineStore } from "pinia";
import { Ref, ref, watch } from "vue";
import { RequestDetails } from "../core/request";

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

const useTabStore = defineStore("tabs", () => {
  const openTabs: Ref<Tab[]> = ref([]);
  const activeTab = ref<string>();
  const activeTabContent = ref<TabContent>();

  watch(activeTab, (id) => {
    const tab = openTabs.value.find((tab) => tab.id === id);
    if (tab) {
      activeTabContent.value = tab.content;
    }
  });

  const openRequestTab = (req: RequestDetails) => {
    const id = nextTabId();
    activeTab.value = id;

    openTabs.value.push({
      id: id,
      content: {
        type: "request",
        value: req,
      },
    });
  };

  const removeTab = (id: string) => {
    const index = openTabs.value.findIndex((tab) => tab.id === id);
    if (index === -1) {
      return;
    }
    openTabs.value.splice(index, 1);
    if (activeTab.value === id) {
      activeTab.value = openTabs.value[Math.max(0, index - 1)]?.id;
    }
  };

  const getTabs = () => {
    return openTabs;
  };

  const tabTitle = (id: string): { name: string; prefix?: string } => {
    const tab = openTabs.value.find((tab) => tab.id === id);
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

  return {
    activeTab,
    openTabs,
    activeTabContent,
    openRequestTab,
    removeTab,
    getTabs,
    tabTitle,
    updateRequest,
  };
});

export default useTabStore;
