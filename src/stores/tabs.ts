import { defineStore } from "pinia";
import { computed, ref } from "vue";
import { useRequestStore } from "./requests";

const nextTabId = ((start: number) => {
  return () => `tab-${start++}`;
})(0);

export interface Tab {
  id: string;
  position: number;
}

export const useTabStore = defineStore("TabStore", () => {
  const openTabs = ref(new Map<string, Tab>());
  const activeTab = ref<string>();

  const reqStore = useRequestStore();

  const openTabsList = computed(() => {
    const tabs = [...openTabs.value.values()];
    return tabs.sort((a, b) => a.position - b.position);
  });

  const openNewRequestTab = () => {
    const id = nextTabId();
    reqStore.addNewRequest(id);

    activeTab.value = id;
    openTabs.value.set(id, {
      id: id,
      position: openTabs.value.size,
    });
  };

  const closeTab = (id: string) => {
    if (openTabs.value.size <= 1) {
      return;
    }
    const oldTab = openTabs.value.get(id);
    if (!openTabs.value.delete(id) || !oldTab) {
      return;
    }

    if (activeTab.value !== id) {
      return;
    }

    let newActiveTab: Tab = openTabs.value.values().next()?.value;
    for (const tab of openTabs.value.values()) {
      if (
        tab.position < oldTab.position &&
        tab.position > newActiveTab.position
      ) {
        newActiveTab = tab;
      }
    }
    activeTab.value = newActiveTab.id;
  };

  return {
    openTabs,
    activeTab,
    openTabsList,
    openNewRequestTab,
    closeTab,
  };
});
