<template>
  <Box class="flex">
    <NTabs size="small" animated pane-wrapper-class="flex-grow">
      <NTabPane name="body" tab="Body" class="h-full">
        <BodyViewer />
      </NTabPane>
      <NTabPane name="headers" tab="Headers">
        Hey Jude
      </NTabPane>
    </NTabs>
  </Box>
</template>

<script setup lang="ts">
import { NTabPane, NTabs } from 'naive-ui';
import BodyViewer from './BodyViewer.vue';
import Box from '../Box.vue';

</script>
