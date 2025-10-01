use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::http::KeyValList;
use crate::http::environment::{Environment, Environments};

use super::ENVIRONMENTS;
use super::{EncodedKeyValue, TOML_EXTENSION, Version};

#[derive(Debug, Deserialize, Serialize)]
pub struct EncodedEnvironment {
    pub name: String,
    pub version: Version,
    pub variables: Vec<EncodedKeyValue>,
}

impl From<EncodedEnvironment> for Environment {
    fn from(val: EncodedEnvironment) -> Self {
        Environment {
            name: val.name,
            variables: KeyValList::from(val.variables.into_iter().map(Into::into).collect()).into(),
        }
    }
}

impl From<Environment> for EncodedEnvironment {
    fn from(environment: Environment) -> Self {
        EncodedEnvironment {
            name: environment.name,
            version: Version::V1,
            variables: KeyValList::clone(&environment.variables)
                .into_iter()
                .filter(|kv| !kv.name.is_empty())
                .map(Into::into)
                .collect(),
        }
    }
}

pub async fn read_environments(col: &Path) -> anyhow::Result<Environments> {
    let env_path = col.join(ENVIRONMENTS);
    let exists = fs::try_exists(&env_path).await?;
    if !exists {
        return Ok(Environments::new());
    }

    let mut files = fs::read_dir(env_path).await?;

    let mut environments = Environments::new();

    while let Some(file) = files.next_entry().await? {
        if !file.file_type().await?.is_file() {
            continue;
        }

        let name = file.file_name();
        let name = name.to_string_lossy();

        let without_ext = name.trim_end_matches(&TOML_EXTENSION);
        if !name.ends_with(&TOML_EXTENSION) || without_ext.is_empty() {
            continue;
        }

        let content = fs::read_to_string(&file.path()).await?;

        let environment: EncodedEnvironment = toml::from_str(&content)?;

        environments.insert(environment.into());
    }

    Ok(environments)
}

pub fn encode_environments(environment: &Environments) -> Vec<EncodedEnvironment> {
    environment
        .entries()
        .map(|(_, env)| EncodedEnvironment::from(env.clone()))
        .collect()
}

pub async fn save_environments(
    path: PathBuf,
    environments: Vec<EncodedEnvironment>,
) -> anyhow::Result<()> {
    let env_path = path.join(ENVIRONMENTS);

    fs::create_dir_all(&env_path).await?;

    for environment in environments.iter() {
        let path = env_path.join(format!("{}{}", &environment.name, TOML_EXTENSION));
        let content = toml::to_string_pretty(environment)?;

        fs::write(&path, content).await?;
    }

    Ok(())
}
