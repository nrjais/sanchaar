use core::http::CollectionRequest;
use std::time::Duration;

use iced::{
    Element, Length, Point, Rectangle, Task,
    advanced::widget,
    widget::{Column, Row, container, rule, space, text},
};

use crate::commands::perf::run_perf_test;
use crate::{
    commands::perf::PerfResult,
    components::{split::Direction, text_input},
    state::{AppState, Tab, tabs::perf_tab::PerfTab},
};

#[derive(Debug, Clone)]
pub enum ConfigMsg {
    SelectRequest,
    UpdateTestDuration(String),
    UpdateConcurrentWorkers(String),
    UpdateTimeout(String),
    StartTest,
    Reset,
    PerfTestCompleted(PerfResult),
    Drop(Point, Rectangle, CollectionRequest),
    HandleZones(
        Vec<(iced::advanced::widget::Id, Rectangle)>,
        CollectionRequest,
    ),
}

impl ConfigMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        let Some(Tab::Perf(tab)) = state.active_tab_mut() else {
            return Task::none();
        };

        match self {
            ConfigMsg::SelectRequest => Task::none(),
            ConfigMsg::Drop(point, _, request) => iced_drop::zones_on_point(
                move |zones| ConfigMsg::HandleZones(zones, request),
                point,
                None,
                None,
            ),
            ConfigMsg::HandleZones(zones, request) => {
                if !zones.is_empty() {
                    // Request was dropped on the drop zone
                    tab.set_request(request);
                }
                Task::none()
            }
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

fn request_selector_view<'a>(tab: &'a PerfTab, state: &'a AppState) -> Element<'a, ConfigMsg> {
    let drop_zone_id = widget::Id::from("perf_request_drop_zone");

    let content = if let (Some(col_key), Some(req_id)) = (tab.collection, tab.request) {
        if let Some(col_req) = state
            .common
            .collections
            .get_ref(CollectionRequest(col_key, req_id))
        {
            Column::new()
                .push(text("Selected Request:").size(14))
                .push(text(&col_req.name).size(18))
                .spacing(8)
        } else {
            Column::new().push(text("Drag and drop a request here").size(16))
        }
    } else {
        Column::new().push(text("Drag and drop a request here").size(16))
    };

    container(content)
        .id(drop_zone_id)
        .width(Length::FillPortion(1))
        .padding(16)
        .style(|theme: &iced::Theme| container::Style {
            border: iced::Border {
                width: 2.0,
                color: theme.extended_palette().primary.weak.color,
                radius: 8.0.into(),
            },
            ..Default::default()
        })
        .into()
}

pub fn view<'a>(state: &'a AppState, tab: &'a PerfTab) -> Element<'a, ConfigMsg> {
    let config_view = config_view(tab);
    let result_view = request_selector_view(tab, state);

    if state.split_direction == Direction::Vertical {
        let separator = rule::horizontal(2).into();
        Column::from_iter([config_view, separator, result_view])
            .spacing(16)
            .into()
    } else {
        let separator = rule::vertical(2).into();
        Row::from_iter([result_view, separator, config_view])
            .spacing(16)
            .into()
    }
}
