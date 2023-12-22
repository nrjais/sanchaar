<template>
  <Box class="flex">
    <NTabs size="small" animated>
      <NTabPane name="body" tab="Body" class="h-full" display-directive="show:lazy">
        <Box class="flex flex-col">
          <Box class="flex-grow">
            <BodyViewer :code="body" />
          </Box>
        </Box>
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
        <Box class="flex gap-4 text-xs items-center font-semibold">
          <NText :type="statusCodeColor(statusCode)">
            {{ statusCode }}&#8226;{{ statusText }}
          </NText>
          <NText depth="2">
            Time: <NText type="info">{{ latency }}ms</NText>
          </NText>
          <NText depth="2">
            Size: <NText type="info">{{ prettyBytes(sizeBytes) }}</NText>
          </NText>
        </Box>
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
