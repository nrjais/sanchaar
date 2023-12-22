<template>
  <NButtonGroup class="my-1" size="small">
    <NButton secondary @click="viewMode = 'pretty'" :type="buttonType(viewMode, 'pretty')">Pretty</NButton>
    <NButton secondary @click="viewMode = 'raw'" :type="buttonType(viewMode, 'raw')">Raw</NButton>
    <NButton secondary :on-click="toggleWrapping" :type="buttonType(options.lineWrapping)">
      <NIcon>
        <IconTextWrap />
      </NIcon>
    </NButton>
  </NButtonGroup>
  <Box class="my-2">
    <Codemirror :value="code" :options="options" placeholder="test placeholder" />
  </Box>
</template>

<script setup lang="ts">
import { IconTextWrap } from "@tabler/icons-vue";
import { NButton, NButtonGroup, NIcon } from 'naive-ui';
import { Ref, computed, ref } from 'vue';
// @ts-ignore
import Codemirror from "codemirror-editor-vue3";
import { EditorConfiguration } from "codemirror"
import Box from "../Box.vue";

type ViewMode = "pretty" | "raw"

const viewMode = ref<ViewMode>('pretty')

type Props = {
  code: string;
}
const props = withDefaults(defineProps<Props>(), {
  code: "",
});

const options: Ref<EditorConfiguration> = ref({
  mode: 'application/ld+json',
  theme: "dracula",
  tabSize: 2,
  lineNumbers: true,
  lineWrapping: false,
  readOnly: true,
  autoCloseBrackets: true,
  matchBrackets: true,
  showCursorWhenSelecting: true,
  foldGutter: true,
  gutters: ['CodeMirror-linenumbers', 'CodeMirror-foldgutter'],
})

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
  options.value.lineWrapping = !options.value.lineWrapping
}
</script>
