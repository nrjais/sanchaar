use super::request::Request;
use super::{CollectionRequest, Collections};
use crate::persistence::request::read_request;

impl Collections {
    pub async fn load_request(&self, req: CollectionRequest) -> anyhow::Result<Option<Request>> {
        let Some(req_ref) = self.get_ref(req) else {
            return Ok(None);
        };

        let request = read_request(&req_ref.path).await?;
        Ok(Some(request))
    }
}
