<template>
  <Box class="p-4">
    <NTabs type="card" v-model:value="activeTab" addable closable @close="closeTab" @add="openTab" size="small" class="h-full">
      <NTabPane v-for="panel in panels" :key="panel.name" :tab="panel.title" :name="panel.name" display-directive="show:lazy" class="h-full">
        <RequestPane />
        <template #tab>
          <span>{{ panel.title }} &#8226;</span>
        </template>
      </NTabPane>
    </NTabs>
  </Box>
</template>

<script setup lang="ts">
import Box from '@/components/Box.vue';
import RequestPane from './Pane/RequestPane.vue';
import { NTabs, NTabPane } from 'naive-ui';
import { ref } from 'vue';

const activeTab = ref('1');
let tabId = 1;

const panels = ref([{
  name: "1",
  title: "Test",
}]);

const closeTab = (name: string) => {
  if (panels.value.length === 1) {
    return;
  }
  panels.value = panels.value.filter((panel) => panel.name !== name);
  activeTab.value = panels.value[panels.value.length - 1].name;
};

const openTab = () => {
  panels.value.push({
    name: String(++tabId),
    title: "Test",
  });
};
</script>
