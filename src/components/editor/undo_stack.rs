use std::sync::Arc;

use iced_core::text::Editor;
use iced_core::text::editor::{Action, Cursor, Edit, Position};

pub(crate) struct EditorAction {
    pub pre_cursor: Cursor,
    pub post_cursor: Cursor,
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

            content.move_to(action.post_cursor);
            match &action.edit {
                Edit::Insert(_) => {
                    insert = true;
                    content.perform(Action::Edit(Edit::Backspace));
                    paste_prev_selection(action, content);
                    continue;
                }
                Edit::Paste(_) => {
                    let cursor = if let Some(selection) = action.pre_cursor.selection {
                        min_position(selection, action.pre_cursor.position)
                    } else {
                        action.pre_cursor.position
                    };
                    let cursor = Cursor {
                        position: cursor,
                        selection: Some(action.post_cursor.position),
                    };
                    content.move_to(cursor);
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
            content.move_to(action.pre_cursor);
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

            content.move_to(action.pre_cursor);
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

fn min_position(a: Position, b: Position) -> Position {
    if a.line < b.line {
        a
    } else if a.line > b.line {
        b
    } else {
        Position {
            line: a.line,
            column: a.column.min(b.column),
        }
    }
}

fn paste_prev_selection(action: &EditorAction, content: &mut impl Editor) -> bool {
    if let Some(selection) = &action.pre_selection_text {
        content.perform(Action::Edit(Edit::Paste(Arc::clone(selection))));
        content.move_to(action.pre_cursor);
        true
    } else {
        false
    }
}
