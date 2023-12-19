<template>
  <div :class="classes">
    <slot></slot>
  </div>
</template>

<script setup lang="ts">
import { computed, defineProps } from 'vue';

const props = defineProps({
  class: {
    type: String,
    default: ''
  },
  width: {
    type: String,
    default: 'w-full'
  },
  height: {
    type: String,
    default: 'h-full'
  }
})

const classes = computed(() => `${props.class} ${props.width} ${props.height}`);

</script>
