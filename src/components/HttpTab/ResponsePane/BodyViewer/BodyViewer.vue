<template>
  <Box class="flex flex-col gap-2">
    <Box class="flex" height="h-fit gap-2">
      <NButtonGroup class="pb-1" size="small">
        <NButton secondary @click="viewMode = 'pretty'" :type="buttonType(viewMode, 'pretty')">Pretty</NButton>
        <NButton secondary @click="viewMode = 'raw'" :type="buttonType(viewMode, 'raw')">Raw</NButton>
      </NButtonGroup>
      <NButtonGroup class="pb-1" size="small">
        <NButton secondary @click="toggleWrapping" :type="buttonType(lineWrap)">
          <NIcon>
            <IconTextWrap />
          </NIcon>
        </NButton>
        <NButton secondary @click="copyToClipboard">
          <NIcon>
            <IconCopy />
          </NIcon>
        </NButton>
      </NButtonGroup>
    </Box>
    <CodeMirror :code="code" :lineWrap="lineWrap" :readOnly="true" />
  </Box>
</template>

<script setup lang="ts">
import Box from '@/components/Shared/Box.vue';
import CodeMirror from '@/components/Shared/CodeMirror/CodeMirror.vue';
import { ContentType } from "@/models/common";
import { ResponseBody } from "@/models/response";
import { IconCopy, IconTextWrap } from "@tabler/icons-vue";
import { NButton, NButtonGroup, NIcon } from 'naive-ui';
import { computed, ref } from 'vue';

type ViewMode = "pretty" | "raw"

const viewMode = ref<ViewMode>('pretty')
const lineWrap = ref(true);

const props = defineProps<{ body: ResponseBody }>();

const codeIn = computed(() => {
  if (props.body.type == ContentType.JSON) {
    return props.body.data
  }
  return ""
})

const prettyCode = computed(() => JSON.stringify(JSON.parse(codeIn.value), null, 2))
const code = computed(() => viewMode.value == 'pretty' ? prettyCode.value : codeIn.value)
const toggleWrapping = () => lineWrap.value = !lineWrap.value
const copyToClipboard = () => navigator.clipboard.writeText(code.value)

const buttonType = <T>(val: T, expected?: T) => {
  return (val == expected) || (!expected && val) ? 'primary' : 'default'
}

</script>
