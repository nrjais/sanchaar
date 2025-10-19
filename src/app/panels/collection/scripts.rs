use iced::widget::{Column, Row, button, container, scrollable, text};
use iced::{Element, Length, Task};

use lib::http::collection::Collection;

use crate::components::{CodeEditorMsg, ContentType, code_editor};
use crate::state::AppState;
use crate::state::tabs::collection_tab::CollectionTab;

#[derive(Debug, Clone)]
pub enum Message {
    SelectScript(Option<String>),
    LoadScriptContent(String),
    EditorAction(CodeEditorMsg),
}

impl Message {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        match self {
            Message::SelectScript(script) => {
                let Some(crate::state::Tab::Collection(tab)) = state.active_tab_mut() else {
                    return Task::none();
                };

                let collection_key = tab.collection_key;
                tab.selected_script = script.clone();

                // Load script content if a script is selected
                if let Some(script_name) = script {
                    let col = state.common.collections.get(collection_key);
                    if let Some(col) = col
                        && let Some(path) = col.get_script_path(&script_name)
                    {
                        return Task::perform(
                            tokio::fs::read_to_string(path),
                            |result| match result {
                                Ok(content) => Message::LoadScriptContent(content),
                                Err(e) => {
                                    log::error!("Failed to load script: {}", e);
                                    Message::LoadScriptContent(String::new())
                                }
                            },
                        );
                    }
                } else {
                    tab.script_content = crate::components::editor::Content::new();
                }
                Task::none()
            }
            Message::LoadScriptContent(content) => {
                let Some(crate::state::Tab::Collection(tab)) = state.active_tab_mut() else {
                    return Task::none();
                };
                tab.script_content = crate::components::editor::Content::with_text(&content);
                Task::none()
            }
            Message::EditorAction(_) => {
                // Read-only viewer, ignore editor actions
                Task::none()
            }
        }
    }
}

pub fn view<'a>(tab: &'a CollectionTab, col: &'a Collection) -> Element<'a, Message> {
    let scripts = &col.scripts;
    let selected = tab.selected_script.as_ref();

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
                            .on_press(Message::SelectScript(Some(script.name.clone())))
                            .into()
                    })
                    .collect::<Vec<_>>(),
            )
            .spacing(4),
        )
        .spacing(8)
        .padding(12)
        .width(Length::FillPortion(1));

    let content_viewer: Element<'a, Message> = if let Some(script_name) = selected {
        let script_content = &tab.script_content;

        Column::new()
            .push(
                container(text(format!("Script: {}", script_name)).size(16))
                    .padding(8)
                    .width(Length::Fill)
                    .style(container::rounded_box),
            )
            .push(
                container(
                    scrollable(
                        code_editor(script_content, ContentType::JavaScript)
                            .map(Message::EditorAction),
                    )
                    .height(Length::Fill),
                )
                .padding(iced::padding::top(8).right(12).bottom(12).left(12))
                .width(Length::Fill)
                .height(Length::Fill),
            )
            .spacing(4)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    } else {
        container(text("Select a script to view its content").size(14))
            .padding(12)
            .center(Length::Fill)
            .into()
    };

    let content_pane = Column::new()
        .push(content_viewer)
        .width(Length::FillPortion(3))
        .height(Length::Fill);

    if scripts.is_empty() {
        container(text("No scripts in this collection").size(14))
            .padding(12)
            .center(Length::Fill)
            .into()
    } else {
        Row::new()
            .push(script_list)
            .push(content_pane)
            .spacing(8)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
