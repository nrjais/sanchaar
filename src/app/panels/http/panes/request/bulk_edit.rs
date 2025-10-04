use std::{collections::HashSet, sync::Arc};

use iced::widget::{container, scrollable};

use crate::{
    components::{ContentType, code_editor, key_value_editor},
    state::request::{BulkEditMsg, BulkEditable},
};

pub fn view(
    keys: &BulkEditable,
    vars: Arc<HashSet<String>>,
    should_scroll: bool,
) -> iced::Element<BulkEditMsg> {
    match keys {
        BulkEditable::KeyValue(keys) => {
            let editor = key_value_editor(keys, &vars).on_change(BulkEditMsg::KeyValue);
            if should_scroll {
                scrollable(editor)
                    .height(iced::Length::Shrink)
                    .width(iced::Length::Shrink)
                    .into()
            } else {
                editor
            }
        }
        BulkEditable::Editor(content) => container(
            code_editor(content, ContentType::Text)
                .editable()
                .map(BulkEditMsg::Editor),
        )
        .style(container::bordered_box)
        .height(iced::Length::Fixed(200.))
        .width(iced::Length::Fill)
        .into(),
    }
}
