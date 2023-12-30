<template>
  <Box class="flex gap-2 flex-col">
    <Box class="flex justify-between" height="h-fit">
      <NText strong depth="3" tag="div">Body</NText>
      <Box width="w-36">
        <NSelect :value="props.body.type" @update:value="changeContentType" size="small" placeholder="Content Type"
          :options="contentTypes" />
      </Box>
    </Box>
    <CodeMirror v-if="editorBody.editor == 'code'" :code="editorBody.data" :lineWrap="true" :update="updateBody" />
  </Box>
</template>

<script setup lang="ts">
import Box from '@/components/Shared/Box.vue';
import CodeMirror from '@/components/Shared/CodeMirror/CodeMirror.vue';
import { ContentType, KeyValue } from '@/models/common';
import { RequestBody } from '@/models/request';
import { NSelect, NText } from 'naive-ui';
import { computed, ref } from 'vue';

const props = defineProps<{ body: RequestBody, update: (type: RequestBody) => void }>();

type EditorType = "code" | "keyval" | "file" | "none";
type EditorBodyType = { editor: "code", data: string }
  | { editor: "keyval", data: KeyValue[] }
  | { editor: "file", data: null }
  | { editor: "none", data: null };

const editorType = ref<EditorType>("code");

const editorBody = computed((): EditorBodyType => {
  switch (props.body.type) {
    case ContentType.JSON:
    case ContentType.XML:
    case ContentType.TEXT:
      return { editor: "code", data: props.body.data };
    case ContentType.URL_ENCODED:
    case ContentType.MUTLIPART_FORM:
      return { editor: "keyval", data: props.body.data };
    default:
      return { editor: "none", data: null };
  }
});

const contentTypes: {
  label: string,
  editor: EditorType,
  value: ContentType,
  body: RequestBody,
}[] = [
    { label: 'JSON', value: ContentType.JSON, editor: "code", body: { type: ContentType.JSON, data: "" } },
    { label: 'XML', value: ContentType.XML, editor: "code", body: { type: ContentType.XML, data: "" } },
    { label: 'Raw Text', value: ContentType.TEXT, editor: "code", body: { type: ContentType.TEXT, data: "" } },
    { label: 'URL Encoded', value: ContentType.URL_ENCODED, editor: "keyval", body: { type: ContentType.URL_ENCODED, data: [] as KeyValue[] } },
    { label: 'Multipart Form', value: ContentType.MUTLIPART_FORM, editor: "keyval", body: { type: ContentType.MUTLIPART_FORM, data: [] as KeyValue[] } },
    { label: 'Binary', value: ContentType.BLOB, editor: "file", body: { type: ContentType.BLOB, data: new Blob() } },
    { label: 'None', value: ContentType.NONE, editor: "none", body: { type: ContentType.NONE } }
  ];

const updateBody = (value: string) => {
  switch (props.body.type) {
    case ContentType.JSON:
    case ContentType.XML:
    case ContentType.TEXT:
      props.update({ type: props.body.type, data: value });
      break;
    case ContentType.URL_ENCODED:
    case ContentType.MUTLIPART_FORM:
      props.update({ type: props.body.type, data: [] as KeyValue[] });
      break;
    default:
      break;
  }
}

const changeContentType = (type: ContentType) => {
  const contentType = contentTypes.find(x => x.value == type);
  if (contentType) {
    editorType.value = contentType.editor;
    props.update(contentType.body);
  }
}
</script>
