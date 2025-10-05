use iced::{
    Alignment, Element, Length, Task,
    widget::{Column, container, row, scrollable, space, text},
};

use crate::components::colors;
use crate::state::tabs::perf_tab::{PerfState, PerfTab};
use core::perf::PerfStats;

#[derive(Debug, Clone)]
pub enum ReportMsg {}

impl ReportMsg {
    pub fn update(self, _state: &mut crate::state::AppState) -> Task<Self> {
        Task::none()
    }
}

pub fn view<'a>(tab: &'a PerfTab) -> Element<'a, ReportMsg> {
    let content = match tab.state {
        PerfState::Idle => empty_view(),
        PerfState::Running => running_view(tab),
        PerfState::Completed => {
            if let Some(ref stats) = tab.stats {
                completed_view(stats)
            } else {
                empty_view()
            }
        }
        PerfState::Failed => failed_view(),
    };

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn empty_view<'a>() -> Element<'a, ReportMsg> {
    Column::new()
        .push(text("Performance Report").size(20))
        .push(space::vertical().height(20))
        .push(text("Configure and run to see results here.").size(14))
        .spacing(4)
        .align_x(Alignment::Center)
        .into()
}

fn running_view<'a>(tab: &'a PerfTab) -> Element<'a, ReportMsg> {
    let test_duration_secs = tab.config.duration.as_secs();

    Column::new()
        .push(text("Test Running...").size(20).color(colors::YELLOW))
        .push(space::vertical().height(20))
        .push(
            text(format!(
                "Test Duration: {}s | Requests Completed: {}",
                test_duration_secs, tab.progress
            ))
            .size(16),
        )
        .spacing(4)
        .into()
}

fn failed_view<'a>() -> Element<'a, ReportMsg> {
    Column::new()
        .push(text("Test Failed").size(20).color(colors::RED))
        .push(space::vertical().height(20))
        .push(
            text("An error occurred while running the performance test.")
                .size(14)
                .color(colors::WHITE),
        )
        .spacing(4)
        .into()
}

fn completed_view(stats: &PerfStats) -> Element<'static, ReportMsg> {
    let content = Column::new()
        .push(
            text("Performance Test Results")
                .size(20)
                .color(colors::PRIMARY),
        )
        .push(space::vertical().height(20))
        .push(
            Column::new()
                .push(text("Summary").size(16).color(colors::CYAN))
                .push(space::vertical().height(8))
                .push(stat_row_owned(
                    "Total Requests".to_string(),
                    stats.total_requests.to_string(),
                ))
                .push(stat_row_owned(
                    "Successful".to_string(),
                    if stats.total_requests > 0 {
                        format!(
                            "{} ({:.1}%)",
                            stats.successful_requests,
                            (stats.successful_requests as f64 / stats.total_requests as f64)
                                * 100.0
                        )
                    } else {
                        stats.successful_requests.to_string()
                    },
                ))
                .push(stat_row_owned(
                    "Failed".to_string(),
                    if stats.total_requests > 0 {
                        format!(
                            "{} ({:.1}%)",
                            stats.failed_requests,
                            (stats.failed_requests as f64 / stats.total_requests as f64) * 100.0
                        )
                    } else {
                        stats.failed_requests.to_string()
                    },
                ))
                .push(stat_row_owned(
                    "Requests/sec".to_string(),
                    format!("{:.2}", stats.requests_per_second),
                ))
                .push(stat_row_owned(
                    "Total Duration".to_string(),
                    format!("{:.2}s", stats.total_duration.as_secs_f64()),
                ))
                .spacing(4),
        )
        .push(space::vertical().height(16))
        .push(
            Column::new()
                .push(
                    text("Response Time Percentiles")
                        .size(16)
                        .color(colors::CYAN),
                )
                .push(space::vertical().height(8))
                .push(stat_row_owned(
                    "Min".to_string(),
                    format_duration(stats.min),
                ))
                .push(stat_row_owned(
                    "p50 (Median)".to_string(),
                    format_duration(stats.p50),
                ))
                .push(stat_row_owned(
                    "Average".to_string(),
                    format_duration(stats.avg),
                ))
                .push(stat_row_owned(
                    "p95".to_string(),
                    format_duration(stats.p95),
                ))
                .push(stat_row_owned(
                    "p99".to_string(),
                    format_duration(stats.p99),
                ))
                .push(stat_row_owned(
                    "Max".to_string(),
                    format_duration(stats.max),
                ))
                .spacing(4),
        )
        .spacing(4);

    scrollable(content).into()
}

fn stat_row_owned(label: String, value: String) -> Element<'static, ReportMsg> {
    row![
        text(label).size(14).width(Length::FillPortion(2)),
        text(value)
            .size(14)
            .width(Length::FillPortion(1))
            .color(colors::WHITE),
    ]
    .padding(4)
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}

fn format_duration(duration: Option<std::time::Duration>) -> String {
    duration
        .map(|d| {
            let millis = d.as_millis();
            if millis < 1000 {
                format!("{}ms", millis)
            } else {
                format!("{:.2}s", d.as_secs_f64())
            }
        })
        .unwrap_or_else(|| "N/A".to_string())
}
