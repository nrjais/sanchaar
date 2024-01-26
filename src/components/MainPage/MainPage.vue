<template>
  <Box>
    <NSplit direction="horizontal" :max="0.25" :min="0.05" :default-size="0.2" class="pt-2 flex-grow">
      <template #1>
        <SidePanel></SidePanel>
      </template>
      <template #2>
        <ContentPanel></ContentPanel>
      </template>
    </NSplit>
  </Box>
</template>

<script setup lang="ts">
import { NSplit } from 'naive-ui';
import Box from '../Shared/Box.vue';
import SidePanel from './SidePanel.vue';
import ContentPanel from './ContentPanel.vue';

</script>
