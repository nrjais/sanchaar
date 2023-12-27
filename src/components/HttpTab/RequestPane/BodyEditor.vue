<template>
  <Box class="flex gap-2 flex-col">
    <NText strong depth="3" tag="div">Body</NText>
    <CodeMirror :code="props.body || '\n\n'" :readOnly="false" :lineWrap="true" :update="props.update" />
  </Box>
</template>

<script setup lang="ts">
import Box from '@/components/Shared/Box.vue';
import CodeMirror from '@/components/Shared/CodeMirror/CodeMirror.vue';
import { NText } from 'naive-ui';
import { defineProps } from 'vue';

const props = defineProps<{ update: (value: string) => void, body?: string }>();
</script>
