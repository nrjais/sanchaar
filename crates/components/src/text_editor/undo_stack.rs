#![allow(unused_variables, dead_code)]

use std::sync::Arc;

use iced_core::text::editor::{Action, Edit};
use iced_core::text::Editor;

pub(crate) struct EditorAction {
    pub pre_cursor_position: (usize, usize),
    pub post_cursor_position: (usize, usize),
    pub pre_selection: Option<Arc<String>>,
    pub char_at_cursor: Option<char>,
    pub char_after_cursor: Option<char>,
    pub edit: Edit,
}

pub struct UndoStack {
    stack: Vec<EditorAction>,
    current_index: usize,
}

impl UndoStack {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            current_index: 0,
        }
    }

    pub fn push(&mut self, action: EditorAction) {
        self.stack.truncate(self.current_index);
        self.stack.push(action);
        self.current_index += 1;
    }

    pub fn undo(&mut self, content: &mut impl Editor) {
        if self.current_index == 0 {
            return;
        }
        let actions = self.stack[0..self.current_index].iter().enumerate().rev();
        let cursor_position = content.cursor_position();

        fn paste_selection(action: &EditorAction, content: &mut impl Editor) {
            if let Some(selection) = &action.pre_selection {
                content.perform(Action::Edit(Edit::Paste(Arc::clone(selection))));
            }
        }
        let mut insert = false;

        for (index, action) in actions {
            if insert && !matches!(action.edit, Edit::Insert(_)) {
                break;
            }
            self.current_index = index;

            // TODO: Find way to restore selection
            match &action.edit {
                Edit::Insert(_) => {
                    insert = true;
                    content.perform(Action::Cursor(
                        action.post_cursor_position.0,
                        action.post_cursor_position.1,
                    ));
                    content.perform(Action::Edit(Edit::Backspace));
                    paste_selection(action, content);
                }
                Edit::Paste(text) => {
                    //TODO: Fix for paste and selection, consider selection direction
                    content.perform(Action::Cursor(
                        action.post_cursor_position.0,
                        action.post_cursor_position.1,
                    ));
                    content.perform(Action::SelectTo(
                        action.pre_cursor_position.0,
                        action.pre_cursor_position.1,
                    ));
                    content.perform(Action::Edit(Edit::Delete));
                    paste_selection(action, content);
                    content.perform(Action::Cursor(
                        action.pre_cursor_position.0,
                        action.pre_cursor_position.1,
                    ));
                    break;
                }
                Edit::Enter => {
                    content.perform(Action::Cursor(
                        action.post_cursor_position.0,
                        action.post_cursor_position.1,
                    ));
                    content.perform(Action::Edit(Edit::Backspace));
                    paste_selection(action, content);
                    content.perform(Action::Cursor(
                        action.pre_cursor_position.0,
                        action.pre_cursor_position.1,
                    ));
                    break;
                }
                edit @ (Edit::Backspace | Edit::Delete) => {
                    content.perform(Action::Cursor(
                        action.post_cursor_position.0,
                        action.post_cursor_position.1,
                    ));
                    paste_selection(action, content);
                    if action.pre_selection.is_some() {
                        content.perform(Action::Cursor(
                            action.pre_cursor_position.0,
                            action.pre_cursor_position.1,
                        ));
                        break;
                    }
                    let char = match edit {
                        Edit::Backspace => action.char_at_cursor,
                        Edit::Delete => action.char_after_cursor,
                        _ => None,
                    };
                    if let Some(char) = char {
                        content.perform(Action::Edit(Edit::Insert(char)));
                        content.perform(Action::Cursor(
                            action.pre_cursor_position.0,
                            action.pre_cursor_position.1,
                        ));
                    }
                    break;
                }
            }
        }
    }

    pub fn redo(&mut self, content: &mut impl Editor) {
        if self.current_index == self.stack.len() {
            return;
        }
        let actions = &self.stack[self.current_index..];
        let cursor_position = content.cursor_position();

        let mut insert = false;

        for (index, action) in actions.iter().enumerate() {
            if insert && !matches!(action.edit, Edit::Insert(_)) {
                break;
            }
            self.current_index += 1;
            content.perform(Action::Cursor(
                action.pre_cursor_position.0,
                action.pre_cursor_position.1,
            ));

            match &action.edit {
                Edit::Insert(char) => {
                    insert = true;
                    content.perform(Action::Edit(action.edit.clone()));
                }
                Edit::Paste(_) => {}
                Edit::Enter => {}
                Edit::Backspace => {}
                Edit::Delete => {}
            }
        }
    }
}
