use crate::state::response::ResponsePane;
use iced_aw::split;
use tokio::sync::oneshot;

use super::request::RequestPane;

#[derive(Debug)]
pub struct Tab {
    pub request: RequestPane,
    pub response: ResponsePane,
    pub tasks: Vec<oneshot::Sender<()>>,
    pub split_axis: split::Axis,
    pub split_pos: Option<u16>,
    pub editing_name: bool,
}

impl Default for Tab {
    fn default() -> Self {
        Self::new()
    }
}

impl Tab {
    pub fn new() -> Self {
        Self {
            request: RequestPane::new(),
            response: ResponsePane::new(),
            tasks: Vec::new(),
            split_axis: split::Axis::Vertical,
            split_pos: None,
            editing_name: false,
        }
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
