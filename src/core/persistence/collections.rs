use std::path::PathBuf;

use crate::core::collection::collection::{
    Collection, Entry, Folder, FolderId, RequestId, RequestRef,
};
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::core::persistence::Version;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncodedCollection {
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    pub version: Version,
}

pub async fn load() -> anyhow::Result<Collection> {
    let path = PathBuf::from("test");

    let data = fs::read_to_string(path.join("collection.toml")).await?;
    let collection: EncodedCollection = toml::from_str(&data)?;

    let entries = walk_entries(&path, true).await?;

    Ok(Collection::new(collection.name, entries, path))
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
            if root && entry.file_name() == "collection.toml" {
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
        description: "".to_string(),
        version: Version::V1,
    };
    let data = toml::to_string_pretty(&encoded).unwrap();
    fs::write(PathBuf::from("test/collection.toml"), data).await?;
    Ok(())
}
