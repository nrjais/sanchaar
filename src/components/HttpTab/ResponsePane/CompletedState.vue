<template>
  <NTabs size="small" animated class="h-full" type="line" pane-wrapper-class="h-0 max-h-full flex-grow flex flex-col">
    <NTabPane name="body" tab="Body" display-directive="show"
      class="flex-grow h-0">
      <BodyViewer :body="result.body" />
    </NTabPane>
    <NTabPane name="headers" tab="Headers" display-directive="show:lazy" class="flex-grow h-0">
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
      <Box class="flex gap-4 text-xs items-center font-semibold">
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

<script lang="ts" setup>
import Box from '@/components/Shared/Box.vue';
import ScrollBox from '@/components/Shared/ScrollBox.vue';
import { ResponseDetails } from '@/models/response';
import { prettyMillis } from '@/utils/prettyMs';
import { NTabPane, NTable, NTabs, NText } from 'naive-ui';
import prettyBytes from 'pretty-bytes';
import { computed, defineProps } from 'vue';
import BodyViewer from './BodyViewer.vue';

const props = defineProps<{ response: ResponseDetails }>();

const result = computed(() => {
  const response = props.response;
  const statusText = response.statusText ? ` • ${response.statusText}` : "";

  return {
    code: response.status,
    text: `${response.status}${statusText}`,
    length: response.contentLength,
    latency: response.latency,
    headers: response.headers,
    body: response.content,
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
</script>
