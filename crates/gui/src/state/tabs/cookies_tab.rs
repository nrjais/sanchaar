use std::sync::Arc;

use cookie_store::Cookie;
use reqwest_cookie_store::CookieStoreRwLock;

use crate::state::CommonState;

#[derive(Debug, Clone)]
pub struct CookiesTab {
    pub store: Arc<CookieStoreRwLock>,
}

impl CookiesTab {
    pub fn new(state: &CommonState) -> Self {
        Self {
            store: Arc::clone(&state.cookie_store),
        }
    }

    pub fn cookies(&self) -> Vec<Cookie<'static>> {
        self.store
            .read()
            .expect("Lock")
            .iter_unexpired()
            .cloned()
            .collect()
    }
}
