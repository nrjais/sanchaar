<template>
  <ScrollBox class="bg-[#171717] px-1">
    <NInput size="small" v-model:value="pattern" placeholder="Search" class="w-full" />
    <NTree :show-line="true" :expand-on-click="true" :show-irrelevant-nodes="false" :pattern="pattern" :data="tree"
      block-line />
  </ScrollBox>
</template>

<script setup lang="ts">
import { Collection, CollectionEntry, EntryType } from '@/models/collection';
import { ContentType } from '@/models/common';
import Environment from '@/models/environment';
import { Methods } from '@/models/methods';
import { RequestConfig } from '@/models/request';
import { NInput, NTree, TreeOption } from 'naive-ui';
import { ref } from 'vue';
import ScrollBox from '@/components/Shared/ScrollBox.vue';

const pattern = ref("");

const req: RequestConfig = {
  method: Methods.GET,
  address: "https://jsonplaceholder.typicode.com/todos/1",
  params: [],
  environment: new Environment("request"),
  query: [],
  headers: [
    {
      key: "Content-Type",
      value: "application/json"
    }
  ],
  body: {
    type: ContentType.JSON,
    data: `{name: "Test"}`
  }
}

const collections: Collection[] = [{
  name: 'Test Collection',
  description: 'Test Description',
  entries: [
    {
      type: EntryType.Folder,
      name: "Folder",
      entries: [
        {
          type: EntryType.Request,
          name: "Req",
          config: req
        }
      ]
    },
    {
      type: EntryType.Request,
      name: "Req 2",
      config: req
    }
  ],
}]

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

const tree: TreeOption[] = collections
  .map((collection): TreeOption => ({
    key: collection.name,
    label: collection.name,
    children: buildTree(collection.entries)
  }))
</script>
