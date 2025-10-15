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

    pub fn delete_cookie(&mut self, name: &str, domain: &str, path: &str) {
        let mut store = self.store.write().expect("Lock");
        let cookies_to_remove: Vec<_> = store
            .iter_any()
            .filter(|cookie| {
                cookie.name() == name
                    && cookie.domain().unwrap_or_default() == domain
                    && cookie.path().unwrap_or_default() == path
            })
            .cloned()
            .collect();

        for cookie in cookies_to_remove {
            store.remove(
                cookie.domain().unwrap_or_default(),
                cookie.path().unwrap_or_default(),
                cookie.name(),
            );
        }
    }

    pub fn clear_all(&mut self) {
        let mut store = self.store.write().expect("Lock");
        store.clear();
    }
}
