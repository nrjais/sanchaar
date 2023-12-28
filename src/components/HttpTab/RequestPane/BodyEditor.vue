<template>
  <Box class="flex gap-2 flex-col">
    <Box class="flex justify-between" height="h-fit">
      <NText strong depth="3" tag="div">Body</NText>
      <Box width="w-32">
        <NSelect :value="props.body.type" @change="props.changeType" size="small" placeholder="Content Type"
          :options="contentTypes" />
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

const props = defineProps<{
  body: RequestBody,
  update: (value: string) => void,
  changeType: (type: ContentType) => void
}>();

const contentTypes = [
  { label: 'JSON', value: ContentType.JSON },
  { label: 'XML', value: ContentType.XML },
  { label: 'Text', value: ContentType.TEXT },
  { label: 'UrlEncoded', value: ContentType.URL_ENCODED },
  { label: 'Multipart', value: ContentType.MUTLIPART_FORM },
  { label: 'Blob', value: ContentType.BLOB },
  { label: 'None', value: ContentType.NONE },
];

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
