<template>
  <NTabs size="small" animated class="h-full" type="line" pane-wrapper-class="h-0 max-h-full flex-grow flex flex-col">
    <NTabPane name="body" tab="Body" display-directive="show" class="flex-grow h-0">
      <BodyViewer :code="body" />
    </NTabPane>
    <NTabPane v-if="result?.headers" name="headers" tab="Headers" display-directive="show:lazy" class="flex-grow h-0">
      <ScrollBox>
        <NTable size="small" bordered :single-line="false">
          <thead>
            <tr>
              <th>Name</th>
              <th>Value</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="header in result.headers">
              <td>{{ header.key }}</td>
              <td width="65%">{{ header.value }}</td>
            </tr>
          </tbody>
        </NTable>
      </ScrollBox>
    </NTabPane>
    <template #suffix>
      <Box v-if="result" class="flex gap-4 text-xs items-center font-semibold">
        <NText :type="statusCodeColor(result.code)">
          {{ result.text }}
        </NText>
        <NText depth="2">
          Time: <NText type="info">{{ prettyMillis(result.latency) }}</NText>
        </NText>
        <NText depth="2">
          Size: <NText type="info">{{ prettyBytes(result.length) }}</NText>
        </NText>
      </Box>
    </template>
  </NTabs>
</template>

<script setup lang="ts">
import { NTabPane, NTable, NTabs, NText } from 'naive-ui';
import prettyBytes from 'pretty-bytes';
import BodyViewer from './BodyViewer.vue';
import Box from '@/components/Shared/Box.vue';
import ScrollBox from '@/components/Shared/ScrollBox.vue';
import { useRequestStore } from '@/stores/requests';
import { computed } from 'vue';
import { defineProps } from 'vue';
import { prettyMillis } from '@/utils/prettyMs';

const props = defineProps<{ tabId: string }>();

const requestStore = useRequestStore();

const response = computed(() => requestStore.getExecutionResult(props.tabId));
const result = computed(() => {
  const result = response?.value;
  switch (result.state) {
    case 'cancelled':
      return {
        code: 0,
        text: 'Cancelled',
        length: 0,
        latency: 0,
      };
    case 'completed':
      const response = result.response;
      const statusText = response.statusText ? ` • ${response.statusText}` : "";
      return {
        code: result.response.status,
        text: `${response.status}${statusText}`,
        length: response.contentLength,
        latency: response.latency,
        headers: response.headers,
      };
    default:
      return null;
  }
});

const statusCodeColor = (code: number) => {
  if (code >= 200 && code < 300) {
    return 'success'
  }

  if (code >= 300 && code < 400) {
    return 'warning'
  }

  if (code >= 400) {
    return 'error'
  }

  return 'default'
}

const body = JSON.stringify({
  "name": "John",
  "age": 30,
  "cars": [
    {
      "name": "Ford",
      "models": ["Fiesta", "Focus", "Mustang"]
    },
    {
      "name": "BMW",
      "models": ["320", "X3", "X5"]
    },
    {
      "name": "Fiat",
      "models": ["500", "Panda"]
    },
    {
      "name": "BMW",
      "models": ["320", "X3", "X5"]
    },
    {
      "name": "Fiat",
      "models": ["500", "Panda"]
    },
    {
      "name": "BMW",
      "models": ["320", "X3", "X5"]
    },
    {
      "name": "Fiat",
      "models": ["500", "Panda"]
    },
    {
      "name": "BMW",
      "models": ["320", "X3", "X5"]
    },
    {
      "name": "Fiat",
      "models": ["500", "Panda"]
    },
    {
      "name": "BMW",
      "models": ["320", "X3", "X5"]
    },
    {
      "name": "Fiat",
      "models": ["500", "Panda"]
    },
    {
      "name": "BMW",
      "models": ["320", "X3", "X5"]
    },
    {
      "name": "Fiat",
      "models": ["500", "Panda"]
    }
  ]
});
</script>
