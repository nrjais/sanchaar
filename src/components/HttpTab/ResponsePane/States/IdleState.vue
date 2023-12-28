<template>
  <Box class="flex">
    <NEmpty description="Send request to see response" size="huge" class="m-auto" :theme-overrides="themOverides">
      <template #icon>
        <NIcon color="grey" size="8rem">
          <IconSend />
        </NIcon>
      </template>
    </NEmpty>
  </Box>
</template>

<script setup lang="ts">
import Box from '@/components/Shared/Box.vue';
import { IconSend } from '@tabler/icons-vue';
import { EmptyProps, NEmpty, NIcon } from 'naive-ui';

const themOverides: NonNullable<EmptyProps['themeOverrides']> = {
  iconSizeHuge: 'fit-content',
  fontSizeHuge: '1.2rem',
}
</script>
