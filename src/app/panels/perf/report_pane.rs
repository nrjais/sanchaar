use iced::{
    Alignment, Element, Length, Task,
    widget::{Column, container, progress_bar, row, scrollable, space, text},
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
        PerfState::Cancelled => cancelled_view(),
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
        .push(
            text("Configure and run to see results here.")
                .size(14)
                .color(colors::WHITE),
        )
        .spacing(4)
        .align_x(Alignment::Center)
        .into()
}

fn running_view<'a>(tab: &'a PerfTab) -> Element<'a, ReportMsg> {
    let test_duration_secs = tab.config.duration.as_secs();

    let mut content = Column::new().push(space::vertical().height(12));

    if let Some(ref stats) = tab.stats {
        let elapsed_secs = stats.total_duration.as_secs_f32();
        let progress = (elapsed_secs / test_duration_secs as f32).clamp(0.0, 1.0);

        content = content
            .push(
                row![
                    text("Elapsed:").size(14),
                    text(format!("{:.1}s", elapsed_secs))
                        .size(16)
                        .color(colors::CYAN),
                    text("/").size(14),
                    text(format!("{}s", test_duration_secs))
                        .size(16)
                        .color(colors::WHITE),
                    text(format!("({:.0}%)", progress * 100.0))
                        .size(14)
                        .color(colors::YELLOW),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .push(space::vertical().height(8))
            .push(
                container(progress_bar(0.0..=test_duration_secs as f32, elapsed_secs))
                    .width(Length::Fill),
            )
            .push(space::vertical().height(8))
            .push(
                text(format!("Concurrent Users: {}", tab.config.concurrency))
                    .size(14)
                    .color(colors::WHITE),
            )
            .push(space::vertical().height(20));
    } else {
        content = content
            .push(
                text(format!(
                    "Duration: {}s | Concurrent Users: {}",
                    test_duration_secs, tab.config.concurrency
                ))
                .size(14),
            )
            .push(space::vertical().height(12))
            .push(text("Initializing test...").size(14).color(colors::WHITE))
            .push(space::vertical().height(20));
    }

    if let Some(ref stats) = tab.stats {
        let success_rate = if stats.total_requests > 0 {
            (stats.successful_requests as f64 / stats.total_requests as f64) * 100.0
        } else {
            0.0
        };

        let success_color = if success_rate > 95.0 {
            colors::GREEN
        } else if success_rate > 80.0 {
            colors::YELLOW
        } else {
            colors::RED
        };

        content = content
            .push(
                row![
                    Column::new()
                        .push(text("Total Requests").size(12).color(colors::GRAY))
                        .push(
                            text(stats.total_requests.to_string())
                                .size(28)
                                .color(colors::CYAN)
                        )
                        .spacing(4)
                        .width(Length::FillPortion(1))
                        .align_x(Alignment::Center),
                    Column::new()
                        .push(text("Requests/sec").size(12).color(colors::GRAY))
                        .push(
                            text(format!("{:.1}", stats.requests_per_second))
                                .size(28)
                                .color(colors::GREEN)
                        )
                        .spacing(4)
                        .width(Length::FillPortion(1))
                        .align_x(Alignment::Center),
                    Column::new()
                        .push(text("Success Rate").size(12).color(colors::GRAY))
                        .push(
                            text(format!("{:.1}%", success_rate))
                                .size(28)
                                .color(success_color)
                        )
                        .spacing(4)
                        .width(Length::FillPortion(1))
                        .align_x(Alignment::Center),
                ]
                .spacing(16),
            )
            .push(space::vertical().height(24))
            .push(
                Column::new()
                    .push(text("Statistics").size(16).color(colors::CYAN))
                    .push(space::vertical().height(8))
                    .push(stat_row_owned(
                        "Successful".to_string(),
                        format!("{}", stats.successful_requests),
                    ))
                    .push(stat_row_owned(
                        "Failed".to_string(),
                        format!(
                            "{} ({:.1}%)",
                            stats.failed_requests,
                            if stats.total_requests > 0 {
                                (stats.failed_requests as f64 / stats.total_requests as f64) * 100.0
                            } else {
                                0.0
                            }
                        ),
                    ))
                    .spacing(4),
            )
            .push(space::vertical().height(16))
            .push(
                Column::new()
                    .push(text("Response Latencies").size(16).color(colors::CYAN))
                    .push(space::vertical().height(8))
                    .push(stat_row_owned(
                        "Min".to_string(),
                        format_duration(stats.min),
                    ))
                    .push(stat_row_owned(
                        "Average".to_string(),
                        format_duration(stats.avg),
                    ))
                    .push(stat_row_owned(
                        "p50 (Median)".to_string(),
                        format_duration(stats.p50),
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
            );
    }

    scrollable(content).spacing(8).into()
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
        .align_x(Alignment::Center)
        .into()
}

fn cancelled_view<'a>() -> Element<'a, ReportMsg> {
    Column::new()
        .push(text("Benchmark Cancelled").size(20).color(colors::YELLOW))
        .push(space::vertical().height(20))
        .push(
            text("The performance benchmark was cancelled.")
                .size(14)
                .color(colors::WHITE),
        )
        .spacing(4)
        .align_x(Alignment::Center)
        .into()
}

fn completed_view(stats: &PerfStats) -> Element<'static, ReportMsg> {
    let mut content = Column::new()
        .push(
            Column::new()
                .push(text("Summary").size(20).color(colors::CYAN))
                .push(space::vertical().height(4))
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
                .push(text("Response Latencies").size(20).color(colors::CYAN))
                .push(space::vertical().height(4))
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
        );

    if !stats.status_codes.is_empty() {
        let mut status_column = Column::new()
            .push(space::vertical().height(12))
            .push(text("Status Distribution").size(20).color(colors::CYAN))
            .push(space::vertical().height(4));

        let mut status_codes: Vec<_> = stats.status_codes.iter().collect();
        status_codes.sort_by_key(|(code, _)| *code);

        for (code, count) in status_codes {
            let percentage = if stats.successful_requests > 0 {
                format!(
                    " ({:.1}%)",
                    (*count as f64 / stats.successful_requests as f64) * 100.0
                )
            } else {
                String::new()
            };

            status_column = status_column.push(stat_row_owned(
                format!("HTTP {}", code),
                format!("{}{}", count, percentage),
            ));
        }

        content = content.push(status_column.spacing(4));
    }

    if !stats.errors.is_empty() {
        let mut error_column = Column::new()
            .push(space::vertical().height(16))
            .push(text("Errors").size(16).color(colors::RED))
            .push(space::vertical().height(8));

        let mut errors: Vec<_> = stats.errors.iter().collect();
        errors.sort_by_key(|(_, count)| std::cmp::Reverse(*count));

        for (error, count) in errors.iter().take(10) {
            let percentage = if stats.failed_requests > 0 {
                format!(
                    " ({:.1}%)",
                    (**count as f64 / stats.failed_requests as f64) * 100.0
                )
            } else {
                String::new()
            };

            let error_label = if error.len() > 50 {
                format!("{}...", &error[..50])
            } else {
                error.to_string()
            };

            error_column = error_column.push(stat_row_owned(
                error_label,
                format!("{}{}", count, percentage),
            ));
        }

        if errors.len() > 10 {
            error_column = error_column.push(
                text(format!("... and {} more error types", errors.len() - 10))
                    .size(12)
                    .color(colors::WHITE),
            );
        }

        content = content.push(error_column.spacing(4));
    }

    content = content.spacing(4);
    scrollable(content).into()
}

fn stat_row_owned(label: String, value: String) -> Element<'static, ReportMsg> {
    row![
        text(label).width(Length::FillPortion(2)),
        text(value)
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
