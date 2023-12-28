<template>
  <ScrollBox>
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
  </ScrollBox>
</template>

<script setup lang="ts">
import ScrollBox from '@/components/Shared/ScrollBox.vue';
import { NTable } from 'naive-ui';

const props = defineProps<{ values: { key: string, value: string }[] }>();

</script>
