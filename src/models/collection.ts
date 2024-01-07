import { RequestConfig } from "./request";

export interface Collection {
  name: string;
  description: string;
  entries: CollectionEntry[];
}

type CollectionEntry = CollectionFolder | CollectionRequest;

export interface CollectionFolder {
  name: string;
  entries: CollectionEntry[];
}

export interface CollectionRequest {
  name: string;
  config: RequestConfig;
}
