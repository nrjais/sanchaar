use tokio::sync::oneshot::Sender;

use crate::commands::builders::ResponseResult;
use crate::state::response::ResponsePane;
use core::http::request::Request;
use core::http::{CollectionKey, CollectionRequest};
use core::utils::SendOnDrop;

use crate::state::request::RequestPane;
use crate::state::response::{CompletedResponse, ResponseState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestDirtyState {
    Clean,
    CheckIfDirty,
    Dirty,
}

#[derive(Debug)]
pub struct HttpTab {
    pub name: String,
    pub collection_ref: CollectionRequest,
    request: RequestPane,
    pub response: ResponsePane,
    pub cancel: SendOnDrop,
    pub editing_name: Option<String>,
    pub split_at: f32,
    pub request_dirty_state: RequestDirtyState,
}

impl HttpTab {
    pub fn new(name: &str, request: Request, req_ref: CollectionRequest) -> Box<Self> {
        Box::new(Self {
            name: name.to_owned(),
            collection_ref: req_ref,
            request: RequestPane::from(request),
            response: ResponsePane::new(),
            cancel: SendOnDrop::new(),
            split_at: 0.45,
            editing_name: None,
            request_dirty_state: RequestDirtyState::Clean,
        })
    }

    pub fn new_def() -> Box<Self> {
        Self::new("Untitled", Default::default(), Default::default())
    }

    pub fn set_split_at(&mut self, at: f32) {
        self.split_at = at.clamp(0.25, 0.70);
    }

    pub fn from_history(
        name: &str,
        request: Request,
        response: core::client::Response,
        req_ref: CollectionRequest,
    ) -> Box<Self> {
        let mut tab = Self::new(name, request, req_ref);
        tab.response.state = ResponseState::Completed(Box::new(CompletedResponse::new(response)));
        tab
    }

    pub fn is_request_dirty(&self) -> bool {
        self.request_dirty_state == RequestDirtyState::Dirty
    }

    pub fn request(&self) -> &RequestPane {
        &self.request
    }

    pub fn request_mut(&mut self) -> &mut RequestPane {
        if self.request_dirty_state == RequestDirtyState::Clean {
            self.check_dirty();
        }

        &mut self.request
    }

    pub fn mark_clean(&mut self) {
        self.request_dirty_state = RequestDirtyState::Clean;
    }

    pub fn check_dirty(&mut self) {
        self.request_dirty_state = RequestDirtyState::CheckIfDirty;
    }

    pub fn cancel_tasks(&mut self) {
        self.cancel.cancel();
        self.response.state = ResponseState::Idle;
    }

    pub fn add_task(&mut self, task: Sender<()>) {
        self.cancel.with(task);
    }

    pub fn collection_key(&self) -> CollectionKey {
        self.collection_ref.0
    }

    pub fn update_response(&mut self, result: ResponseResult) {
        self.cancel_tasks();
        match result {
            ResponseResult::Completed(res) => {
                self.response.state =
                    ResponseState::Completed(Box::new(CompletedResponse::new(res)));
            }
            ResponseResult::Error(e) => {
                self.response.state = ResponseState::Failed(e);
            }
            ResponseResult::Cancelled => (),
        }
    }
}
