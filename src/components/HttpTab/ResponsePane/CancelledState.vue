<template>
  <Box class="flex">
    <NEmpty description="Request cancelled" size="huge" class="m-auto" :theme-overrides="themOverides">
      <template #icon>
        <NIcon color="grey" size="8rem">
          <IconSendOff />
        </NIcon>
      </template>
    </NEmpty>
  </Box>
</template>

<script setup lang="ts">
import Box from '@/components/Shared/Box.vue';
import { IconSendOff } from '@tabler/icons-vue';
import { EmptyProps, NEmpty, NIcon } from 'naive-ui';

const themOverides: NonNullable<EmptyProps['themeOverrides']> = {
  iconSizeHuge: 'fit-content',
  fontSizeHuge: '1.5rem',
}
</script>
