<template>
  <Box class="flex">
    <NTabs class="flex-grow" animated size="small">
      <NTabPane name="Params" class="flex-grow">
        <Box class="flex flex-col gap-4 mt-2">
          <KeyVal header="Query Params" :value="activeReq.query" :update="updateQuery" />
          <KeyVal v-if="activeReq.params.length > 0" :value="activeReq.params" :update="updateParams"
            header="Path Variables" />
        </Box>
      </NTabPane>
      <NTabPane name="Headers" class="flex-grow">
        <KeyVal :value="[]" header="Headers" class="mt-2" :update="updateHeaders" />
      </NTabPane>
      <NTabPane name="Body" class="flex-grow">
        Body
      </NTabPane>
    </NTabs>
  </Box>
</template>

<script setup lang="ts">
import { NTabPane, NTabs } from 'naive-ui';
import Box from '../Box.vue';
import KeyVal from '../KeyVal.vue';
import { getRequest, updateRequest } from '@/stores/requests';
import { KeyValue } from '@/core/request';

const props = defineProps<{ tabId: string }>();
const activeReq = getRequest(props.tabId)!;

const updateQuery = (value: KeyValue[]) => {
  console.log("query", value);

  updateRequest(props.tabId, (req) => {
    req.query = value;
  });
};

const updateHeaders = (value: KeyValue[]) => {
  updateRequest(props.tabId, (req) => {
    req.headers = value;
  });
};

const updateParams = (value: KeyValue[]) => {
  updateRequest(props.tabId, (req) => {
    req.params = value;
  });
};

</script>
