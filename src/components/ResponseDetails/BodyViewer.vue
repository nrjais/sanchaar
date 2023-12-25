<template>
  <Box class="flex flex-col">
    <Box class="flex" height="h-fit">
      <NButtonGroup class="my-1" size="small">
        <NButton secondary @click="viewMode = 'pretty'" :type="buttonType(viewMode, 'pretty')">Pretty</NButton>
        <NButton secondary @click="viewMode = 'raw'" :type="buttonType(viewMode, 'raw')">Raw</NButton>
      </NButtonGroup>
      <NButtonGroup class="my-1 ml-2" size="small">
        <NButton secondary :on-click="toggleWrapping" :type="buttonType(lineWrap)">
          <NIcon>
            <IconTextWrap />
          </NIcon>
        </NButton>
        <NButton secondary :on-click="copyToClipboard">
          <NIcon>
            <IconCopy />
          </NIcon>
        </NButton>
      </NButtonGroup>
    </Box>
    <div ref="editorRef" class="mt-2 overflow-scroll flex-grow"></div>
  </Box>
</template>

<script setup lang="ts">
import { autocompletion, closeBrackets, closeBracketsKeymap, completionKeymap } from "@codemirror/autocomplete";
import { defaultKeymap, history, historyKeymap } from "@codemirror/commands";
import { json } from '@codemirror/lang-json';
import {
  bracketMatching,
  defaultHighlightStyle,
  foldGutter, foldKeymap,
  indentOnInput,
  syntaxHighlighting
} from "@codemirror/language";
import { lintKeymap } from "@codemirror/lint";
import { highlightSelectionMatches, searchKeymap } from "@codemirror/search";
import { Compartment, EditorState, Extension } from "@codemirror/state";
import { EditorView, crosshairCursor, drawSelection, dropCursor, highlightActiveLine, highlightActiveLineGutter, highlightSpecialChars, keymap, lineNumbers, rectangularSelection } from "@codemirror/view";
import { IconTextWrap } from "@tabler/icons-vue";
import { vscodeDark } from '@uiw/codemirror-theme-vscode';
import { NButton, NButtonGroup, NIcon } from 'naive-ui';
import { computed, onMounted, onUnmounted, ref, watch } from 'vue';
import Box from "../Box.vue";
import { IconCopy } from "@tabler/icons-vue";

type ViewMode = "pretty" | "raw"

const viewMode = ref<ViewMode>('pretty')
const lineWrap = ref(true);
const editorRef = ref(null)
const editor = ref<EditorView | null>(null)

const lineWrappingComp = new Compartment()
const config = [
  EditorState.readOnly.of(true),
  EditorState.tabSize.of(2),
  EditorState.allowMultipleSelections.of(true),
  lineWrappingComp.of(EditorView.lineWrapping),
]

const basicSetup: Extension = [
  lineNumbers(),
  highlightActiveLineGutter(),
  highlightSpecialChars(),
  history(),
  foldGutter(),
  drawSelection(),
  dropCursor(),
  indentOnInput(),
  syntaxHighlighting(defaultHighlightStyle, { fallback: true }),
  bracketMatching(),
  closeBrackets(),
  autocompletion(),
  rectangularSelection(),
  crosshairCursor(),
  highlightActiveLine(),
  highlightSelectionMatches(),
  json(),
  ...config,
  keymap.of([
    ...closeBracketsKeymap,
    ...defaultKeymap,
    ...searchKeymap,
    ...historyKeymap,
    ...foldKeymap,
    ...completionKeymap,
    ...lintKeymap
  ]),
]

onMounted(() => {
  editor.value = new EditorView({
    doc: code.value,
    extensions: [
      basicSetup,
      vscodeDark
    ],
    parent: editorRef.value!,
  })
})

type Props = {
  code: string;
}
const props = withDefaults(defineProps<Props>(), {
  code: "",
});

const prettyCode = computed(() => {
  return JSON.stringify(JSON.parse(props.code), null, 2)
})

const code = computed(() => {
  return viewMode.value == 'pretty' ? prettyCode.value : props.code
})

const buttonType = <T>(val: T, expected?: T) => {
  return (val == expected) || (!expected && val) ? 'primary' : 'default'
}

const toggleWrapping = () => {
  lineWrap.value = !lineWrap.value;
  editor.value?.dispatch({
    effects: lineWrappingComp.reconfigure(lineWrap.value ? EditorView.lineWrapping : [])
  });
}

const copyToClipboard = () => {
  const codeToCopy = viewMode.value == 'pretty' ? prettyCode.value : props.code;
  navigator.clipboard.writeText(codeToCopy)
}

watch(() => code.value, (doc) => {
  editor.value?.dispatch({
    changes: {
      from: 0,
      to: editor.value.state.doc.length,
      insert: doc
    }
  })
})

onUnmounted(() => {
  editor.value?.destroy()
})
</script>
