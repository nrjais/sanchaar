<template>
  <NTabs type="line" animated size="small" :class="props.class" class="h-full"
    pane-wrapper-class="h-0 max-h-full flex-grow flex flex-col">
    <NTabPane name="Params" display-directive="show:lazy" class="flex-grow h-0">
      <ScrollBox class="flex flex-col gap-4 pt-2">
        <KeyValEditor header="Query Params" :value="activeReq.query" :update="updateQuery" />
        <KeyValEditor v-if="hasPathParams" :value="activeReq.params" :update="updateParams" fixed
          header="Path Variables" />
      </ScrollBox>
    </NTabPane>
    <NTabPane name="Headers" display-directive="show:lazy" class="flex-grow h-0">
      <KeyValEditor :value="activeReq.headers" header="Headers" class="mt-2" :update="updateHeaders" />
    </NTabPane>
    <NTabPane name="Body" display-directive="show:lazy" class="flex-grow h-0">
      Body
    </NTabPane>
  </NTabs>
</template>

<script setup lang="ts">
import { useRequestStore } from '@/stores/requests';
import { NTabPane, NTabs } from 'naive-ui';
import KeyValEditor from '@/components/Shared/KeyValEditor.vue';
import { computed } from 'vue';
import { KeyValue } from '@/models/request';
import ScrollBox from '@/components/Shared/ScrollBox.vue';

const props = defineProps<{ tabId: string, class?: string }>();
const { getRequest, updateRequest } = useRequestStore();

const activeReq = computed(() => getRequest(props.tabId)!);

const hasPathParams = computed(() => activeReq.value.params.length > 0);

const updateQuery = (value: KeyValue[]) => {
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
