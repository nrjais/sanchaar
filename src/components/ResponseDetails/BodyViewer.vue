<template>
  <NRadioGroup :on-update-value="updateBodyView" v-model:value="viewMode" size="small" class="my-2">
    <NRadioButton label="Pretty" value="pretty" />
    <NRadioButton label="Raw" value="raw" />
  </NRadioGroup>
  <Box class="my-2">
    <VueMonacoEditor :options="MONACO_EDITOR_OPTIONS" @mount="handleMount" :language="language" />
  </Box>
</template>

<script setup lang="ts">
import {
  VueMonacoEditor,
} from "@guolao/vue-monaco-editor";
import * as monacoEditor from 'monaco-editor/esm/vs/editor/editor.api';
import { NRadioButton, NRadioGroup } from 'naive-ui';
import { computed, ref, shallowRef } from 'vue';
import Box from "../Box.vue";

const editorRef = shallowRef<monacoEditor.editor.IStandaloneCodeEditor>()
const viewMode = ref('pretty')

type Props = {
  code: string;
  language: string;
}

const props = withDefaults(defineProps<Props>(), {
  code: "",
  language: "json"
});

const MONACO_EDITOR_OPTIONS = {
  automaticLayout: true,
  formatOnType: true,
  formatOnPaste: true,
  fontSize: 14,
  tabSize: 2,
  contextmenu: false,
  minimap: {
    enabled: false
  },
  theme: 'vs-dark',
  language: props.language,
  readOnly: true,
  scrollBeyondLastLine: false,
} satisfies monacoEditor.editor.IStandaloneEditorConstructionOptions;

const prettyCode = computed(() => {
  return JSON.stringify(JSON.parse(props.code), null, 2)
})

const updateBodyView = (mode: string) => {
  if (mode === 'pretty') {
    editorRef.value?.setValue(prettyCode.value)
  }

  if (mode === 'raw') {
    editorRef.value?.setValue(props.code)
  }
}

const handleMount = (editor: any) => {
  editorRef.value = editor
  updateBodyView(viewMode.value)
}
</script>
