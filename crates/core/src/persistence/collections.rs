use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::http::collection::{Collection, Entry, Folder, FolderId, RequestId, RequestRef, Script};
use crate::http::{KeyValList, KeyValue};
use crate::persistence::Version;

use super::environment::read_environments;
use super::{
    decode_key_values, encode_key_values, EncodedKeyValue, COLLECTION_ROOT_FILE, HCL_EXTENSION,
    JS_EXTENSION, REQUESTS, SCRIPTS, TS_EXTENSION,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct EncodedCollection {
    pub name: String,
    pub version: Version,
    pub default_environment: Option<String>,
    #[serde(default)]
    pub headers: Vec<EncodedKeyValue>,
    #[serde(default)]
    pub variables: Vec<EncodedKeyValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CollectionConfig {
    Path(PathBuf),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionsState {
    pub open: Vec<CollectionConfig>,
}

fn project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "nrjais", "sanchaar")
}

fn collections_file() -> Option<PathBuf> {
    let dirs = project_dirs()?;
    let data_dir = dirs.data_dir();
    Some(data_dir.join("collections.hcl"))
}

async fn create_collections_state(collections_file: PathBuf) -> Result<CollectionsState> {
    let dirs = project_dirs().context("Failed to find project dir during init")?;
    let data_dir = dirs.data_dir();

    let default_path = data_dir.join("Sanchaar");
    fs::create_dir_all(&default_path).await?;

    save_collection(
        default_path.clone(),
        EncodedCollection {
            name: "Sanchaar".to_string(),
            version: Version::V1,
            default_environment: None,
            headers: vec![],
            variables: vec![],
        },
    )
    .await?;

    let state = CollectionsState {
        open: vec![CollectionConfig::Path(default_path)],
    };

    let data = hcl::to_string(&state)?;
    fs::write(collections_file, data).await?;

    Ok(state)
}

async fn open_collections_list() -> Option<Vec<CollectionConfig>> {
    let collections_path = collections_file()?;
    let data = match fs::read_to_string(&collections_path).await {
        Ok(data) => data,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                return Some(
                    create_collections_state(collections_path)
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
    let collections: CollectionsState = hcl::from_str(&data).ok()?;

    Some(collections.open)
}

pub async fn load() -> Result<Vec<Collection>> {
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

pub async fn save(collection: Vec<Collection>) -> Result<()> {
    let collections_file = collections_file().context("Failed to find collections file")?;
    let state = CollectionsState {
        open: collection
            .into_iter()
            .map(|col| CollectionConfig::Path(col.path.clone()))
            .collect(),
    };

    let data = hcl::to_string(&state)?;
    fs::write(collections_file, data).await?;

    Ok(())
}

pub async fn open_collection(path: PathBuf) -> Result<Collection, anyhow::Error> {
    let data = fs::read_to_string(path.join(COLLECTION_ROOT_FILE)).await?;

    let collection: EncodedCollection = hcl::from_str(&data)?;
    let environments = read_environments(&path).await?;
    let entries = find_all_requests(&path).await?;
    let scripts = find_all_scripts(&path).await?;
    let dotenv = read_dotenv(&path);

    let default_env = collection
        .default_environment
        .as_deref()
        .and_then(|n| environments.find_by_name(n));

    Ok(Collection::new(
        collection.name,
        entries,
        scripts,
        path,
        environments,
        default_env,
        decode_key_values(collection.headers).into(),
        decode_key_values(collection.variables).into(),
        dotenv.into(),
    ))
}

fn read_dotenv(path: &PathBuf) -> KeyValList {
    let Ok(vars) = dotenvy::from_filename_iter(path.join(".env")) else {
        return KeyValList::new();
    };

    let vars = vars
        .filter_map(|r| r.ok())
        .map(|(k, v)| KeyValue {
            name: k,
            value: v,
            disabled: false,
        })
        .collect();

    KeyValList::from(vars)
}

pub async fn find_all_scripts(col: &Path) -> Result<Vec<Script>> {
    let path = col.join(SCRIPTS);
    let exists = fs::try_exists(&path).await?;
    if !exists {
        return Ok(Vec::new());
    }

    let mut files = fs::read_dir(path).await?;

    let mut scripts = Vec::new();

    while let Some(file) = files.next_entry().await? {
        if !file.file_type().await?.is_file() {
            continue;
        }

        let path = file.path();

        let ext = path.extension().and_then(|ext| ext.to_str());
        if ext != Some(JS_EXTENSION) && ext != Some(TS_EXTENSION) {
            continue;
        }

        match path.file_name() {
            Some(name) => scripts.push(Script {
                name: name.to_string_lossy().to_string(),
                path,
            }),
            None => continue,
        }
    }

    Ok(scripts)
}

async fn find_all_requests(path: &Path) -> Result<Vec<Entry>> {
    let requests = path.join(REQUESTS);
    let exists = fs::try_exists(&requests).await?;
    if !exists {
        return Ok(Vec::new());
    }

    walk_entries(&requests).await
}

async fn walk_entries(dir_path: &Path) -> Result<Vec<Entry>> {
    let mut all_entries = vec![];
    let mut dir = fs::read_dir(dir_path).await?;

    while let Some(entry) = dir.next_entry().await? {
        if entry.file_type().await?.is_dir() {
            let entries = Box::pin(walk_entries(&entry.path())).await?;
            all_entries.push(Entry::Folder(Folder {
                id: FolderId::new(),
                name: entry.file_name().to_string_lossy().to_string(),
                entries,
                path: entry.path(),
                expanded: false,
            }));
        } else {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            let without_ext = name.trim_end_matches(&HCL_EXTENSION);

            if !name.ends_with(&HCL_EXTENSION) || without_ext.is_empty() {
                continue;
            }

            all_entries.push(Entry::Item(RequestRef {
                name: without_ext.to_string(),
                path: entry.path(),
                id: RequestId::new(),
            }));
        }
    }
    Ok(all_entries)
}

pub fn encode_collection(collection: &Collection) -> EncodedCollection {
    EncodedCollection {
        name: collection.name.clone(),
        version: Version::V1,
        default_environment: collection
            .default_env
            .and_then(|env| collection.environments.get(env))
            .map(|env| env.name.clone()),
        headers: encode_key_values(KeyValList::clone(&collection.headers)),
        variables: encode_key_values(KeyValList::clone(&collection.variables)),
    }
}

pub async fn save_collection(path: PathBuf, collection: EncodedCollection) -> Result<()> {
    let data = hcl::to_string(&collection).expect("Failed to encode collection");

    fs::create_dir_all(&path).await?;
    fs::write(path.join(COLLECTION_ROOT_FILE), data).await?;

    Ok(())
}

pub async fn save_script(path: PathBuf, name: &str, content: &str) -> Result<()> {
    let path = path.join(SCRIPTS).join(name);
    fs::create_dir_all(&path).await?;
    fs::write(path, content).await?;
    Ok(())
}
