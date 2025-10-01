use std::{collections::HashSet, sync::Arc};

use iced::widget::{container, scrollable};

use crate::{
    components::{ContentType, code_editor, key_value_editor},
    state::request::{BulkEditMsg, BulkEditable},
};

pub fn view(keys: &BulkEditable, vars: Arc<HashSet<String>>) -> iced::Element<BulkEditMsg> {
    match keys {
        BulkEditable::KeyValue(keys) => {
            scrollable(key_value_editor(keys, &vars).on_change(BulkEditMsg::KeyValue))
                .height(iced::Length::Shrink)
                .width(iced::Length::Shrink)
                .into()
        }
        BulkEditable::Editor(content) => container(
            code_editor(content, ContentType::Text)
                .editable()
                .map(BulkEditMsg::Editor),
        )
        .style(container::bordered_box)
        .height(iced::Length::Fill)
        .width(iced::Length::Fill)
        .into(),
    }
}
