use core::http::CollectionRequest;
use std::time::Duration;

use iced::{
    Alignment, Element, Length, Task,
    widget::{Column, Row, button, container, row, space, text},
};

use crate::{
    commands::perf::PerfResult,
    components::{split::Direction, text_input},
    state::{
        AppState, Tab,
        tabs::perf_tab::{PerfState, PerfTab},
    },
};
use crate::{commands::perf::run_perf_test, components::colors};

#[derive(Debug, Clone)]
pub enum ConfigMsg {
    SelectRequest,
    UpdateTestDuration(String),
    UpdateConcurrentWorkers(String),
    UpdateTimeout(String),
    StartTest,
    Reset,
    PerfTestCompleted(PerfResult),
}

impl ConfigMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        let Some(Tab::Perf(tab)) = state.active_tab_mut() else {
            return Task::none();
        };

        match self {
            ConfigMsg::SelectRequest => Task::none(),
            ConfigMsg::UpdateTestDuration(val) => {
                if let Ok(secs) = val.parse::<u64>() {
                    tab.config.duration = Duration::from_secs(secs.max(1));
                }
                if val.is_empty() {
                    tab.config.duration = Duration::from_secs(1);
                }
                Task::none()
            }
            ConfigMsg::UpdateConcurrentWorkers(val) => {
                if let Ok(workers) = val.parse::<usize>() {
                    tab.config.concurrency = workers.max(1).min(1000);
                }
                if val.is_empty() {
                    tab.config.concurrency = 1;
                }
                Task::none()
            }
            ConfigMsg::UpdateTimeout(val) => {
                if let Ok(millis) = val.parse::<u64>() {
                    tab.config.timeout = Duration::from_millis(millis.max(1));
                }
                if val.is_empty() {
                    tab.config.timeout = Duration::from_millis(1);
                }
                Task::none()
            }
            ConfigMsg::StartTest => {
                tab.start_test();
                run_perf_test(state, ConfigMsg::PerfTestCompleted)
            }
            ConfigMsg::Reset => {
                tab.reset();
                Task::none()
            }
            ConfigMsg::PerfTestCompleted(result) => {
                match result {
                    PerfResult::Completed(metrics) => {
                        tab.complete_test((*metrics).clone());
                    }
                    PerfResult::Error(_) => {
                        tab.fail_test();
                    }
                }
                Task::none()
            }
        }
    }
}

fn config_view<'a>(tab: &'a PerfTab) -> Element<'a, ConfigMsg> {
    let duration = text_input(
        "seconds",
        &tab.config.duration.as_secs().to_string(),
        ConfigMsg::UpdateTestDuration,
    );
    let duration = Row::new()
        .push(text("Duration (seconds)").width(Length::FillPortion(1)))
        .push(space::horizontal())
        .push(duration.width(Length::FillPortion(1)));

    let concurrency = text_input(
        "users",
        &tab.config.concurrency.to_string(),
        ConfigMsg::UpdateConcurrentWorkers,
    );
    let concurrency = Row::new()
        .push(text("Virtual Users").width(Length::FillPortion(1)))
        .push(space::horizontal())
        .push(concurrency.width(Length::FillPortion(1)));

    let timeout = text_input(
        "millis",
        &tab.config.timeout.as_millis().to_string(),
        ConfigMsg::UpdateTimeout,
    );
    let timeout = Row::new()
        .push(text("Timeout (millis)").width(Length::FillPortion(1)))
        .push(space::horizontal())
        .push(timeout.width(Length::FillPortion(1)));

    Column::from_iter([duration.into(), concurrency.into(), timeout.into()])
        .spacing(8)
        .width(Length::FillPortion(1))
        .into()
}

fn request_selector_view<'a>(tab: &'a PerfTab) -> Element<'a, ConfigMsg> {
    text("request selector")
        .width(Length::FillPortion(1))
        .into()
}

pub fn view<'a>(state: &'a AppState, tab: &'a PerfTab) -> Element<'a, ConfigMsg> {
    let config_view = config_view(tab);
    let result_view = request_selector_view(tab);

    if state.split_direction == Direction::Vertical {
        Column::from_iter([config_view, result_view])
            .spacing(16)
            .into()
    } else {
        Row::from_iter([config_view, result_view])
            .spacing(16)
            .into()
    }
}
