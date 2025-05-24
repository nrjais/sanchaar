use anyhow::{Context, Result};
use directories::ProjectDirs;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

use crate::http::collection::{Collection, Entry, Folder, FolderId, RequestId, RequestRef, Script};
use crate::http::environment::{Environment, EnvironmentKey};
use crate::http::request::Request;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionRecord {
    pub id: String,
    pub name: String,
    pub data: String, // JSON serialized Collection data
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestRecord {
    pub id: String,
    pub name: String,
    pub collection_id: String,
    pub folder_id: Option<String>,
    pub data: String, // JSON serialized Request data
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentRecord {
    pub id: String,
    pub name: String,
    pub collection_id: String,
    pub data: String, // JSON serialized Environment data
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptRecord {
    pub id: String,
    pub name: String,
    pub collection_id: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderRecord {
    pub id: String,
    pub name: String,
    pub collection_id: String,
    pub parent_id: Option<String>,
    pub data: String, // JSON for additional folder metadata
}

pub struct DatabaseManager {
    data_dir: PathBuf,
}

impl DatabaseManager {
    pub fn new() -> Result<Self> {
        let data_dir = Self::get_data_dir()?;
        std::fs::create_dir_all(&data_dir)?;

        Ok(Self { data_dir })
    }

    fn get_data_dir() -> Result<PathBuf> {
        let dirs = ProjectDirs::from("com", "nrjais", "sanchaar")
            .context("Failed to find project directory")?;
        Ok(dirs.data_dir().join("databases"))
    }

    pub fn scan_and_load_collections(&mut self) -> Result<Vec<Collection>> {
        let mut collections = Vec::new();
        let dir = std::fs::read_dir(&self.data_dir)?;

        for entry in dir {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("db") {
                    let collection_id = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or_default()
                        .to_string();

                    if let Ok(collection) = self.load_collection(&collection_id) {
                        collections.push(collection);
                    }
                }
            }
        }

        Ok(collections)
    }

    pub fn create_collection(&mut self, name: String) -> Result<String> {
        let collection_id = Uuid::new_v4().to_string();
        let db_path = self.data_dir.join(format!("{}.db", collection_id));

        let conn = Connection::open(&db_path)?;
        self.init_database(&conn)?;

        let collection_data = Collection::new(name.clone(), collection_id.clone());
        let data_json = serde_json::to_string(&collection_data)?;

        conn.execute(
            "INSERT INTO collections (id, name, data) VALUES (?1, ?2, ?3)",
            params![collection_id, name, data_json],
        )?;

        Ok(collection_id)
    }

    pub fn load_collection(&mut self, collection_id: &str) -> Result<Collection> {
        let conn = self.get_connection(collection_id)?;

        let mut stmt = conn.prepare("SELECT data FROM collections WHERE id = ?1")?;
        let collection_data: String =
            stmt.query_row(params![collection_id], |row| Ok(row.get(0)?))?;

        let mut collection: Collection = serde_json::from_str(&collection_data)?;

        // Load entries (requests and folders)
        collection.entries = self.load_entries(&conn, collection_id, None)?;

        // Load environments
        let environments = self.load_environments(&conn, collection_id)?;
        for (env_key, env) in environments {
            collection.environments.update(env_key, env);
        }

        // Load scripts
        collection.scripts = self.load_scripts(&conn, collection_id)?;

        Ok(collection)
    }

    fn load_entries(
        &self,
        conn: &Connection,
        collection_id: &str,
        parent_folder_id: Option<&str>,
    ) -> Result<Vec<Entry>> {
        let mut entries = Vec::new();

        // Load folders
        let mut stmt = conn.prepare(
            "SELECT id, name, data FROM folders WHERE collection_id = ?1 AND parent_id IS ?2",
        )?;

        let folder_rows = stmt.query_map(params![collection_id, parent_folder_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?;

        for folder_row in folder_rows {
            let (folder_id, folder_name, _folder_data) = folder_row?;
            let child_entries = self.load_entries(conn, collection_id, Some(&folder_id))?;

            entries.push(Entry::Folder(Folder {
                id: FolderId::from_string(folder_id),
                name: folder_name,
                entries: child_entries,
                expanded: false,
            }));
        }

        // Load requests
        let mut stmt = conn.prepare(
            "SELECT id, name, data FROM requests WHERE collection_id = ?1 AND folder_id IS ?2",
        )?;

        let request_rows = stmt.query_map(params![collection_id, parent_folder_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?;

        for request_row in request_rows {
            let (request_id, request_name, _request_data) = request_row?;

            entries.push(Entry::Item(RequestRef {
                id: RequestId::from_string(request_id),
                name: request_name,
            }));
        }

        Ok(entries)
    }

    fn load_environments(
        &self,
        conn: &Connection,
        collection_id: &str,
    ) -> Result<Vec<(EnvironmentKey, Environment)>> {
        let mut stmt =
            conn.prepare("SELECT id, name, data FROM environments WHERE collection_id = ?1")?;
        let env_rows = stmt.query_map(params![collection_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?;

        let mut environments = Vec::new();
        for env_row in env_rows {
            let (env_id, _env_name, env_data) = env_row?;
            let env: Environment = serde_json::from_str(&env_data)?;
            let env_key = EnvironmentKey::from_string(env_id);
            environments.push((env_key, env));
        }

        Ok(environments)
    }

    fn load_scripts(&self, conn: &Connection, collection_id: &str) -> Result<Vec<Script>> {
        let mut stmt =
            conn.prepare("SELECT name, content FROM scripts WHERE collection_id = ?1")?;
        let script_rows = stmt.query_map(params![collection_id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;

        let mut scripts = Vec::new();
        for script_row in script_rows {
            let (script_name, script_content) = script_row?;
            scripts.push(Script {
                name: script_name,
                content: script_content,
            });
        }

        Ok(scripts)
    }

    fn get_connection(&self, collection_id: &str) -> Result<Connection> {
        let db_path = self.data_dir.join(format!("{}.db", collection_id));
        let conn = Connection::open(&db_path)?;
        self.init_database(&conn)?;
        Ok(conn)
    }

    fn init_database(&self, conn: &Connection) -> Result<()> {
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS collections (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                data TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS requests (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                collection_id TEXT NOT NULL,
                folder_id TEXT,
                data TEXT NOT NULL,
                FOREIGN KEY (collection_id) REFERENCES collections (id)
            );

            CREATE TABLE IF NOT EXISTS environments (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                collection_id TEXT NOT NULL,
                data TEXT NOT NULL,
                FOREIGN KEY (collection_id) REFERENCES collections (id)
            );

            CREATE TABLE IF NOT EXISTS scripts (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                collection_id TEXT NOT NULL,
                content TEXT NOT NULL,
                FOREIGN KEY (collection_id) REFERENCES collections (id)
            );

            CREATE TABLE IF NOT EXISTS folders (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                collection_id TEXT NOT NULL,
                parent_id TEXT,
                data TEXT NOT NULL,
                FOREIGN KEY (collection_id) REFERENCES collections (id),
                FOREIGN KEY (parent_id) REFERENCES folders (id)
            );
            "#,
        )?;

        Ok(())
    }

    pub fn save_request(
        &mut self,
        collection_id: &str,
        request_id: RequestId,
        name: &str,
        request: &Request,
        folder_id: Option<FolderId>,
    ) -> Result<()> {
        let conn = self.get_connection(collection_id)?;
        let data_json = serde_json::to_string(request)?;
        let folder_id_str = folder_id.map(|id| id.to_string());

        conn.execute(
            "INSERT OR REPLACE INTO requests (id, name, collection_id, folder_id, data) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![request_id.to_string(), name, collection_id, folder_id_str, data_json],
        )?;

        Ok(())
    }

    pub fn load_request(&mut self, collection_id: &str, request_id: RequestId) -> Result<Request> {
        let conn = self.get_connection(collection_id)?;

        let mut stmt =
            conn.prepare("SELECT data FROM requests WHERE collection_id = ?1 AND id = ?2")?;
        let request_data: String = stmt
            .query_row(params![collection_id, request_id.to_string()], |row| {
                Ok(row.get(0)?)
            })?;

        let request: Request = serde_json::from_str(&request_data)?;
        Ok(request)
    }

    pub fn save_environment(
        &mut self,
        collection_id: &str,
        env_id: EnvironmentKey,
        environment: &Environment,
    ) -> Result<()> {
        let conn = self.get_connection(collection_id)?;
        let data_json = serde_json::to_string(environment)?;

        conn.execute(
            "INSERT OR REPLACE INTO environments (id, name, collection_id, data) VALUES (?1, ?2, ?3, ?4)",
            params![env_id.to_string(), environment.name, collection_id, data_json],
        )?;

        Ok(())
    }

    pub fn save_script(&mut self, collection_id: &str, script: &Script) -> Result<()> {
        let conn = self.get_connection(collection_id)?;
        let script_id = Uuid::new_v4().to_string();

        conn.execute(
            "INSERT OR REPLACE INTO scripts (id, name, collection_id, content) VALUES (?1, ?2, ?3, ?4)",
            params![script_id, script.name, collection_id, script.content],
        )?;

        Ok(())
    }

    pub fn delete_request(&mut self, collection_id: &str, request_id: RequestId) -> Result<()> {
        let conn = self.get_connection(collection_id)?;

        conn.execute(
            "DELETE FROM requests WHERE collection_id = ?1 AND id = ?2",
            params![collection_id, request_id.to_string()],
        )?;

        Ok(())
    }

    pub fn delete_environment(
        &mut self,
        collection_id: &str,
        env_id: EnvironmentKey,
    ) -> Result<()> {
        let conn = self.get_connection(collection_id)?;

        conn.execute(
            "DELETE FROM environments WHERE collection_id = ?1 AND id = ?2",
            params![collection_id, env_id.to_string()],
        )?;

        Ok(())
    }

    pub fn delete_collection(&mut self, collection_id: &str) -> Result<()> {
        let conn = self.get_connection(collection_id)?;
        conn.execute(
            "DELETE FROM collections WHERE id = ?1",
            params![collection_id],
        )?;

        let db_path = self.data_dir.join(format!("{}.db", collection_id));
        if db_path.exists() {
            std::fs::remove_file(db_path)?;
        }

        Ok(())
    }
}
