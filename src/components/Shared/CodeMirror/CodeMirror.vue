<template>
  <ScrollBox class="h-full">
    <div ref="editorRef" class="overflow-scroll h-full flex-grow codemirror"></div>
  </ScrollBox>
</template>

<script setup lang="ts">
import ScrollBox from "@/components/Shared/ScrollBox.vue";
import { ContentType } from "@/models/common";
import { autocompletion, closeBrackets, closeBracketsKeymap, completionKeymap } from "@codemirror/autocomplete";
import { defaultKeymap, history, historyKeymap } from "@codemirror/commands";
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
import {
  EditorView,
  ViewUpdate,
  crosshairCursor, drawSelection,
  dropCursor, highlightActiveLine, highlightActiveLineGutter,
  highlightSpecialChars, keymap, lineNumbers, rectangularSelection
} from "@codemirror/view";
import { vscodeDark } from '@uiw/codemirror-theme-vscode';
import { onBeforeUnmount, onMounted, ref, watch } from 'vue';
import { jsonExtensions } from "./json";

const props = defineProps<{
  code: string,
  lineWrap?: boolean,
  readOnly?: boolean,
  type: ContentType,
  update?: (value: string) => void
}>();

const editorRef = ref(null)
const editor = ref<EditorView | null>(null)
const lastUpdated = ref(props.code)

const lineWrappingComp = new Compartment()
const editableComp = new Compartment();

const update = (update: ViewUpdate) => {
  if (!update.docChanged) {
    return
  }
  const doc = update.state.doc.toString()
  if (doc === lastUpdated.value) return
  lastUpdated.value = doc
  props.update?.(doc)
}

const contentTypeExtension = (type: ContentType): Extension[] => {
  switch (type) {
    case ContentType.JSON:
      return jsonExtensions;
    default:
      return []
  }
}

const config = [
  EditorState.tabSize.of(2),
  EditorState.allowMultipleSelections.of(true),
  lineWrappingComp.of(EditorView.lineWrapping),
  EditorState.readOnly.of(props.readOnly),
  EditorView.updateListener.of(update),
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
  ...contentTypeExtension(props.type),
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
  if (doc === lastUpdated.value) return
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

watch(() => props.readOnly, (readOnly) => {
  editor.value?.dispatch({
    effects: editableComp.reconfigure(EditorView.editable.of(!readOnly))
  });
})

onBeforeUnmount(() => editor.value?.destroy())
</script>

<style scoped lang="scss">
:deep(.codemirror) {
  .cm-editor {
    height: 100%;
    width: 100%;

    .cm-panels {
      background-color: #18181c;
      @apply px-0.5;
      @apply pt-0.5;
      @apply text-gray-300;

      .cm-search {
        .cm-button {
          background-color: var(--nc-bg-color);
          @apply bg-none;
          @apply rounded-sm;
          @apply border-none;
          @apply text-xs;
          @apply capitalize;

          &:hover {
            color: var(--nc-primary-color);
          }
        }

        label {
          @apply capitalize;
          @apply mx-1;
        }

        button[name="close"] {
          @apply px-2;
          @apply text-center;
          @apply text-lg;

          &:hover {
            color: var(--nc-primary-color);
          }
        }
      }
    }
  }
}
</style>
