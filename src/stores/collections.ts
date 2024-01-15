import { Collection, EntryType } from "@/models/collection";
import { ContentType } from "@/models/common";
import Environment from "@/models/environment";
import { Methods } from "@/models/methods";
import { RequestConfig } from "@/models/request";
import { nanoid } from "nanoid";
import { defineStore } from "pinia";
import { computed, ref } from "vue";

interface OpenCollection {
  id: string;
  position: number;
  collection: Collection;
}

export const useCollectionStore = defineStore("CollectionStore", () => {
  const openCollections = ref<ObjectMap<OpenCollection>>({});

  const openCollectionsList = computed<Collection[]>((): Collection[] => {
    const tabs = Object.values(openCollections.value);
    const open = tabs
      .sort((a, b) => a.position - b.position)
      .map((o) => o.collection);

    return open;
  });

  const openCollection = (collection: Collection): void => {
    const id = nanoid();
    const position = Object.keys(openCollections.value).length;
    openCollections.value[id] = { id, position, collection };
  };

  const createCollection = (name: string): void => {
    const collection: Collection = {
      name,
      description: "",
      entries: [],
    };
    openCollection(collection);
  };

  const req: RequestConfig = {
    method: Methods.GET,
    address: "https://jsonplaceholder.typicode.com/todos/1",
    params: [],
    environment: new Environment("request"),
    query: [],
    headers: [
      {
        key: "Content-Type",
        value: "application/json",
      },
    ],
    body: {
      type: ContentType.JSON,
      data: `{name: "Test"}`,
    },
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
              config: req,
            },
          ],
        },
        {
          type: EntryType.Request,
          name: "Req 2",
          config: req,
        },
      ],
    },
  ];

  collections.forEach((collection) => openCollection(collection));

  return {
    openCollections,
    openCollectionsList,
    openCollection,
    createCollection,
  };
});
