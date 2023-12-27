<template>
  <IdleState v-if="result.state === 'idle'" />
  <ExecutingState v-if="result.state === 'running'" :cancel="result.abort" />
  <CancelledState v-if="result.state === 'cancelled'" />
  <ErrorState v-if="result.state === 'error'" :error="result.error"/>
  <CompletedState v-if="result.state === 'completed'" :response="result.response" />
</template>

<script setup lang="ts">
import { useRequestStore } from '@/stores/requests';
import { computed, defineProps } from 'vue';
import CancelledState from './CancelledState.vue';
import CompletedState from './CompletedState.vue';
import ExecutingState from './ExecutingState.vue';
import IdleState from './IdleState.vue';
import ErrorState from './ErrorState.vue';

const props = defineProps<{ tabId: string }>();

const requestStore = useRequestStore();

const result = computed(() => requestStore.getExecutionResult(props.tabId));

</script>
