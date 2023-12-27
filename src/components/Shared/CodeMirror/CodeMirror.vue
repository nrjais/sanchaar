<template>
  <ScrollBox>
    <div ref="editorRef" class=" overflow-scroll flex-grow codemirror"></div>
  </ScrollBox>
</template>

<script setup lang="ts">
import ScrollBox from "@/components/Shared/ScrollBox.vue";
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
import { vscodeDark } from '@uiw/codemirror-theme-vscode';
import { onMounted, onUnmounted, ref, watch } from 'vue';

const props = defineProps<{ code: string, lineWrap: boolean, readOnly: boolean }>();

const editorRef = ref(null)
const editor = ref<EditorView | null>(null)

const lineWrappingComp = new Compartment()
const config = [
  EditorState.readOnly.of(props.readOnly),
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
    doc: props.code,
    extensions: [
      basicSetup,
      vscodeDark
    ],
    parent: editorRef.value!,
  })
})

watch(() => props.code, (doc) => {
  editor.value?.dispatch({
    changes: {
      from: 0,
      to: editor.value.state.doc.length,
      insert: doc
    }
  })
})

watch(() => props.lineWrap, (lineWrap) => {
  editor.value?.dispatch({
    effects: lineWrappingComp.reconfigure(lineWrap ? EditorView.lineWrapping : [])
  });
})

onUnmounted(() => {
  editor.value?.destroy()
})
</script>
