import { RequestConfig } from "./request";

export interface Collection {
  name: string;
  description: string;
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
}

export interface CollectionRequest {
  type: EntryType.Request;
  name: string;
  config: RequestConfig;
}
