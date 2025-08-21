use iced::widget::pane_grid;
use iced::widget::pane_grid::Configuration;
use tokio::sync::oneshot;

use crate::commands::builders::ResponseResult;
use crate::state::SplitState;
use crate::state::response::ResponsePane;
use core::http::request::Request;
use core::http::{CollectionKey, CollectionRequest};

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
    pub tasks: Vec<oneshot::Sender<()>>,
    pub editing_name: Option<String>,
    pub panes: pane_grid::State<SplitState>,
    pub request_dirty_state: RequestDirtyState,
}

impl HttpTab {
    pub fn new(name: &str, request: Request, req_ref: CollectionRequest) -> Box<Self> {
        Box::new(Self {
            name: name.to_owned(),
            collection_ref: req_ref,
            request: RequestPane::from(request),
            response: ResponsePane::new(),
            tasks: Vec::new(),
            panes: pane_grid::State::with_configuration(Configuration::Split {
                axis: pane_grid::Axis::Vertical,
                ratio: 0.45,
                a: Box::new(Configuration::Pane(SplitState::First)),
                b: Box::new(Configuration::Pane(SplitState::Second)),
            }),
            editing_name: None,
            request_dirty_state: RequestDirtyState::Clean,
        })
    }

    pub fn new_def() -> Box<Self> {
        Self::new("Untitled", Default::default(), Default::default())
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
        for task in self.tasks.drain(..) {
            let _ = task.send(());
        }
    }

    pub fn add_task(&mut self, task: oneshot::Sender<()>) {
        self.tasks.push(task);
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

impl Drop for HttpTab {
    fn drop(&mut self) {
        self.cancel_tasks();
    }
}
