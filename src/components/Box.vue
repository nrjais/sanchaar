<template>
  <n-el :class="classes" as="div">
    <slot></slot>
  </n-el>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { NEl } from 'naive-ui';

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

const classes = computed(() => `${props.class} ${props.width} ${props.height}`);

</script>
