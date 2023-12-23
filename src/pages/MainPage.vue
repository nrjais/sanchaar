<template>
  <Box class="p-4">
    <NTabs type="card" :value="tabStore.activeTab.value" addable closable @close="closeTab" @add="openTab" size="small"
      class="h-full">
      <NTabPane v-for="tab in tabStore.openTabs" :key="tab.id" :name="tab.id" display-directive="show:lazy"
        class="h-full">
        <RequestPane />
        <template #tab>
          <NText class="mr-1 font-medium" :style="{ color: methodColor(tab.id) }">
            {{ tabTitle(tab.id).prefix }}
          </NText>
          <NText>{{ tabTitle(tab.id).name }}</NText>
        </template>
      </NTabPane>
    </NTabs>
  </Box>
</template>

<script setup lang="ts">
import Box from '@/components/Box.vue';
import { Methods } from '@/core/methods';
import useTabStore from '@/stores/tabs';
import { NTabPane, NTabs, NText } from 'naive-ui';
import RequestPane from './Pane/RequestPane.vue';

const tabStore = useTabStore();
const openTab = () => {
  tabStore.openRequestTab("Untitled");
};

if (tabStore.openTabs.length === 0) {
  openTab();
}

const closeTab = (id: string) => tabStore.removeTab(id);

const tabTitle = (id: string): { name: string, prefix?: string } => {
  return tabStore.tabTitle(id);
};

const methodColor = (id: string) => {
  const method = tabStore.tabTitle(id).prefix;
  switch (method) {
    case Methods.GET:
      return '#059669';
    case Methods.POST:
      return '#2563EB';
    case Methods.PUT:
      return '#DB2777';
    case Methods.DELETE:
      return '#DC2626';
    case Methods.PATCH:
      return '#F59E0B';
    case Methods.HEAD:
      return '#7C3AED';
    case Methods.OPTIONS:
      return '#14B8A6';
    default:
      return '#D1D5DB';
  }
};

</script>
