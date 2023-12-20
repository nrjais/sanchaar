<template>
  <Box class="flex">
    <NTabs size="small" animated pane-wrapper-class="flex-grow">
      <NTabPane name="body" tab="Body" class="h-full" display-directive="show:lazy">
        <BodyViewer :code="body" />
      </NTabPane>
      <NTabPane name="headers" tab="Headers">
        <NTable size="small" bordered :single-line="false">
          <thead>
            <tr>
              <th>Name</th>
              <th>Value</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="([key, value]) in headers">
              <td>{{ key }}</td>
              <td width="70%">{{ value }}</td>
            </tr>
          </tbody>
        </NTable>
      </NTabPane>
      <template #suffix>
        <NText :type="statusCodeColor(statusCode)" class="mx-2 font-semibold">{{ statusCode }} {{ statusText }}</NText>
        <NText class="mx-2 font-semibold">{{ latency }}ms</NText>
        <NText class="mx-2 font-semibold">{{ prettyBytes(sizeBytes) }}</NText>
      </template>
    </NTabs>

  </Box>
</template>

<script setup lang="ts">
import { NTabPane, NTable, NTabs, NText } from 'naive-ui';
import prettyBytes from 'pretty-bytes';
import BodyViewer from './BodyViewer.vue';
import Box from '../Box.vue';

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

const body = `{
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
    }
  ]
}`

const headers = {
  'content-type': 'application/json',
  'x-powered-by': 'Express',
  'content-length': '306',
  etag: 'W/"132-+qQ4XQ8Q8Q8Q8Q8Q8Q8Q8Q8Q8Q8"',
  date: 'Thu, 01 Jul 2021 15:01:01 GMT',
  connection: 'close'
}

const statusCode = 200;
const statusText = 'OK';
const latency = 112;
const sizeBytes = 123500;
</script>
