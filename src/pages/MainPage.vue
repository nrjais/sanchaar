<template>
  <NTabs type="card" :value="activeTab" addable closable @close="closeTab" @add="openNewRequestTab" size="small"
    class="h-full p-2">
    <NTabPane v-for="tab in openTabs" :key="tab.id" :name="tab.id" display-directive="show:lazy"
      class="h-0 max-h-full flex-grow">
      <RequestPane :tabId="tab.id" />
      <template #tab>
        <NText class="pr-2 font-medium" v-if="requestTitle(tab.id).method" :style="{ color: methodColor(tab.id) }">
          {{ requestTitle(tab.id).method }}
        </NText>
        <NText>{{ requestTitle(tab.id).name }}</NText>
      </template>
    </NTabPane>
  </NTabs>
</template>

<script setup lang="ts">
import { Methods } from '@/core/methods';
import { requestTitle } from '@/stores/requests';
import { activeTab, closeTab, openNewRequestTab, openTabsList } from '@/stores/tabs';
import { NTabPane, NTabs, NText } from 'naive-ui';
import { computed } from 'vue';
import RequestPane from './Pane/RequestPane.vue';

const openTabs = computed(() => openTabsList());

if (openTabs.value.length === 0) {
  openNewRequestTab();
}

const methodColor = (id: string) => {
  const method = requestTitle(id).method;
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
    case Methods.TRACE:
      return '#6B7280';
    default:
      return '#D1D5DB';
  }
};

</script>
