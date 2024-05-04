use iced::widget::pane_grid;
use iced::widget::pane_grid::Configuration;
use tokio::sync::oneshot;

use crate::state::response::ResponsePane;
use crate::state::SplitState;
use core::http::request::Request;
use core::http::CollectionRequest;

use super::request::RequestPane;

core::new_id_type!(
    pub struct TabOpenId;
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestDirtyState {
    Clean,
    CheckIfDirty,
    Dirty,
}

#[derive(Debug)]
pub struct Tab {
    pub id: TabOpenId,
    pub collection_ref: Option<CollectionRequest>,
    request: RequestPane,
    pub response: ResponsePane,
    pub tasks: Vec<oneshot::Sender<()>>,
    pub editing_name: Option<String>,
    pub panes: pane_grid::State<SplitState>,
    pub request_dirty_state: RequestDirtyState,
}

impl Default for Tab {
    fn default() -> Self {
        Self::new(Request::default())
    }
}

impl Tab {
    pub fn new(request: Request) -> Self {
        Self {
            id: TabOpenId::new(),
            collection_ref: None,
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
            self.request_dirty_state = RequestDirtyState::CheckIfDirty;
        }

        &mut self.request
    }

    pub fn check_dirty(&mut self) {
        self.request_dirty_state = RequestDirtyState::CheckIfDirty;
    }

    pub(crate) fn with_ref(req: Request, req_ref: CollectionRequest) -> Tab {
        let mut tab = Tab::new(req);
        tab.collection_ref = Some(req_ref);
        tab
    }

    pub fn cancel_tasks(&mut self) {
        for task in self.tasks.drain(..) {
            let _ = task.send(());
        }
    }

    pub fn add_task(&mut self, task: oneshot::Sender<()>) {
        self.tasks.push(task);
    }
}

impl Drop for Tab {
    fn drop(&mut self) {
        self.cancel_tasks();
    }
}
