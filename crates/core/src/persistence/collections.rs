use anyhow::Context;
use directories::ProjectDirs;

use std::path::PathBuf;

use crate::collection::collection::{Collection, Entry, Folder, FolderId, RequestId, RequestRef};
use crate::collection::environment::Environments;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::persistence::Version;

const COLLECTION_ROOT_FILE: &str = "collection.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncodedCollection {
    pub name: String,
    pub version: Version,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollectionConfig {
    Path(PathBuf),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionsState {
    pub open: Vec<CollectionConfig>,
}

fn project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "nrjais", env!("CARGO_PKG_NAME"))
}

fn collections_file() -> Option<PathBuf> {
    let dirs = project_dirs()?;
    let data_dir = dirs.data_dir();
    Some(data_dir.join("collections.toml"))
}

async fn create_collections_state(collections_file: PathBuf) -> anyhow::Result<CollectionsState> {
    let dirs = project_dirs().context("Failed to find project dir during init")?;
    let data_dir = dirs.data_dir();

    let default_path = data_dir.join("Sanchaar");
    fs::create_dir_all(&default_path).await?;

    save(&Collection {
        name: "Sanchaar".to_string(),
        children: vec![],
        path: default_path.clone(),
        expanded: false,
        environments: Environments::new(),
    })
    .await?;

    let state = CollectionsState {
        open: vec![CollectionConfig::Path(default_path)],
    };

    let data = toml::to_string_pretty(&state)?;
    fs::write(collections_file, data).await?;

    Ok(state)
}

async fn open_collections_list() -> Option<Vec<CollectionConfig>> {
    let collections_file = collections_file()?;

    let data = match fs::read_to_string(&collections_file).await {
        Ok(data) => data,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                return Some(
                    create_collections_state(collections_file)
                        .await
                        .inspect_err(|e| {
                            println!("Error creating collections state: {:?}", e);
                        })
                        .ok()?
                        .open,
                );
            }
            return None;
        }
    };
    let collections: CollectionsState = toml::from_str(&data).ok()?;

    Some(collections.open)
}

pub async fn load() -> anyhow::Result<Vec<Collection>> {
    let collections = match open_collections_list().await {
        Some(collections) => collections,
        None => return Ok(vec![]),
    };

    let mut result = vec![];

    for collection in collections {
        match collection {
            CollectionConfig::Path(path) => {
                let data = fs::read_to_string(path.join(COLLECTION_ROOT_FILE)).await?;
                let collection: EncodedCollection = toml::from_str(&data)?;
                let entries = walk_entries(&path, true).await?;
                result.push(Collection::new(collection.name, entries, path));
            }
        }
    }

    Ok(result)
}

async fn walk_entries(dir_path: &PathBuf, root: bool) -> anyhow::Result<Vec<Entry>> {
    let mut entries = vec![];
    let mut dir = fs::read_dir(dir_path).await?;

    while let Some(entry) = dir.next_entry().await? {
        if entry.file_type().await?.is_dir() {
            let children = Box::pin(walk_entries(&entry.path(), false)).await?;
            entries.push(Entry::Folder(Folder {
                id: FolderId::new(),
                name: entry.file_name().to_string_lossy().to_string(),
                children,
                path: entry.path(),
                expanded: false,
            }));
        } else {
            if root && entry.file_name() == COLLECTION_ROOT_FILE {
                continue;
            }

            if !entry.file_name().to_string_lossy().ends_with(".toml") {
                continue;
            }

            entries.push(Entry::Item(RequestRef {
                name: entry
                    .file_name()
                    .to_string_lossy()
                    .trim_end_matches(".toml")
                    .to_string(),
                path: entry.path(),
                id: RequestId::new(),
            }));
        }
    }
    Ok(entries)
}

pub async fn save(collection: &Collection) -> anyhow::Result<()> {
    let encoded = EncodedCollection {
        name: collection.name.clone(),
        version: Version::V1,
    };
    let data = toml::to_string_pretty(&encoded).unwrap();

    fs::create_dir_all(&collection.path).await?;
    fs::write(collection.path.join(COLLECTION_ROOT_FILE), data).await?;

    Ok(())
}
