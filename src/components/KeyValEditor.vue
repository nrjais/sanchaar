<template>
  <Box height="h-fit" class="flex flex-col">
    <Box class="flex justify-between">
      <NText strong depth="3" tag="div" class="mb-2">{{ props.header }}</NText>
      <NButtonGroup size="tiny">
        <NButton class="px-2" tertiary type="primary" @click="addRow">
          New
        </NButton>
      </NButtonGroup>
    </Box>
    <NTable size="small" bordered :single-line="false" class="flex-grow">
      <thead>
        <tr>
          <th></th>
          <th>Key</th>
          <th>Value</th>
          <th>Description</th>
          <th></th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="row, i in rows">
          <td>
            <NCheckbox class="px-2" size="small" v-model:checked="row.enabled" />
          </td>
          <td>
            <NInput placeholder="Key" size="small" v-model:value="row.key" :theme-overrides="themOverides" />
          </td>
          <td>
            <NInput placeholder="Value" size="small" v-model:value="row.value" :theme-overrides="themOverides" />
          </td>
          <td>
            <NInput placeholder="Description" size="small" v-model:value="row.description"
              :theme-overrides="themOverides" />
          </td>
          <td>
            <NButton type="error" quaternary class="px-2" size="tiny" v-on:click="removeRow(i)" :disabled="lastRowLeft">
              <NIcon>
                <IconTrash />
              </NIcon>
            </NButton>
          </td>
        </tr>
      </tbody>
    </NTable>
  </Box>
</template>

<script setup lang="ts">
import { KeyValue } from '@/core/request';
import { IconTrash } from '@tabler/icons-vue';
import { InputProps, NButton, NButtonGroup, NCheckbox, NIcon, NInput, NTable, NText } from 'naive-ui';
import { computed, onMounted, reactive, watch } from 'vue';
import Box from './Box.vue';

const themOverides: NonNullable<InputProps['themeOverrides']> = {
  color: "#18181c"
}

type Props = {
  header: string;
  value: KeyValue[];
  update: (value: KeyValue[]) => void;
};

const props = defineProps<Props>();

const rows = reactive<KeyValue[]>(props.value);
const lastRowLeft = computed(() => rows.length == 1)

watch(
  () => rows,
  (value) => props.update(value),
  { deep: true }
);

const addRow = () => {
  rows.push({
    enabled: true,
    key: '',
    value: '',
  });
};

const addIfEmpty = () => {
  if (rows.length == 0) {
    addRow();
  }
};

const removeRow = (index: number) => {
  rows.splice(index, 1);
  addIfEmpty();
};

onMounted(() => {
  addIfEmpty();
});

</script>
