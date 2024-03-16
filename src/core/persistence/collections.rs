use crate::core::persistence::Version;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use walkdir::{DirEntry, WalkDir};

use crate::state::collection::{Collection, Entry, Folder, Item};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncodedCollection {
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    pub version: Version,
}

pub async fn load() -> anyhow::Result<Collection> {
    let data = fs::read_to_string(PathBuf::from("test/collection.toml")).await?;
    let collection: EncodedCollection = toml::from_str(&data)?;

    let filter = |e: &DirEntry| {
        e.file_name()
            .to_str()
            .map(|s| s == "collection.toml")
            .unwrap_or(false)
    };

    let entries = walk_entries(filter)?;
    Ok(Collection::new(collection.name, entries))
}

fn walk_entries(filter: fn(&DirEntry) -> bool) -> anyhow::Result<Vec<Entry>> {
    let mut entries = vec![];
    let walker = WalkDir::new("test/").into_iter().filter_entry(filter);
    for entry in walker {
        let entry = entry?;
        let path = entry.path();
        let name = path.file_name().unwrap().to_str().unwrap().to_string();

        if path.is_dir() {
            entries.push(Entry::Folder(Folder {
                name,
                children: walk_entries(|_| false)?,
                expanded: false,
            }));
        } else {
            entries.push(Entry::Item(Item { name }));
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
