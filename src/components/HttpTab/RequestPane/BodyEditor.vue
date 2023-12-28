<template>
  <Box class="flex gap-2 flex-col">
    <Box class="flex justify-between" height="h-fit">
      <NText strong depth="3" tag="div">Body</NText>
      <Box width="w-32">
        <NSelect :value="props.body.type" size="small" placeholder="Content Type" />
      </Box>
    </Box>
    <CodeMirror v-if="editorBody != null" :code="editorBody" :lineWrap="true" :update="props.update" />
  </Box>
</template>

<script setup lang="ts">
import Box from '@/components/Shared/Box.vue';
import CodeMirror from '@/components/Shared/CodeMirror/CodeMirror.vue';
import { ContentType } from '@/models/common';
import { RequestBody } from '@/models/request';
import { NSelect, NText } from 'naive-ui';
import { computed } from 'vue';

const props = defineProps<{ update: (value: string) => void, body: RequestBody }>();

const editorBody = computed(() => {
  switch (props.body.type) {
    case ContentType.JSON:
    case ContentType.XML:
    case ContentType.TEXT:
      return props.body.data;
    default:
      return null;
  }
});
</script>
