import { storage } from "@/backend/store";
import { Collection, EntryType } from "@/models/collection";
import { nanoid } from "nanoid";
import { defineStore } from "pinia";
import { computed, ref, watchEffect } from "vue";

interface OpenCollection {
  id: string;
  position: number;
  path: string;
  collection: Collection;
}

const loadCollections = async (): Promise<ObjectMap<OpenCollection>> => {
  return (await storage.load<ObjectMap<OpenCollection>>("collections")) || {};
};

export const useCollectionStore = defineStore("CollectionStore", () => {
  const openCollections = ref<ObjectMap<OpenCollection>>({});

  loadCollections().then((collections) => {
    openCollections.value = collections;
  });

  const openCollectionsList = computed<Collection[]>((): Collection[] => {
    const tabs = Object.values(openCollections.value);
    const open = tabs
      .sort((a, b) => a.position - b.position)
      .map((o) => o.collection);

    return open;
  });

  const openCollection = (path: string, collection: Collection): void => {
    const id = nanoid();
    const position = Object.keys(openCollections.value).length;
    openCollections.value[id] = { id, position, collection, path };
  };

  const createCollection = (name: string): void => {
    const collection: Collection = {
      name,
      entries: [],
    };
    openCollection("", collection);
  };

  const collections: Collection[] = [
    {
      name: "Test Collection",
      description: "Test Description",
      entries: [
        {
          type: EntryType.Folder,
          name: "Folder",
          entries: [
            {
              type: EntryType.Request,
              name: "Req",
            },
          ],
        },
        {
          type: EntryType.Request,
          name: "Req 2",
        },
      ],
    },
  ];

  collections.forEach((collection, i) =>
    openCollection("test" + i, collection)
  );

  watchEffect(() => {
    storage.save("collections", openCollections.value);
  });

  return {
    openCollections,
    openCollectionsList,
    openCollection,
    createCollection,
  };
});
