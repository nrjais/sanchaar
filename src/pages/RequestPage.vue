<template>
  <Box>
    <NInputGroup class="flex">
      <Box width="w-32">
        <NSelect size="large" :options="methods" v-model:value="method" filterable tag :consistent-menu-width="false" />
      </Box>
      <NInput size="large" class="flex-grow" placeholder="Address"></NInput>
      <NButton type="success" size="large">
        <NIcon>
          <IconSend2 />
        </NIcon>
      </NButton>
    </NInputGroup>
    <Box>
      <NSplit direction="horizontal" :max="0.75" :min="0.25" :default-size="0.40" class="py-2">
        <template #1>
          <Box class="pr-2">
            <RequestConfig />
          </Box>
        </template>
        <template #2>
          <Box class="pl-2">
            <ResponseDetails />
          </Box>
        </template>
      </NSplit>
    </Box>
  </Box>
</template>

<script setup lang="ts">
import { IconSend2 } from '@tabler/icons-vue';
import { NButton, NIcon, NInput, NInputGroup, NSelect, NSplit } from 'naive-ui';
import { ref } from 'vue';
import Box from '../components/Box.vue';
import RequestConfig from '../components/RequestConfig/RequestConfig.vue';
import { Methods } from '../core/methods';
import ResponseDetails from '../components/ResponseDetails/ResponseDetails.vue';

const methods = Object.values(Methods).map((method) => ({ label: method, value: method }));
const method = ref(Methods.GET);

</script>

<style scoped>
</style>
