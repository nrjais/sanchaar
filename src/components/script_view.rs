use iced::widget::{Column, Row, button, container, pick_list, scrollable, text};
use iced::{Element, Length};
use lib::http::collection::Script;

use crate::components::editor::Content;
use crate::components::{ContentType, code_editor, icon_button, icons, tooltip};

/// Configuration for script view behavior
pub struct ScriptViewConfig {
    pub editable: bool,
    pub show_create_button: bool,
    pub enable_type_tabs: bool,
}

impl Default for ScriptViewConfig {
    fn default() -> Self {
        Self {
            editable: false,
            show_create_button: false,
            enable_type_tabs: false,
        }
    }
}

/// Renders a script selector dropdown with optional create/remove/save buttons
pub fn script_selector<'a, Message: 'a + Clone>(
    scripts: &'a [Script],
    selected: Option<&'a String>,
    on_select: impl Fn(String) -> Message + 'a,
    on_deselect: Option<Message>,
    on_create: Option<Message>,
    on_save: Option<(bool, Message)>, // (is_edited, save_message)
) -> Element<'a, Message> {
    let picker = pick_list(
        scripts.iter().map(|s| s.name.clone()).collect::<Vec<_>>(),
        selected,
        move |s| on_select(s),
    )
    .placeholder("Select Script")
    .width(Length::Fill)
    .padding([2, 8])
    .text_size(14);

    let mut selector_row = Row::new()
        .push(text("Script:").width(Length::Shrink))
        .push(picker)
        .spacing(12)
        .align_y(iced::Alignment::Center);

    // Add save button if provided
    if let Some((is_edited, save_msg)) = on_save {
        let save_button = if is_edited {
            button("Save Script")
                .padding([6, 16])
                .on_press(save_msg)
                .style(button::primary)
        } else {
            button("Saved").padding([6, 16]).style(button::secondary)
        };
        selector_row = selector_row.push(save_button);
    }

    // Add action buttons to the same row
    if let Some(create_msg) = on_create {
        selector_row = selector_row.push(tooltip(
            "New Script",
            icon_button(icons::Plus, Some(18), Some(8))
                .on_press(create_msg)
                .style(button::secondary),
        ));
    }

    if let Some(deselect_msg) = on_deselect {
        selector_row = selector_row.push(tooltip(
            "Remove Script",
            icon_button(icons::Close, Some(18), Some(8))
                .on_press_maybe(selected.map(|_| deselect_msg))
                .style(button::secondary),
        ));
    }

    selector_row = selector_row.spacing(8);

    Column::new().push(selector_row).into()
}

/// Renders the script editor without header bar
pub fn script_editor<'a, Message: 'a + Clone>(
    script_content: &'a Content,
    editable: bool,
    on_edit: impl Fn(crate::components::CodeEditorMsg) -> Message + 'a,
) -> Element<'a, Message> {
    let editor = if editable {
        code_editor(script_content, ContentType::JS)
            .editable()
            .map(on_edit)
    } else {
        code_editor(script_content, ContentType::JS).map(on_edit)
    };

    container(scrollable(editor))
        .padding(12)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(container::rounded_box)
        .into()
}

/// Renders a placeholder message when no script is selected or when loading
pub fn script_placeholder<'a, Message: 'a>(message: &'a str) -> Element<'a, Message> {
    container(text(message).size(14))
        .padding(12)
        .center(Length::Fill)
        .into()
}

/// Simple script list view for collection panel (read-only)
pub fn script_list_view<'a, Message: 'a + Clone>(
    scripts: &'a [Script],
    selected: Option<&'a String>,
    script_content: Option<&'a Content>,
    on_select: impl Fn(Option<String>) -> Message + 'a,
    on_edit: impl Fn(crate::components::CodeEditorMsg) -> Message + 'a,
) -> Element<'a, Message> {
    if scripts.is_empty() {
        return script_placeholder("No scripts in this collection");
    }

    let script_list = Column::new()
        .push(text("Scripts:").size(16))
        .push(
            Column::with_children(
                scripts
                    .iter()
                    .map(|script| {
                        let is_selected = selected.map(|s| s == &script.name).unwrap_or(false);
                        let btn_style = if is_selected {
                            button::primary
                        } else {
                            button::secondary
                        };

                        button(text(&script.name).size(14))
                            .width(Length::Fill)
                            .padding([6, 12])
                            .style(btn_style)
                            .on_press(on_select(Some(script.name.clone())))
                            .into()
                    })
                    .collect::<Vec<_>>(),
            )
            .spacing(4),
        )
        .spacing(8)
        .padding(12)
        .width(Length::FillPortion(1));

    let content_viewer: Element<'a, Message> = if selected.is_some() {
        if let Some(content) = script_content {
            script_editor(content, false, on_edit)
        } else {
            script_placeholder("Loading script...")
        }
    } else {
        script_placeholder("Select a script to view its content")
    };

    let content_pane = Column::new()
        .push(content_viewer)
        .width(Length::FillPortion(3))
        .height(Length::Fill);

    Row::new()
        .push(script_list)
        .push(content_pane)
        .spacing(8)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
