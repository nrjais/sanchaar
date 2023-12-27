<template>
  <Box class="flex">
    <NEmpty :description="description" size="huge" class="m-auto" :theme-overrides="themOverides">
      <template #icon>
        <NSpin :size="96" />
      </template>
      <template #extra>
        <NButton v-if="sending" size="large" tertiary type="error" @click="cancel">
          Cancel
        </NButton>
      </template>
    </NEmpty>
  </Box>
</template>

<script setup lang="ts">
import Box from '@/components/Shared/Box.vue';
import { EmptyProps, NButton, NEmpty, NSpin } from 'naive-ui';
import { computed, ref } from 'vue';

const themOverides: NonNullable<EmptyProps['themeOverrides']> = {
  iconSizeHuge: 'fit-content',
  fontSizeHuge: '1.5rem',
}

const props = defineProps<{ cancel: () => void }>();

const sending = ref(true);

const description = computed(() => {
  if (sending.value) {
    return 'Sending Request...';
  }
  return 'Cancelling Request...';
});

const cancel = () => {
  sending.value = false;
  props.cancel?.();
}
</script>
