<script setup lang="ts">
import { NConfigProvider, NGlobalStyle, darkTheme } from 'naive-ui';
import Box from './components/Box.vue';
import RequestPage from './pages/RequestPage.vue';
</script>

<template>
  <NConfigProvider :theme="darkTheme" class="h-full">
    <NGlobalStyle />
    <Box class="p-4">
      <RequestPage />
    </Box>
  </NConfigProvider>
</template>
