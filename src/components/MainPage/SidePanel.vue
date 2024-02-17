<template>
  <ScrollBox class="bg-[#1a1a1a] px-1 h-full gap-1 flex flex-col py-2">
    <NInputGroup>
      <NInput size="small" v-model:value="pattern" placeholder="Search" class="w-full" />
      <NDropdown :options="menuOptions" trigger="click" placement="bottom-start">
        <NButton size="small" tertiary type="primary">
          <NIcon>
            <IconDotsVertical />
          </NIcon>
        </NButton>
      </NDropdown>
    </NInputGroup>
    <NTree :show-line="true" :expand-on-click="true" :show-irrelevant-nodes="false" :pattern="pattern" :data="tree"
      block-line :node-props="nodeProps">
      <template #empty>
        <div>
        </div>
      </template>
    </NTree>
  </ScrollBox>
</template>

<script setup lang="ts">
import ScrollBox from '@/components/Shared/ScrollBox.vue';
import { Collection, CollectionEntry, EntryType } from '@/models/collection';
import { useCollectionStore } from '@/stores/collections';
import { IconDotsVertical } from '@tabler/icons-vue';
import { NButton, NDropdown, NIcon, NInput, NInputGroup, NTree, TreeOption } from 'naive-ui';
import { computed, ref } from 'vue';

const pattern = ref("");
const collectionStore = useCollectionStore()
const collections = computed<Collection[]>(() => collectionStore.openCollectionsList);

const nodeProps = ({ option }: { option: TreeOption }) => {
  return {
    onClick() {
      const data = option.data as any as { entry: CollectionEntry, name: string };
      if (!option.isLeaf) {
        return
      }
      if (data.entry.type === EntryType.Request) {
        collectionStore.openRequest(data.name, data.entry)
      }
    }
  }
}

const menuOptions = [
  { label: "New Collection", key: "new-collection" },
]

const buildTree = (name: string, entries: CollectionEntry[]): TreeOption[] => {
  return entries.map((entry): TreeOption => {
    const children = entry.type === "folder" ? buildTree(name, entry.entries) : undefined;
    return {
      label: entry.name,
      key: entry.name,
      isLeaf: entry.type !== "folder",
      children,
      data: {
        collection: name,
        entry
      },
    }
  })
}

const tree = computed<TreeOption[]>(() => collections.value
  .map((collection: Collection): TreeOption => ({
    key: collection.name,
    label: collection.name,
    children: buildTree(collection.name, collection.entries)
  })))
</script>
