use std::sync::Arc;

use cookie_store::Cookie;
use reqwest_cookie_store::CookieStoreRwLock;

use crate::components::editor;
use crate::state::CommonState;

#[derive(Debug)]
pub struct CookiesTab {
    pub store: Arc<CookieStoreRwLock>,
    pub search_query: editor::Content,
    pub search_query_text: String,
    pub filtered_cookies: Vec<Cookie<'static>>,
}

impl CookiesTab {
    pub fn new(state: &CommonState) -> Self {
        let store = Arc::clone(&state.cookie_store);
        let filtered_cookies = Self::get_all_cookies(&store);
        Self {
            store,
            search_query: editor::Content::new(),
            search_query_text: String::new(),
            filtered_cookies,
        }
    }

    fn get_all_cookies(store: &Arc<CookieStoreRwLock>) -> Vec<Cookie<'static>> {
        store
            .read()
            .expect("Lock")
            .iter_unexpired()
            .cloned()
            .collect()
    }

    pub fn cookies(&self) -> &[Cookie<'static>] {
        &self.filtered_cookies
    }

    pub fn set_search_query(&mut self, query: &str) {
        self.search_query_text = query.to_string();
        self.update_filtered_cookies();
    }

    pub fn clear_search_query(&mut self) {
        use crate::components::editor::ContentAction;
        self.search_query
            .perform(ContentAction::Replace("".to_string()));
        self.search_query_text.clear();
        self.update_filtered_cookies();
    }

    fn update_filtered_cookies(&mut self) {
        let all_cookies = Self::get_all_cookies(&self.store);

        if self.search_query_text.is_empty() {
            self.filtered_cookies = all_cookies;
        } else {
            let query = self.search_query_text.to_lowercase();
            self.filtered_cookies = all_cookies
                .into_iter()
                .filter(|cookie| {
                    cookie.name().to_lowercase().contains(&query)
                        || cookie.value().to_lowercase().contains(&query)
                        || cookie
                            .domain()
                            .unwrap_or_default()
                            .to_lowercase()
                            .contains(&query)
                })
                .collect();
        }
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
        drop(store);
        self.update_filtered_cookies();
    }

    pub fn clear_all(&mut self) {
        let mut store = self.store.write().expect("Lock");
        store.clear();
        drop(store);
        self.update_filtered_cookies();
    }
}
