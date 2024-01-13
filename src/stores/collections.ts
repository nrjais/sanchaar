import { defineStore } from "pinia";
import { computed, ref } from "vue";

const nextCollectionId = ((start: number) => {
  return () => `col-${start++}`;
})(0);

interface CollectionEntry {
  id: string;
  position: number;
}

export const useTabStore = defineStore("CollectoinStore", () => {
  const openCollections = ref(new Map<string, CollectionEntry>());

  const openCollectionsList = computed(() => {
    const tabs = [...openCollections.value.values()];
    return tabs.sort((a, b) => a.position - b.position);
  });

  const openNewCollection = () => {
    const id = nextCollectionId();
  };

  return {
    openCollections,
    openCollectionsList,
    openNewCollection,
  };
});
