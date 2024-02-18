use super::request::Request;

#[derive(Debug)]
pub struct Tab {
    pub request: Request,
}

impl Default for Tab {
    fn default() -> Self {
        Self::new()
    }
}

impl Tab {
    pub fn new() -> Self {
        Self {
            request: Request::new(),
        }
    }
}
