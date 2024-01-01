<template>
  <NTabs type="card" v-model:value="tabStore.activeTab" addable closable @close="tabStore.closeTab"
    @add="tabStore.openNewRequestTab" size="small" class="h-full p-2">
    <NTabPane v-for="tab in tabStore.openTabsList" :key="tab.id" :name="tab.id" display-directive="show:lazy"
      class="h-0 max-h-full flex-grow">
      <HttpTab :tabId="tab.id" />
      <template #tab>
        <NText class="pr-2 font-medium" v-if="requestTitle(tab.id).method" :style="{ color: tabColor(tab.id) }">
          {{ requestTitle(tab.id).method }}
        </NText>
        <NText>{{ requestTitle(tab.id).name }}</NText>
      </template>
    </NTabPane>
  </NTabs>
</template>

<script setup lang="ts">
import { Methods } from '@/models/methods';
import { useRequestStore } from '@/stores/requests';
import { useTabStore } from '@/stores/tabs';
import { methodColor } from '@/utils/methodColor';
import { NTabPane, NTabs, NText } from 'naive-ui';
import HttpTab from '../HttpTab/HttpTab.vue';

const tabStore = useTabStore();
const requestStore = useRequestStore();

const requestTitle = (id: string) => {
  return requestStore.getRequestTitle(id);
};

if (tabStore.openTabsList.length === 0) {
  tabStore.openNewRequestTab();
}

const tabColor = (id: string) => {
  const method = requestTitle(id).method;
  return methodColor(method as Methods);
};

</script>
