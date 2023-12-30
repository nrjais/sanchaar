<template>
  <Box height="h-fit" class="flex flex-col">
    <Box class="flex justify-between">
      <NText strong depth="3" tag="div" class="mb-2">{{ props.header }}</NText>
      <NButtonGroup size="tiny" v-if="!$props.fixed">
        <NButton class="px-2" tertiary type="primary" @click="addRow">
          Add New
        </NButton>
      </NButtonGroup>
    </Box>
    <NTable size="small" bordered :single-line="false" class="flex-grow">
      <thead>
        <tr>
          <th v-if="!$props.fixed"></th>
          <th>Key</th>
          <th>Value</th>
          <th>Description</th>
          <th v-if="!$props.fixed"></th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="row, i in rows">
          <td v-if="!$props.fixed">
            <NCheckbox v-if="!$props.fixed" class="px-2" size="small" v-model:checked="row.enabled" />
          </td>
          <td>
            <NInput placeholder="Key" size="small" v-model:value="row.key" :theme-overrides="themOverides"
              :disabled="$props.fixed" />
          </td>
          <td>
            <NInput placeholder="Value" size="small" v-model:value="row.value" :theme-overrides="themOverides" />
          </td>
          <td>
            <NInput placeholder="Description" size="small" v-model:value="row.description"
              :theme-overrides="themOverides" />
          </td>
          <td v-if="!$props.fixed">
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
import { IconTrash } from '@tabler/icons-vue';
import { InputProps, NButton, NButtonGroup, NCheckbox, NIcon, NInput, NTable, NText } from 'naive-ui';
import { computed, onMounted, reactive, watch, watchEffect } from 'vue';
import Box from './Box.vue';
import { KeyValue } from '@/models/common';

const themOverides: NonNullable<InputProps['themeOverrides']> = {
  color: "#18181c",
  colorDisabled: "#18181c",
  textColorDisabled: "#fff",
}

type Props = {
  header: string;
  value: KeyValue[];
  fixed?: boolean;
  update: (value: KeyValue[]) => void;
};

const props = withDefaults(defineProps<Props>(), { fixed: false });

const rows = reactive<KeyValue[]>(props.value);
const lastRowLeft = computed(() => rows.length == 1)

watch(
  () => rows,
  (value) => {
    props.update(value)
    addIfEmpty();
  },
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
  if (props.fixed) {
    return;
  }
  const allFilled = rows.every((row) => row.key);
  if (allFilled) {
    addRow();
  }
}

const removeRow = (index: number) => {
  rows.splice(index, 1);
};

onMounted(() => {
  addIfEmpty();
});

// Effects
watchEffect(addIfEmpty);
</script>
