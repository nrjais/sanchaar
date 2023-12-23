<template>
  <Box>
    <NInputGroup class="flex">
      <Box width="w-32">
        <NSelect size="large" :options="methods" v-model:value="method" :on-update-value="updateMethod" filterable tag
          :consistent-menu-width="false" />
      </Box>
      <NInput size="large" class="flex-grow" placeholder="Address"></NInput>
      <NButton type="success" size="large">
        <NIcon>
          <IconSend2 />
        </NIcon>
      </NButton>
    </NInputGroup>
    <NSplit direction="horizontal" :max="0.75" :min="0.25" :default-size="0.40" class="py-2">
      <template #1>
        <Box class="pr-4">
          <RequestConfig />
        </Box>
      </template>
      <template #2>
        <Box class="pl-4">
          <ResponseDetails />
        </Box>
      </template>
    </NSplit>
  </Box>
</template>

<script setup lang="ts">
import Box from '@/components/Box.vue';
import RequestConfig from '@/components/RequestConfig/RequestConfig.vue';
import ResponseDetails from '@/components/ResponseDetails/ResponseDetails.vue';
import { Methods } from '@/core/methods';
import { RequestDetails } from '@/core/request';
import useTabStore from '@/stores/tabs';
import { IconSend2 } from '@tabler/icons-vue';
import { NButton, NIcon, NInput, NInputGroup, NSelect, NSplit } from 'naive-ui';
import { computed } from 'vue';

const tabStore = useTabStore();

const methods = Object.values(Methods).map((method) => ({ label: method, value: method }));
const method = computed(() => tabStore.activeTabContent?.value?.value.config.method);

const updateMethod = (method: string) => {
  tabStore.updateRequest((req: RequestDetails) => {
    req.config.method = method;
  });
};
</script>
