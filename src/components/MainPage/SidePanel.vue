<template>
  <ScrollBox class="bg-[#1a1a1a] px-1 h-full">
    <NInput size="small" v-model:value="pattern" placeholder="Search" class="w-full" />
    <NTree :show-line="true" :expand-on-click="true" :show-irrelevant-nodes="false" :pattern="pattern" :data="tree"
      block-line class="mt-2">
      <template #empty>
        <div>
        </div>
      </template>
    </NTree>
  </ScrollBox>
</template>

<script setup lang="ts">
import ScrollBox from '@/components/Shared/ScrollBox.vue';
import { Collection, CollectionEntry } from '@/models/collection';
import { useCollectionStore } from '@/stores/collections';
import { NInput, NTree, TreeOption } from 'naive-ui';
import { computed, ref } from 'vue';

const pattern = ref("");
const collectionStore = useCollectionStore()
const collections = computed<Collection[]>(() => collectionStore.openCollectionsList)

const buildTree = (entries: CollectionEntry[]): TreeOption[] => {
  return entries.map((entry): TreeOption => {
    const children = entry.type === "folder" ? buildTree(entry.entries) : undefined;
    return {
      label: entry.name,
      key: entry.name,
      isLeaf: entry.type !== "folder",
      children
    }
  })
}

const tree = computed<TreeOption[]>(() => collections.value
  .map((collection: Collection): TreeOption => ({
    key: collection.name,
    label: collection.name,
    children: buildTree(collection.entries)
  })))
</script>
