<template>
  <Box class="flex">
    <NEmpty :description="props.error.message" size="huge" class="m-auto" :themeOverrides="themOverides">
      <template #icon>
        <NIcon color="grey" size="8rem">
          <IconLocationBroken />
        </NIcon>
      </template>
    </NEmpty>
  </Box>
</template>
<script setup lang="ts">
import Box from '@/components/Shared/Box.vue';
import { IconLocationBroken } from '@tabler/icons-vue';
import { EmptyProps, NEmpty, NIcon } from 'naive-ui';
import { defineProps } from 'vue';

const themOverides: NonNullable<EmptyProps['themeOverrides']> = {
  iconSizeHuge: 'fit-content',
  fontSizeHuge: '1.5rem',
}
const props = defineProps<{ error: Error }>();
</script>
