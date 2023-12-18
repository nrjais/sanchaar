<script setup lang="ts">
import Request from './pages/Request.vue';
import { darkTheme } from 'naive-ui'

</script>

<template>
  <div class="h-full">
    <NConfigProvider :theme="darkTheme">
      <div class="p-4">
        <Request />
      </div>
    </NConfigProvider>
  </div>
</template>

<style scoped>
</style>
