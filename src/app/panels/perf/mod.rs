use std::time::Duration;

use iced::padding;
use iced::{
    Element, Length, Task,
    widget::{Column, container},
};

use crate::components::split::vertical_split;
use crate::state::{AppState, tabs::perf_tab::PerfTab};

pub mod config_pane;
pub mod report_pane;

#[derive(Debug, Clone)]
pub enum PerfTabMsg {
    Config(Box<config_pane::ConfigMsg>),
    Report(report_pane::ReportMsg),
    SplitResize(f32),
}

impl PerfTabMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        match self {
            PerfTabMsg::Config(msg) => msg
                .update(state)
                .map(|msg| PerfTabMsg::Config(Box::new(msg))),
            PerfTabMsg::Report(msg) => msg.update(state).map(PerfTabMsg::Report),
            PerfTabMsg::SplitResize(ratio) => {
                let Some(crate::state::Tab::Perf(tab)) = state.active_tab_mut() else {
                    return Task::none();
                };
                tab.set_split_at(ratio);
                Task::none()
            }
        }
    }
}

pub fn view<'a>(state: &'a AppState, tab: &'a PerfTab) -> Element<'a, PerfTabMsg> {
    let config_view = config_pane::view(state, tab).map(|msg| PerfTabMsg::Config(Box::new(msg)));
    let report_view = report_pane::view(state, tab).map(PerfTabMsg::Report);

    let panes = vertical_split(
        config_view,
        report_view,
        tab.split_at,
        PerfTabMsg::SplitResize,
    )
    .direction(state.split_direction)
    .focus_delay(Duration::from_millis(50))
    .handle_width(8.);

    let content = container(panes).padding(padding::top(4));

    Column::new()
        .push(content)
        .height(Length::Fill)
        .width(Length::Fill)
        .spacing(4)
        .into()
}
