<script setup lang="ts">
import { GlobalThemeOverrides, NConfigProvider, NGlobalStyle, darkTheme } from 'naive-ui';
import MainPage from './pages/MainPage.vue';

const themeOverrides: GlobalThemeOverrides = {
  Tabs: { tabGapSmallBar: '16px' }
}
</script>

<template>
  <NConfigProvider :theme="darkTheme" class="h-full" :theme-overrides="themeOverrides">
    <NGlobalStyle />
    <MainPage />
  </NConfigProvider>
</template>
