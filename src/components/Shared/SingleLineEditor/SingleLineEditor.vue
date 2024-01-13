<template>
  <div ref="editorRef" class="overflow-scroll h-full flex-grow codemirror"></div>
</template>

<script setup lang="ts">
import { autocompletion, closeBrackets, completionKeymap } from "@codemirror/autocomplete";
import { defaultKeymap, history, historyKeymap } from "@codemirror/commands";
import {
  defaultHighlightStyle,
  syntaxHighlighting
} from "@codemirror/language";
import { EditorState, Extension } from "@codemirror/state";
import {
  EditorView,
  ViewUpdate,
  drawSelection,
  dropCursor,
  keymap
} from "@codemirror/view";
import { androidstudio } from '@uiw/codemirror-themes-all';
import { onBeforeUnmount, onMounted, ref, watchEffect } from 'vue';

const props = defineProps<{ value: string, update?: (value: string) => void }>();

const editorRef = ref()
const editor = ref<EditorView | null>(null)
const lastUpdated = ref(props.value)

const fontTheme = EditorView.theme({ '&': { fontSize: "120%" } });

const update = (update: ViewUpdate) => {
  if (!update.docChanged) {
    return
  }
  const doc = update.state.doc.toString()
  if (doc === lastUpdated.value) return
  lastUpdated.value = doc
  props.update?.(doc)
}

const config = [
  EditorView.updateListener.of(update),
  EditorState.transactionFilter.of(tr => tr.newDoc.lines > 1 ? [] : tr)
]

const basicSetup: Extension = [
  history(),
  drawSelection(),
  dropCursor(),
  syntaxHighlighting(defaultHighlightStyle, { fallback: true }),
  closeBrackets(),
  autocompletion(),
  ...config,
  keymap.of([
    ...defaultKeymap,
    ...historyKeymap,
    ...completionKeymap,
  ]),
]

onMounted(() => {
  editor.value = new EditorView({
    doc: props.value,
    extensions: [
      basicSetup,
      androidstudio,
      fontTheme
    ],
    parent: editorRef.value!,
  })
})

watchEffect(() => {
  const doc = props.value
  if (doc === lastUpdated.value) return
  editor.value?.dispatch({
    changes: {
      from: 0,
      to: editor.value.state.doc.length,
      insert: doc
    }
  })
})

onBeforeUnmount(() => editor.value?.destroy())
</script>

<style scoped lang="scss">
:deep(.cm-editor) {
  height: 100%;
  width: 100%;
  outline: none;

  .cm-scroller {
    font-family: inherit;
    color: #d5d1d1d1;
  }
}

:deep(.cm-focused) {
  outline: none;
  color: #ffffffd1;
}
</style>
