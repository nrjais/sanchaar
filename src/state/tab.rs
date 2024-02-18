use iced_aw::split;

use super::request::Request;

#[derive(Debug)]
pub struct Tab {
    pub request: Request,
}

impl Tab {
    pub fn new() -> Self {
        Self {
            request: Request::new(),
        }
    }
}
