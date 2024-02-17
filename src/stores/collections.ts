import { storage } from "@/backend/store";
import { Collection, CollectionRequest, EntryType } from "@/models/collection";
import { Methods } from "@/models/methods";
import { nanoid } from "nanoid";
import { defineStore } from "pinia";
import { computed, ref, watchEffect } from "vue";
import { useTabStore } from "./tabs";
import { ContentType } from "@/models/common";
import Environment from "@/models/environment";

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
  const tabStore = useTabStore();

  loadCollections().then((collections) => {
    openCollections.value = collections;

    openCollections.value = {};
    [
      <Collection>{
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
    ].forEach((collection, i) => openCollection("test" + i, collection));
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

  watchEffect(() => {
    storage.save("collections", openCollections.value);
  });

  const openRequest = (collection: string, entry: CollectionRequest) => {
    tabStore.openRequestTab(entry.name, {
      method: Methods.POST,
      address: "https://echo.nrjais.com",
      headers: [],
      params: [],
      query: [],
      body: { type: ContentType.NONE },
      environment: Environment.request(),
    });
  };

  return {
    openCollections,
    openCollectionsList,
    openRequest,
    openCollection,
    createCollection,
  };
});
