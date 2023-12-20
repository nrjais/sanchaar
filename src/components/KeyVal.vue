<template>
  <Box height="h-fit">
    <NTable size="small" striped>
      <thead>
        <tr>
          <th></th>
          <th>Key</th>
          <th>Value</th>
          <th>Description</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="_row in rows">
          <td class="p-1">
            <NCheckbox class="px-2" />
          </td>
          <td>
            <NInput placeholder="Key" />
          </td>
          <td>
            <NInput placeholder="Value" />
          </td>
          <td>
            <NInput placeholder="Description" />
          </td>
          <td>
            <NButton type="tertiary">
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
import { NButton, NCheckbox, NIcon, NInput, NTable } from 'naive-ui';
import Box from './Box.vue';

const rows = [1, 2]
</script>
