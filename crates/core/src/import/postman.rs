// generated with `cargo typify -B crates/core/src/import/postman/schema.json`
// Schema: https://schema.postman.com/collection/json/v2.1.0/draft-07/collection.json
pub mod schema;

use anyhow::Result;

use std::path::Path;

/// Import a Postman collection from a file path
pub async fn import_postman_collection(_postman_path: &Path, _output_dir: &Path) -> Result<()> {
    Ok(())
}
