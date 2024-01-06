<template>
  <Box class="flex flex-col">
    <NInputGroup class="flex">
      <Box width="w-28">
        <NSelect :options="methods" v-model:value="method" :on-update-value="updateMethod" :consistent-menu-width="false"
          filterable tag />
      </Box>
      <SingleLineEditor :value="address" :update="updateAddress"></SingleLineEditor>
      <NButton type="success" @click="sendRequest">
        Send
      </NButton>
    </NInputGroup>
    <NSplit direction="horizontal" :max="0.75" :min="0.25" :default-size="0.40" class="pt-2 flex-grow">
      <template #1>
        <RequestPane :tabId="props.tabId" class="pr-2" />
      </template>
      <template #2>
        <ResponsePane :tabId="tabId" class="pl-2" />
      </template>
    </NSplit>
  </Box>
</template>

<script setup lang="ts">
import { parsePathParams } from '@/backend/preprocess/pathParams';
import { Methods } from '@/models/methods';
import { useRequestStore } from '@/stores/requests';
import { NButton, NInputGroup, NSelect, NSplit } from 'naive-ui';
import { computed } from 'vue';
import Box from '../Shared/Box.vue';
import SingleLineEditor from '../Shared/SingleLineEditor/SingleLineEditor.vue';
import RequestPane from './RequestPane/RequestPane.vue';
import ResponsePane from './ResponsePane/ResponsePane.vue';

const props = defineProps<{ tabId: string }>();
const reqStore = useRequestStore();

const activeReq = computed(() => reqStore.getRequest(props.tabId)!);
const method = computed(() => activeReq.value.method);
const address = computed(() => reqStore.getRequestAddress(props.tabId));

const updateMethod = (value: string) => {
  reqStore.updateRequest(props.tabId, (req) => {
    req.method = value as Methods;
  });
};

const updateAddress = (value: string) => {
  reqStore.updateRequest(props.tabId, (req) => {
    req.params = parsePathParams(value);
    req.address = value;
  });
};

const methods = Object.values(Methods).map((method) => ({
  label: method, value: method,
}));

const sendRequest = () => reqStore.executeRequest(props.tabId);
</script>
