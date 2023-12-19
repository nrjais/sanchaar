<template>
  <Box>
    <NInputGroup class="flex">
      <Box width="w-32">
        <NSelect :options="methods" v-model:value="method" />
      </Box>
      <NInput class="flex-grow" placeholder="Address"></NInput>
      <NButton type="success">
        <NIcon>
          <IconSend2 />
        </NIcon>
      </NButton>
      <NButton type="success">
        <NIcon>
          <IconDeviceFloppy />
        </NIcon>
      </NButton>
    </NInputGroup>
    <Box>
      <NSplit direction="horizontal" :max="0.75" :min="0.25">
        <template #1>
          <Box>
            Pane 1
          </Box>
        </template>
        <template #2>
          <Box>
            Pane 2
          </Box>
        </template>
      </NSplit>
    </Box>
  </Box>
</template>

<script setup lang="ts">
import { IconDeviceFloppy } from '@tabler/icons-vue';
import { IconSend2 } from '@tabler/icons-vue';
import { NButton, NIcon, NInput, NInputGroup, NSelect, NSpace, NSplit } from 'naive-ui';
import { ref } from 'vue';
import Box from '../components/Box.vue';

const methods = [
  { label: 'GET', value: 'GET' },
  { label: 'POST', value: 'POST' },
  { label: 'PUT', value: 'PUT' },
  { label: 'DELETE', value: 'DELETE' },
  { label: 'PATCH', value: 'PATCH' },
  { label: 'HEAD', value: 'HEAD' },
  { label: 'OPTIONS', value: 'OPTIONS' },
  { label: 'CONNECT', value: 'CONNECT' },
  { label: 'TRACE', value: 'TRACE' }
]
const method = ref(methods[0].value);

</script>

<style scoped>
</style>
