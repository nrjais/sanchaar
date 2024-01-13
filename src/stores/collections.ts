import { Collection } from "@/models/collection";
import { defineStore } from "pinia";
import { computed, ref } from "vue";
import { nanoid } from "nanoid";

interface OpenCollection {
  id: string;
  position: number;
  collection: Collection;
}

export const useTabStore = defineStore("CollectoinStore", () => {
  const openCollections = ref(new Map<string, OpenCollection>());

  const openCollectionsList = computed(() => {
    const tabs = [...openCollections.value.values()];
    return tabs.sort((a, b) => a.position - b.position);
  });

  const openNewCollection = () => {
    const id = nanoid();

  };

  return {
    openCollections,
    openCollectionsList,
    openNewCollection,
  };
});
