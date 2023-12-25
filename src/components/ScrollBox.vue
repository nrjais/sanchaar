<template>
  <div class="overflow-auto max-h-full max-w-full" :class="props.class">
    <slot></slot>
  </div>
</template>

<script setup lang="ts">

export interface Props {
  class: string
}

const props = withDefaults(defineProps<Props>(), {
  class: '',
})
</script>
