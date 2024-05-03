use std::path::PathBuf;

use anyhow::Context;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::http::collection::{Collection, Entry, Folder, FolderId, RequestId, RequestRef};
use crate::persistence::Version;

use super::environment::read_environments;
use super::{COLLECTION_ROOT_FILE, REQUESTS, TOML_EXTENSION};

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

    save_collection(
        default_path.clone(),
        EncodedCollection {
            name: "Sanchaar".to_string(),
            version: Version::V1,
        },
    )
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
                let col = open_collection(path).await;
                match col {
                    Ok(col) => result.push(col),
                    Err(e) => {
                        println!("Error opening collection: {:?}", e);
                    }
                }
            }
        }
    }

    Ok(result)
}

pub async fn save(collection: Vec<Collection>) -> anyhow::Result<()> {
    let collections_file = collections_file().context("Failed to find collections file")?;
    let state = CollectionsState {
        open: collection
            .into_iter()
            .map(|col| CollectionConfig::Path(col.path.clone()))
            .collect(),
    };

    let data = toml::to_string_pretty(&state)?;
    fs::write(collections_file, data).await?;

    Ok(())
}

pub async fn open_collection(path: PathBuf) -> Result<Collection, anyhow::Error> {
    let data = fs::read_to_string(path.join(COLLECTION_ROOT_FILE)).await?;

    let collection: EncodedCollection = toml::from_str(&data)?;
    let environments = read_environments(&path).await?;
    let entries = find_all_requests(&path).await?;

    Ok(Collection::new(
        collection.name,
        entries,
        path,
        environments,
    ))
}

async fn find_all_requests(path: &PathBuf) -> anyhow::Result<Vec<Entry>> {
    let requests = path.join(REQUESTS);
    let exists = fs::try_exists(&requests).await?;
    if !exists {
        return Ok(Vec::new());
    }

    walk_entries(&requests).await
}

async fn walk_entries(dir_path: &PathBuf) -> anyhow::Result<Vec<Entry>> {
    let mut entries = vec![];
    let mut dir = fs::read_dir(dir_path).await?;

    while let Some(entry) = dir.next_entry().await? {
        if entry.file_type().await?.is_dir() {
            let children = Box::pin(walk_entries(&entry.path())).await?;
            entries.push(Entry::Folder(Folder {
                id: FolderId::new(),
                name: entry.file_name().to_string_lossy().to_string(),
                children,
                path: entry.path(),
                expanded: false,
            }));
        } else {
            let name = entry.file_name();
            let name = name.to_string_lossy();

            if !name.ends_with(TOML_EXTENSION) || name.trim_end_matches(TOML_EXTENSION).is_empty() {
                continue;
            }

            entries.push(Entry::Item(RequestRef {
                name: name.trim_end_matches(".toml").to_string(),
                path: entry.path(),
                id: RequestId::new(),
            }));
        }
    }
    Ok(entries)
}

pub fn encode_collection(collection: &Collection) -> EncodedCollection {
    EncodedCollection {
        name: collection.name.clone(),
        version: Version::V1,
    }
}

pub async fn save_collection(path: PathBuf, collection: EncodedCollection) -> anyhow::Result<()> {
    let data = toml::to_string_pretty(&collection).unwrap();

    fs::create_dir_all(&path).await?;
    fs::write(path.join(COLLECTION_ROOT_FILE), data).await?;

    Ok(())
}
