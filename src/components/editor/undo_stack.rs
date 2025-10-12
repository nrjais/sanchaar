use std::sync::Arc;

use iced_core::text::Editor;
use iced_core::text::editor::{Action, Edit};

type Cursor = (usize, usize);

pub(crate) struct EditorAction {
    pub pre_cursor: Cursor,
    pub post_cursor: Cursor,
    pub pre_selection: Option<Cursor>,
    pub post_selection: Option<Cursor>,
    pub char_at_cursor: Option<char>,
    pub char_after_cursor: Option<char>,
    pub pre_selection_text: Option<Arc<String>>,
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

        let mut insert = false;
        for (index, action) in actions {
            if insert && !matches!(action.edit, Edit::Insert(_)) {
                break;
            }
            self.current_index = index;

            restore_selection(content, action.post_cursor, action.post_selection);
            match &action.edit {
                Edit::Insert(_) => {
                    insert = true;
                    content.perform(Action::Edit(Edit::Backspace));
                    paste_prev_selection(action, content);
                    continue;
                }
                Edit::Paste(_) => {
                    let cursor = if let Some(selection) = action.pre_selection {
                        selection.min(action.pre_cursor)
                    } else {
                        action.pre_cursor
                    };
                    content.perform(Action::Cursor(cursor.0, cursor.1));
                    content.perform(Action::SelectTo(action.post_cursor.0, action.post_cursor.1));
                    content.perform(Action::Edit(Edit::Delete));

                    paste_prev_selection(action, content);
                }
                Edit::Enter => {
                    content.perform(Action::Edit(Edit::Backspace));
                    paste_prev_selection(action, content);
                }
                Edit::Indent => {}
                Edit::Unindent => {}
                edit @ (Edit::Backspace | Edit::Delete) => {
                    let char = match edit {
                        Edit::Backspace => action.char_at_cursor,
                        Edit::Delete => action.char_after_cursor,
                        _ => None,
                    };
                    if !paste_prev_selection(action, content)
                        && let Some(char) = char
                    {
                        content.perform(Action::Edit(Edit::Insert(char)));
                    }
                }
            }
            restore_selection(content, action.pre_cursor, action.pre_selection);
            break;
        }
    }

    pub fn redo(&mut self, content: &mut impl Editor) {
        if self.current_index == self.stack.len() {
            return;
        }
        let actions = &self.stack[self.current_index..];
        let mut insert = false;

        for action in actions.iter() {
            if insert && !matches!(action.edit, Edit::Insert(_)) {
                break;
            }
            self.current_index += 1;

            restore_selection(content, action.pre_cursor, action.pre_selection);
            match &action.edit {
                Edit::Insert(_) => {
                    insert = true;
                    content.perform(Action::Edit(action.edit.clone()));
                    continue;
                }
                Edit::Paste(_) | Edit::Enter | Edit::Delete | Edit::Backspace => {
                    content.perform(Action::Edit(action.edit.clone()));
                }
                Edit::Indent => {}
                Edit::Unindent => {}
            }
            break;
        }
    }
}

fn restore_selection(content: &mut impl Editor, cursor: Cursor, selection: Option<Cursor>) {
    if let Some((line, col)) = selection {
        content.perform(Action::Cursor(line, col));
        content.perform(Action::SelectTo(cursor.0, cursor.1));
    } else {
        content.perform(Action::Cursor(cursor.0, cursor.1));
    }
}

fn paste_prev_selection(action: &EditorAction, content: &mut impl Editor) -> bool {
    if let Some(selection) = &action.pre_selection_text {
        content.perform(Action::Edit(Edit::Paste(Arc::clone(selection))));
        restore_selection(content, action.pre_cursor, action.pre_selection);
        true
    } else {
        false
    }
}
