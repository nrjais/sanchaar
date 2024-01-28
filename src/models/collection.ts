export interface Collection {
  name: string;
  description?: string;
  entries: CollectionEntry[];
}

export type CollectionEntry = CollectionFolder | CollectionRequest;

export enum EntryType {
  Folder = "folder",
  Request = "request",
}

export interface CollectionFolder {
  type: EntryType.Folder;
  name: string;
  entries: CollectionEntry[];
  path: string;
}

export interface CollectionRequest {
  type: EntryType.Request;
  name: string;
  path: string;
}
