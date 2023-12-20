<template>
  <Box class="flex">
    <NTabs class="flex-grow" animated size="small">
      <NTabPane name="Params" class="flex-grow">
        <Box>
          <NText strong depth="3" tag="div" class="mb-2">Query Params</NText>
          <KeyVal />
        </Box>
        <Box v-if="props.pathParams?.length">
          <NText strong depth="3" tag="div" class="mb-2 mt-4">Path Variables</NText>
          <KeyVal />
        </Box>
      </NTabPane>
      <NTabPane name="Headers" class="flex-grow">
        <KeyVal />
      </NTabPane>
      <NTabPane name="Body" class="flex-grow">
        Body
      </NTabPane>
    </NTabs>
  </Box>
</template>

<script setup lang="ts">
import { NTabs, NTabPane, NText } from 'naive-ui';
import Box from '../Box.vue';
import KeyVal from '../KeyVal.vue';

interface Props {
  pathParams: string[];
}

const props = withDefaults(defineProps<Props>(), {
  pathParams: () => ["Test"],
});

</script>
