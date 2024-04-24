#![allow(unused_variables, dead_code)]

use std::sync::Arc;

use iced_core::text::editor::{Action, Edit};
use iced_core::text::Editor;

pub(crate) struct EditorAction {
    pub pre_cursor_position: (usize, usize),
    pub post_cursor_position: (usize, usize),
    pub pre_selection: Option<Arc<String>>,
    pub post_selection: Option<Arc<String>>,
    pub char_at_cursor: Option<char>,
    pub char_after_cursor: Option<char>,
    pub action: Action,
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
            if insert && !matches!(action.action, Action::Edit(Edit::Insert(_))) {
                break;
            }
            self.current_index = index;

            match &action.action {
                Action::Edit(edit) => match edit {
                    Edit::Insert(_) => {
                        insert = true;
                        paste_selection(action, content);
                        content.perform(Action::Edit(Edit::Backspace));
                    }
                    Edit::Paste(text) => {
                        paste_selection(action, content);
                        break;
                    }
                    Edit::Enter => {
                        content.perform(Action::Edit(Edit::Backspace));
                        paste_selection(action, content);
                        break;
                    }
                    Edit::Backspace => {
                        paste_selection(action, content);
                        if action.pre_selection.is_some() {
                            break;
                        }
                        if let Some(char_at_cursor) = action.char_at_cursor {
                            content.perform(Action::Edit(Edit::Insert(char_at_cursor)));
                        }
                        break;
                    }
                    Edit::Delete => {
                        paste_selection(action, content);
                        if let Some(char_after_cursor) = action.char_after_cursor {
                            content.perform(Action::Edit(Edit::Insert(char_after_cursor)));
                        }
                        content.perform(Action::Cursor(
                            action.pre_cursor_position.0,
                            action.pre_cursor_position.1,
                        ));
                        break;
                    }
                },
                _ => {}
            }
        }
    }
}
