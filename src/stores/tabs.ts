import { reactive, ref } from "vue";
import { addNewRequest } from "./requests";

const nextTabId = ((start: number) => {
  return () => `tab-${start++}`;
})(0);

export interface Tab {
  id: string;
  position: number;
}

const openTabs = reactive(new Map<string, Tab>());
export const activeTab = ref<string>();

export const openNewRequestTab = () => {
  const id = nextTabId();
  activeTab.value = id;
  addNewRequest(id);
  openTabs.set(id, {
    id: id,
    position: openTabs.size,
  });
};

export const openTabsList = () => {
  const tabs = [...openTabs.values()];
  return tabs.sort((a, b) => a.position - b.position);
};

export const closeTab = (id: string) => {
  if (openTabs.size <= 1) {
    return;
  }
  const oldTab = openTabs.get(id);
  if (!openTabs.delete(id) || !oldTab) {
    return;
  }

  if (activeTab.value !== id) {
    return;
  }

  let newActiveTab: Tab = openTabs.values().next()?.value;
  for (const tab of openTabs.values()) {
    if (
      tab.position < oldTab.position &&
      tab.position > newActiveTab.position
    ) {
      newActiveTab = tab;
    }
  }
  activeTab.value = newActiveTab.id;
};
