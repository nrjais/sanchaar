<template>
  <Box class="flex flex-col">
    <NInputGroup class="flex">
      <Box width="w-32">
        <NSelect size="large" :options="methods" v-model:value="method" :on-update-value="updateMethod"
          :consistent-menu-width="false" filterable tag />
      </Box>
      <NInput size="large" class="flex-grow" placeholder="Address"></NInput>
      <NButton size="large" type="success">
        <NIcon>
          <IconSend2 />
        </NIcon>
      </NButton>
    </NInputGroup>
    <NSplit direction="horizontal" :max="0.75" :min="0.25" :default-size="0.40" class="pt-2 flex-grow">
      <template #1>
        <RequestConfig :tabId="props.tabId" class="pr-2" />
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
import { getRequest, updateRequest } from '@/stores/requests';
import { IconSend2 } from '@tabler/icons-vue';
import { NButton, NIcon, NInput, NInputGroup, NSelect, NSplit } from 'naive-ui';
import { ref } from 'vue';

const props = defineProps<{ tabId: string }>();

const activeReq = getRequest(props.tabId)!;

const method = ref(activeReq.method);

const updateMethod = (value: string) => {
  updateRequest(props.tabId, (req) => {
    req.method = value as Methods;
  });
};

const methods = Object.values(Methods).map((method) => ({ label: method, value: method }));

</script>
