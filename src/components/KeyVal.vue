<template>
  <Box height="h-fit">
    <NTable size="small" striped>
      <thead>
        <tr>
          <th></th>
          <th>Key</th>
          <th>Value</th>
          <th>Description</th>
          <th>
            <NButton type="tertiary" size="small" class="px-2 justify-between" :on-click="addRow">
              <NIcon>
                <IconPlus />
              </NIcon>
            </NButton>
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="row in rows">
          <td>
            <NCheckbox class="px-2" v-model:checked="row.enabled" />
          </td>
          <td>
            <NInput placeholder="Key" v-model:value="row.key" />
          </td>
          <td>
            <NInput placeholder="Value" v-model:value="row.value" />
          </td>
          <td>
            <NInput placeholder="Description" v-model:value="row.description" />
          </td>
          <td>
            <NButton type="tertiary" class="px-2" size="small" v-on:click="removeRow(row)" :disabled="lastRowLeft">
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
import { IconPlus, IconTrash } from '@tabler/icons-vue';
import { NButton, NCheckbox, NIcon, NInput, NTable } from 'naive-ui';
import { computed, reactive } from 'vue';
import Box from './Box.vue';

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
