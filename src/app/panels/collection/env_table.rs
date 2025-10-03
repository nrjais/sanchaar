use core::http::Collection;

use iced::{
    Element, Length,
    widget::{container, table, text},
};

use crate::{
    app::panels::collection::env_editor::Message,
    components::{bold, line_editor},
    state::{environment::EnvVariable, tabs::collection_tab::CollectionTab},
};

pub fn view<'a>(tab: &'a CollectionTab, col: &'a Collection) -> Element<'a, Message> {
    let editor = &tab.env_editor;
    let vars = col.dotenv_env_chain().all_var_set();

    let mut columns = vec![
        table::column(bold("Key"), |(index, env): (usize, &EnvVariable)| {
            container(
                line_editor(&env.name)
                    .highlight(false)
                    .placeholder("Name")
                    .map(move |msg| Message::UpdateVarName(index, msg)),
            )
            .width(Length::Fixed(150.))
        })
        .width(Length::Fixed(150.)),
    ];
    columns.extend(editor.environments.iter().map(|(env_key, env)| {
        let vars = vars.clone();
        table::column(
            bold(env.name.as_str()),
            move |(index, env): (usize, &EnvVariable)| -> Element<'a, Message> {
                let var = env.values.get(env_key);
                if let Some(var) = var {
                    container(
                        line_editor(var)
                            .vars(vars.clone())
                            .editable()
                            .map(move |msg| Message::UpdateVarValue(index, *env_key, msg)),
                    )
                    .width(Length::Fixed(200.))
                    .into()
                } else {
                    text("").into()
                }
            },
        )
        .width(Length::Fixed(200.))
    }));

    container(table(columns, editor.variables.iter().enumerate()))
        .height(Length::Shrink)
        .width(Length::Shrink)
        .into()
}
