use iced::{Element, Task};

use lib::http::collection::Collection;

use crate::commands::builders::delete_script_cmd;
use crate::components::{CodeEditorMsg, script_editor, script_placeholder, script_selector};
use crate::state::AppState;
use crate::state::popups::{Popup, PopupNameAction};
use crate::state::tabs::collection_tab::CollectionTab;

#[derive(Debug, Clone)]
pub enum Message {
    SelectScript(Option<String>),
    LoadScriptContent(String),
    EditorAction(CodeEditorMsg),
    SaveScript,
    ScriptSaved,
    CreateScript,
    RenameScript,
    DeleteScript,
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
                    tab.script_edited = false;
                }
                Task::none()
            }
            Message::LoadScriptContent(content) => {
                let Some(crate::state::Tab::Collection(tab)) = state.active_tab_mut() else {
                    return Task::none();
                };
                tab.script_content = crate::components::editor::Content::with_text(&content);
                tab.script_edited = false;
                Task::none()
            }
            Message::EditorAction(msg) => {
                let Some(crate::state::Tab::Collection(tab)) = state.active_tab_mut() else {
                    return Task::none();
                };
                msg.update(&mut tab.script_content);
                tab.script_edited = true;
                Task::none()
            }
            Message::SaveScript => {
                let Some(crate::state::Tab::Collection(tab)) = state.active_tab_mut() else {
                    return Task::none();
                };

                let Some(script_name) = tab.selected_script.clone() else {
                    return Task::none();
                };

                let text = tab.script_content.text();
                let col_key = tab.collection_key;

                // Release the mutable borrow before getting the collection
                let col = state.common.collections.get(col_key);
                if let Some(col) = col
                    && let Some(path) = col.get_script_path(&script_name)
                {
                    return Task::perform(tokio::fs::write(path.clone(), text), move |result| {
                        match result {
                            Ok(_) => {
                                log::info!("Script saved successfully");
                                Message::ScriptSaved
                            }
                            Err(e) => {
                                log::error!("Failed to save script: {}", e);
                                Message::ScriptSaved
                            }
                        }
                    });
                }
                Task::none()
            }
            Message::ScriptSaved => {
                let Some(crate::state::Tab::Collection(tab)) = state.active_tab_mut() else {
                    return Task::none();
                };
                tab.script_edited = false;
                Task::none()
            }
            Message::CreateScript => {
                let col_key = {
                    let Some(crate::state::Tab::Collection(tab)) = state.active_tab_mut() else {
                        return Task::none();
                    };
                    tab.collection_key
                };
                Popup::popup_name(
                    &mut state.common,
                    String::new(),
                    PopupNameAction::NewScript(col_key),
                );
                Task::none()
            }
            Message::RenameScript => {
                let (col_key, script_name) = {
                    let Some(crate::state::Tab::Collection(tab)) = state.active_tab_mut() else {
                        return Task::none();
                    };
                    let Some(script_name) = tab.selected_script.clone() else {
                        return Task::none();
                    };
                    (tab.collection_key, script_name)
                };
                let name = script_name
                    .strip_suffix(".js")
                    .unwrap_or(&script_name)
                    .to_string();
                Popup::popup_name(
                    &mut state.common,
                    name,
                    PopupNameAction::RenameScript(col_key, script_name),
                );
                Task::none()
            }
            Message::DeleteScript => {
                let Some(crate::state::Tab::Collection(tab)) = state.active_tab_mut() else {
                    return Task::none();
                };
                let Some(script_name) = tab.selected_script.take() else {
                    return Task::none();
                };
                let col_key = tab.collection_key;
                tab.script_content = crate::components::editor::Content::new();
                tab.script_edited = false;
                let _ = delete_script_cmd(&mut state.common, col_key, script_name);
                Task::none()
            }
        }
    }
}

pub fn view<'a>(tab: &'a CollectionTab, col: &'a Collection) -> Element<'a, Message> {
    use crate::components::{icon_button, icons, tooltip};
    use iced::Length;
    use iced::widget::{Column, Row, button};

    let scripts = &col.scripts;
    let selected = tab.selected_script.as_ref();

    // Top row with selector and actions
    let mut selector_row = Row::new();

    // Script selector with save button
    let selector = script_selector(
        scripts,
        selected,
        |s| Message::SelectScript(Some(s)),
        None, // Remove deselect button
        None, // No create in selector
        selected.map(|_| (tab.script_edited, Message::SaveScript)),
    );

    selector_row = selector_row.push(selector);

    // Action buttons
    selector_row = selector_row
        .push(tooltip(
            "New Script",
            icon_button(icons::Plus, Some(18), Some(8))
                .on_press(Message::CreateScript)
                .style(button::secondary),
        ))
        .push(tooltip(
            "Rename Script",
            icon_button(icons::Edit, Some(18), Some(8))
                .on_press_maybe(selected.map(|_| Message::RenameScript))
                .style(button::secondary),
        ))
        .push(tooltip(
            "Delete Script",
            icon_button(icons::Delete, Some(18), Some(8))
                .on_press_maybe(selected.map(|_| Message::DeleteScript))
                .style(button::secondary),
        ))
        .spacing(8)
        .align_y(iced::Alignment::Center);

    // Editor view
    let editor_view: Element<'a, Message> = if selected.is_some() {
        script_editor(&tab.script_content, true, Message::EditorAction)
    } else if scripts.is_empty() {
        script_placeholder("No scripts in this collection")
    } else {
        script_placeholder("Select a script to edit")
    };

    Column::new()
        .push(selector_row)
        .push(editor_view)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
