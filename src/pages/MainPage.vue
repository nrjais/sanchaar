<template>
  <Box class="p-4">
    <NTabs type="card" v-model:value="tabStore.activeTab" addable closable @close="closeTab" @add="openTab" size="small"
      class="h-full">
      <NTabPane v-for="tab in tabStore.openTabs" :key="tab.id" :name="tab.id" display-directive="show:lazy"
        class="h-full">
        <RequestPane />
        <template #tab>
          <NText>{{ tabTitle(tab.id).prefix }}</NText>
          <NText>{{ tabTitle(tab.id).name }}</NText>
        </template>
      </NTabPane>
    </NTabs>
  </Box>
</template>

<script setup lang="ts">
import Box from '@/components/Box.vue';
import RequestPane from './Pane/RequestPane.vue';
import { NTabs, NTabPane, NText } from 'naive-ui';
import useTabStore from '@/stores/tabs';
import { ContentType, RequestDetails } from '@/core/request';

const tabStore = useTabStore();
const openTab = () => {
  tabStore.openRequestTab(<RequestDetails>{
    name: "Untitled",
    description: "",
    request: {
      method: "GET",
      headers: new Map(),
      contentType: ContentType.NONE,
    },
  });
};

if (tabStore.openTabs.length === 0) {
  openTab();
}

const closeTab = (id: string) => tabStore.removeTab(id);

const tabTitle = (id: string): { name: string, prefix?: string } => {
  return tabStore.tabTitle(id);
};

</script>
