<script setup lang="ts">
import { NConfigProvider, NGlobalStyle, darkTheme } from 'naive-ui';
import Box from './components/Box.vue';
import Request from './pages/Request.vue';
</script>

<template>
  <NConfigProvider :theme="darkTheme" class="h-full">
    <NGlobalStyle />
    <Box class="p-4">
      <Request />
    </Box>
  </NConfigProvider>
</template>
