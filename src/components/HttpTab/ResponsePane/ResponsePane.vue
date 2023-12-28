<template>
  <IdleState v-if="result.state === 'idle'" />
  <ExecutingState v-if="result.state === 'running'" :cancel="result.abort" />
  <CancelledState v-if="result.state === 'cancelled'" />
  <ErrorState v-if="result.state === 'error'" :error="result.error" />
  <CompletedState v-if="result.state === 'completed'" :response="result.response" />
</template>

<script setup lang="ts">
import { useRequestStore } from '@/stores/requests';
import { computed } from 'vue';
import CancelledState from './States/CancelledState.vue';
import CompletedState from './States/CompletedState.vue';
import ExecutingState from './States/ExecutingState.vue';
import IdleState from './States/IdleState.vue';
import ErrorState from './States/ErrorState.vue';

const props = defineProps<{ tabId: string }>();

const requestStore = useRequestStore();

const result = computed(() => requestStore.getExecutionResult(props.tabId));

</script>
