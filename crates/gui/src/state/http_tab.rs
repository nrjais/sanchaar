use iced::widget::pane_grid;
use iced::widget::pane_grid::Configuration;
use tokio::sync::oneshot;

use crate::state::response::ResponsePane;
use crate::state::SplitState;
use core::http::request::Request;
use core::http::{CollectionKey, CollectionRequest};

use super::request::RequestPane;

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

impl Default for HttpTab {
    fn default() -> Self {
        HttpTab::new(
            "Untitled",
            Default::default(),
            CollectionRequest(Default::default(), Default::default()),
        )
    }
}

impl HttpTab {
    pub fn new(name: &str, request: Request, req_ref: CollectionRequest) -> Self {
        Self {
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
        }
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
}

impl Drop for HttpTab {
    fn drop(&mut self) {
        self.cancel_tasks();
    }
}
