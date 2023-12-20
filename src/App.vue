<script setup lang="ts">
import { NConfigProvider, NGlobalStyle, darkTheme } from 'naive-ui';
import MainPage from './pages/MainPage.vue';
</script>

<template>
  <NConfigProvider :theme="darkTheme" class="h-full">
    <NGlobalStyle />
    <MainPage />
  </NConfigProvider>
</template>
