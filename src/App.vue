<script setup lang="ts">
import Box from './components/Box.vue';
import Request from './pages/Request.vue';
import { darkTheme, NConfigProvider } from 'naive-ui'

</script>

<template>
  <Box class="bg-white dark:bg-neutral-900">
    <NConfigProvider :theme="darkTheme" class="h-full">
      <Box class="p-4">
        <Request />
      </Box>
    </NConfigProvider>
  </Box>
</template>

<style scoped>
</style>
