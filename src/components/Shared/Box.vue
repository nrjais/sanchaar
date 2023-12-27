<template>
  <div :class="classes" as="div">
    <slot></slot>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

export interface Props {
  class: string
  width: string
  height: string
}

const props = withDefaults(defineProps<Props>(), {
  class: '',
  width: 'w-full',
  height: 'h-full'
})

const classes = computed(() => `${props.width} ${props.height} ${props.class}`);
</script>
