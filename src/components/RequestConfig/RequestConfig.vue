<template>
  <Box class="flex">
    <NTabs class="flex-grow" animated size="small">
      <NTabPane name="Params" class="flex-grow">
        <Box class="flex flex-col gap-4 mt-2">
          <KeyVal header="Query Params" />
          <KeyVal v-if="props.pathParams.length > 0" header="Path Variables" />
        </Box>
      </NTabPane>
      <NTabPane name="Headers" class="flex-grow">
        <KeyVal header="Headers" class="mt-2"/>
      </NTabPane>
      <NTabPane name="Body" class="flex-grow">
        Body
      </NTabPane>
    </NTabs>
  </Box>
</template>

<script setup lang="ts">
import { NTabPane, NTabs } from 'naive-ui';
import Box from '../Box.vue';
import KeyVal from '../KeyVal.vue';

interface Props {
  pathParams: string[];
}

const props = withDefaults(defineProps<Props>(), {
  pathParams: () => ["Test"],
});

</script>
