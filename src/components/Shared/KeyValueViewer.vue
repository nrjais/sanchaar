<template>
  <Box>
    <NText v-if="props.header" strong depth="3" tag="div" class="mb-2">{{ props.header }}</NText>
    <NTable size="small" bordered :single-line="false">
      <thead>
        <tr>
          <th>Name</th>
          <th>Value</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="kv in props.values">
          <td>{{ kv.key }}</td>
          <td width="65%">{{ kv.value }}</td>
        </tr>
      </tbody>
    </NTable>
  </Box>
</template>

<script setup lang="ts">
import { NTable, NText } from 'naive-ui';
import Box from './Box.vue';

const props = defineProps<{ values: { key: string, value: string }[], header?: string }>();
</script>
