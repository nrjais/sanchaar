<template>
  <Box height="h-fit">
    <Box class="flex justify-between">
      <NText strong depth="3" tag="div" class="mb-2">{{ props.header }}</NText>
      <NButtonGroup size="tiny">
        <NButton class="px-2" quaternary type="primary" @click="addRow">
          Bulk Edit
        </NButton>
        <NButton class="px-2" quaternary type="primary" @click="addRow">
          Add New
        </NButton>
      </NButtonGroup>
    </Box>
    <NTable size="small" bordered :single-line="false">
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
        <tr v-for="row in rows">
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
            <NButton type="error" quaternary class="px-2" size="tiny" v-on:click="removeRow(row)" :disabled="lastRowLeft">
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
import { computed, reactive } from 'vue';
import Box from './Box.vue';

const themOverides: NonNullable<InputProps['themeOverrides']> = {
  color: "#18181c"
}

const props = defineProps({
  header: {
    type: String,
    default: 'Query Params',
  },
});

type KeyVal = {
  enabled: boolean;
  key: string;
  value: string;
  description: string;
};

const rows = reactive([] as KeyVal[]);

const removeRow = (row: KeyVal) => {
  if (rows.length === 1) {
    return;
  }
  const index = rows.indexOf(row);
  rows.splice(index, 1);
};

const lastRowLeft = computed(() => rows.length == 1)

const addRow = () => {
  rows.push({
    enabled: true,
    key: '',
    value: '',
    description: '',
  });
};
addRow();
</script>
