<template>
  <NButtonGroup class="my-1" size="small">
    <NButton secondary :on-click="prettify" :type="buttonType(viewMode, 'pretty')">Pretty</NButton>
    <NButton secondary :on-click="rawify" :type="buttonType(viewMode, 'raw')">Raw</NButton>
    <NButton secondary :on-click="toggleWrapping" :type="buttonType(wrappingMode, 'on')">
      <NIcon>
        <IconTextWrap />
      </NIcon>
    </NButton>
  </NButtonGroup>
  <Box class="my-2">
    <VueMonacoEditor :options="MONACO_EDITOR_OPTIONS" @mount="handleMount" :language="language" />
  </Box>
</template>

<script setup lang="ts">
import { VueMonacoEditor } from "@guolao/vue-monaco-editor";
import { IconTextWrap } from "@tabler/icons-vue";
import * as monacoEditor from 'monaco-editor/esm/vs/editor/editor.api';
import { NButton, NButtonGroup, NIcon } from 'naive-ui';
import { computed, ref, shallowRef } from 'vue';
import Box from "../Box.vue";

type ViewMode = "pretty" | "raw"

const editorRef = shallowRef<monacoEditor.editor.IStandaloneCodeEditor>()
const viewMode = ref<ViewMode>('pretty')
const wrappingMode = ref<"on" | "off">('on')

type Props = {
  code: string;
  language: string;
}

const props = withDefaults(defineProps<Props>(), {
  code: "",
  language: "json"
});

const MONACO_EDITOR_OPTIONS = ref({
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
  wordWrap: wrappingMode.value,
} satisfies monacoEditor.editor.IStandaloneEditorConstructionOptions);

const prettyCode = computed(() => {
  return JSON.stringify(JSON.parse(props.code), null, 2)
})

const prettify = () => {
  viewMode.value = 'pretty'
  editorRef.value?.setValue(prettyCode.value)
}

const rawify = () => {
  viewMode.value = 'raw'
  editorRef.value?.setValue(props.code)
}

const handleMount = (editor: any) => {
  editorRef.value = editor
  viewMode.value = 'pretty'
  prettify()
}

const buttonType = <T>(val: T, expected: T) => {
  return val == expected ? 'primary' : 'default'
}

const toggleWrapping = () => {
  wrappingMode.value = wrappingMode.value == 'on' ? 'off' : 'on'
  editorRef.value?.updateOptions({ wordWrap: wrappingMode.value })
}
</script>
