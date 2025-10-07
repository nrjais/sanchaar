use core::http::CollectionRequest;
use std::time::Duration;

use iced::{
    Alignment, Border, Element, Length, Point, Rectangle, Task, padding,
    widget::{Button, Column, Row, button, container, rule, space, text},
};

use crate::{
    commands::perf::PerfResult,
    components::{icon, icons, split::Direction, text_input},
    ids::PERF_REQUEST_DROP_ZONE,
    state::{
        AppState, Tab,
        tabs::perf_tab::{PerfState, PerfTab},
    },
};
use crate::{commands::perf::start_benchmark, components::NerdIcon};

#[derive(Debug, Clone)]
pub enum ConfigMsg {
    SelectRequest,
    UpdateTestDuration(String),
    UpdateConcurrentWorkers(String),
    UpdateTimeout(String),
    StartTest,
    StopTest,
    ClearRequest,
    Reset,
    Benchmark(PerfResult),
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
                    tab.config.concurrency = workers.clamp(1, 1000);
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
            ConfigMsg::StartTest => start_benchmark(state).map(ConfigMsg::Benchmark),
            ConfigMsg::StopTest => {
                tab.cancel_test();
                Task::none()
            }
            ConfigMsg::ClearRequest => {
                tab.request = None;
                Task::none()
            }
            ConfigMsg::Reset => {
                tab.reset();
                Task::none()
            }
            ConfigMsg::Benchmark(result) => {
                match result {
                    PerfResult::Progress(metrics) => {
                        tab.update_progress(metrics);
                    }
                    PerfResult::Completed(Ok(metrics)) => {
                        tab.complete_test(metrics);
                    }
                    PerfResult::Completed(Err(_)) => {
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

    let has_request = tab.request.is_some();

    let start_stop_button = match &tab.state {
        PerfState::Running => button_with_icon(icons::CloseBox, "Stop", ConfigMsg::StopTest)
            .style(button::danger)
            .on_press(ConfigMsg::StopTest),
        PerfState::Idle => button_with_icon(icons::Send, "Start", ConfigMsg::StartTest)
            .style(button::primary)
            .on_press_maybe(has_request.then_some(ConfigMsg::StartTest)),
        PerfState::Completed | PerfState::Failed | PerfState::Cancelled => {
            button_with_icon(icons::Replay, "Restart", ConfigMsg::StartTest)
                .style(button::primary)
                .on_press_maybe(has_request.then_some(ConfigMsg::StartTest))
        }
    };

    let start_stop_button = container(start_stop_button)
        .width(Length::Fill)
        .padding(padding::top(16))
        .align_x(Alignment::Center);

    Column::from_iter([
        duration.into(),
        concurrency.into(),
        timeout.into(),
        start_stop_button.into(),
    ])
    .spacing(8)
    .width(Length::FillPortion(1))
    .into()
}

fn request_selector_view<'a>(tab: &'a PerfTab, state: &'a AppState) -> Element<'a, ConfigMsg> {
    let request = tab
        .request
        .and_then(|request| state.common.collections.get_ref(request));
    let collection = tab
        .request
        .and_then(|request| state.common.collections.get(request.0));

    let reqcol = request.zip(collection);
    let content = if let Some((reqref, col)) = reqcol {
        selected_request_view(reqref, col)
    } else {
        empty_drop_zone()
    };

    container(content)
        .id(PERF_REQUEST_DROP_ZONE)
        .width(Length::FillPortion(1))
        .style(container::bordered_box)
        .padding(12)
        .into()
}

fn selected_request_view<'a>(
    request: &'a core::http::RequestRef,
    collection: &'a core::http::Collection,
) -> Column<'a, ConfigMsg> {
    let request_badge = container(
        Row::new()
            .push(
                icon(icons::API)
                    .size(16)
                    .style(|theme: &iced::Theme| text::Style {
                        color: Some(theme.extended_palette().primary.strong.color),
                    }),
            )
            .push(
                text(&request.name)
                    .size(16)
                    .style(|theme: &iced::Theme| text::Style {
                        color: Some(theme.extended_palette().background.base.text),
                    }),
            )
            .spacing(8)
            .align_y(Alignment::Center),
    )
    .padding([10, 14])
    .width(Length::Fill)
    .style(|theme: &iced::Theme| container::Style {
        background: Some(theme.extended_palette().primary.weak.color.into()),
        border: Border {
            radius: 6.0.into(),
            width: 1.0,
            color: theme.extended_palette().primary.strong.color,
        },
        ..Default::default()
    });

    let collection_row = Row::new()
        .push(
            container(icon(icons::Folder).size(16))
                .width(Length::Fixed(24.0))
                .align_x(Alignment::Center)
                .style(|theme: &iced::Theme| container::Style {
                    text_color: Some(theme.extended_palette().background.strong.text),
                    ..Default::default()
                }),
        )
        .push(
            text("Collection:")
                .size(14)
                .style(|theme: &iced::Theme| text::Style {
                    color: Some(theme.extended_palette().background.strong.text),
                }),
        )
        .push(space::horizontal().width(8))
        .push(
            text(&collection.name)
                .size(14)
                .style(|theme: &iced::Theme| text::Style {
                    color: Some(theme.extended_palette().background.base.text),
                }),
        )
        .spacing(8)
        .align_y(Alignment::Center);

    let path_display = request
        .path
        .strip_prefix(&collection.path)
        .ok()
        .and_then(|p| p.to_str())
        .unwrap_or_else(|| request.path.to_str().unwrap_or(""));

    let path_row = Row::new()
        .push(
            container(icon(icons::Path).size(16))
                .width(Length::Fixed(24.0))
                .align_x(Alignment::Center)
                .style(|theme: &iced::Theme| container::Style {
                    text_color: Some(theme.extended_palette().background.strong.text),
                    ..Default::default()
                }),
        )
        .push(
            text("Path:")
                .size(14)
                .style(|theme: &iced::Theme| text::Style {
                    color: Some(theme.extended_palette().background.strong.text),
                }),
        )
        .push(space::horizontal().width(40))
        .push(
            text(path_display)
                .size(14)
                .style(|theme: &iced::Theme| text::Style {
                    color: Some(theme.extended_palette().background.base.text),
                }),
        )
        .spacing(8)
        .align_y(Alignment::Center);

    let info_card = container(
        Column::new()
            .push(collection_row)
            .push(path_row)
            .spacing(10),
    )
    .padding(10)
    .width(Length::Fill)
    .style(|theme: &iced::Theme| container::Style {
        background: None,
        border: Border {
            radius: 4.0.into(),
            width: 1.0,
            color: theme.extended_palette().background.weak.color,
        },
        ..Default::default()
    });

    Column::new()
        .push(request_badge)
        .push(info_card)
        .spacing(12)
        .width(Length::Fill)
}

fn empty_drop_zone<'a>() -> Column<'a, ConfigMsg> {
    Column::new()
        .push(
            icon(icons::Import)
                .size(48)
                .style(|theme: &iced::Theme| text::Style {
                    color: Some(theme.extended_palette().background.weak.text),
                }),
        )
        .push(
            text("Drag request here to select")
                .size(16)
                .style(|theme: &iced::Theme| text::Style {
                    color: Some(theme.extended_palette().background.strong.text),
                }),
        )
        .spacing(12)
        .width(Length::Fill)
        .align_x(Alignment::Center)
}

fn button_with_icon<'a, M: 'a>(ico: NerdIcon, label: &'a str, on_press: M) -> Button<'a, M> {
    button(
        Row::new()
            .push(icon(ico).size(20))
            .push(text(label).size(20))
            .spacing(12)
            .align_y(Alignment::Center),
    )
    .padding([4, 12])
    .style(button::primary)
    .on_press(on_press)
}

pub fn view<'a>(state: &'a AppState, tab: &'a PerfTab) -> Element<'a, ConfigMsg> {
    let config_view = config_view(tab);
    let result_view = request_selector_view(tab, state);

    if state.split_direction == Direction::Vertical {
        let separator = rule::horizontal(2).into();
        Column::from_iter([result_view, separator, config_view])
            .spacing(16)
            .into()
    } else {
        let separator = rule::vertical(2).into();
        Row::from_iter([result_view, separator, config_view])
            .spacing(16)
            .width(Length::FillPortion(1))
            .into()
    }
}
