use iced::widget::pane_grid;
use iced::widget::pane_grid::Configuration;
use std::path::PathBuf;
use tokio::sync::oneshot;

use crate::state::response::ResponsePane;
use crate::state::{CollectionKey, SplitState};

use super::request::{Request, RequestPane};

#[derive(Debug)]
pub struct Tab {
    pub req_ref: Option<(CollectionKey, PathBuf)>,
    pub request: RequestPane,
    pub response: ResponsePane,
    pub tasks: Vec<oneshot::Sender<()>>,
    pub editing_name: bool,
    pub panes: pane_grid::State<SplitState>,
}

impl Default for Tab {
    fn default() -> Self {
        Self::new(Request::default())
    }
}

impl Tab {
    pub fn new(request: Request) -> Self {
        Self {
            req_ref: None,
            request: RequestPane::from(request),
            response: ResponsePane::new(),
            tasks: Vec::new(),
            panes: pane_grid::State::with_configuration(Configuration::Split {
                axis: pane_grid::Axis::Vertical,
                ratio: 0.45,
                a: Box::new(Configuration::Pane(SplitState::First)),
                b: Box::new(Configuration::Pane(SplitState::Second)),
            }),
            editing_name: false,
        }
    }

    pub fn set_req_ref(mut self, col: CollectionKey, path: PathBuf) -> Self {
        self.req_ref = Some((col, path));
        self
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
