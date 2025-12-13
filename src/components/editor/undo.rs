use std::sync::Arc;

use iced::widget::text_editor::Content;
use iced_core::text::editor::{Action, Cursor, Edit, Line, Motion, Position};

#[derive(Debug, Clone, Default)]
pub struct TextContent(Content, UndoStack);

#[derive(Debug, Clone)]
pub enum ContentAction {
    Action(Action),
    Undo,
    Redo,
    Delete(Motion),
    Replace(String),
}

impl ContentAction {
    /// Returns `true` if the [`ContentAction`] is an edit.
    pub fn is_edit(&self) -> bool {
        matches!(
            self,
            Self::Action(Action::Edit(_)) | Self::Delete(_) | Self::Undo | Self::Redo
        )
    }
}

fn track_action(content: &mut Content, undo_stack: &mut UndoStack, edit: Edit) {
    let cursor = content.cursor();
    let pre_selection = content.selection().map(Arc::new);

    let (mut at, mut after) = content
        .line(cursor.position.line)
        .map(|line| {
            let mut chars = line.text.chars();
            (
                chars.nth(cursor.position.column.saturating_sub(1)),
                chars.next(),
            )
        })
        .unwrap_or((None, None));
    if cursor.position.column == 0 {
        at = (cursor.position.line > 0).then_some('\n');
    }
    if after.is_none() {
        after = content.line(cursor.position.line + 1).map(|_| '\n');
    }

    content.perform(Action::Edit(edit.clone()));

    undo_stack.push(EditorAction {
        edit,
        pre_selection_text: pre_selection,
        pre_cursor: cursor,
        post_cursor: cursor,
        char_at_cursor: at,
        char_after_cursor: after,
    });
}

fn delete_action(mov: Motion, content: &mut Content, undo_stack: &mut UndoStack) {
    let cursor = content.cursor();
    if cursor.selection.is_some() {
        track_action(content, undo_stack, Edit::Delete);
        return;
    };

    content.perform(Action::Move(mov));
    let post_cursor = content.cursor();
    content.move_to(Cursor {
        position: post_cursor.position,
        selection: Some(cursor.position),
    });
    track_action(content, undo_stack, Edit::Delete);
}

fn replace_action(text: String, content: &mut Content, undo_stack: &mut UndoStack) {
    content.perform(Action::SelectAll);
    track_action(content, undo_stack, Edit::Paste(Arc::new(text)));
}

impl TextContent {
    /// Creates an empty [`Content`].
    pub fn new() -> Self {
        Self::with_text("")
    }

    /// Creates a [`Content`] with the given text.
    pub fn with_text(text: &str) -> Self {
        Self(Content::with_text(text), UndoStack::new())
    }

    /// Performs an [`ContentAction`] on the [`Content`].
    pub fn perform(&mut self, action: ContentAction) {
        // let internal = self.0.get_mut();

        match action {
            ContentAction::Action(Action::Edit(edit)) => {
                track_action(&mut self.0, &mut self.1, edit)
            }
            ContentAction::Action(action) => self.0.perform(action),
            ContentAction::Delete(motion) => delete_action(motion, &mut self.0, &mut self.1),
            ContentAction::Undo => self.1.undo(&mut self.0),
            ContentAction::Redo => self.1.redo(&mut self.0),
            ContentAction::Replace(text) => replace_action(text, &mut self.0, &mut self.1),
        }
    }

    /// Returns the text of the [`Content`].
    pub fn text(&self) -> String {
        self.0.text()
    }

    /// Returns the selected text of the [`Content`].
    pub fn selection(&self) -> Option<String> {
        self.0.selection()
    }

    /// Returns whether or not the the [`Content`] is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn internal(&self) -> &Content {
        &self.0
    }

    pub fn line_count(&self) -> usize {
        self.0.line_count()
    }

    pub fn line(&self, index: usize) -> Option<Line<'_>> {
        self.0.line(index)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct EditorAction {
    pub pre_cursor: Cursor,
    pub post_cursor: Cursor,
    pub char_at_cursor: Option<char>,
    pub char_after_cursor: Option<char>,
    pub pre_selection_text: Option<Arc<String>>,
    pub edit: Edit,
}

#[derive(Debug, Clone, Default)]
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

    fn push(&mut self, action: EditorAction) {
        self.stack.truncate(self.current_index);
        self.stack.push(action);
        self.current_index += 1;
    }

    fn undo(&mut self, content: &mut Content) {
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
                    let position = if let Some(selection) = action.pre_cursor.selection {
                        Position {
                            column: selection.column.min(action.pre_cursor.position.column),
                            line: selection.line.min(action.pre_cursor.position.line),
                        }
                    } else {
                        action.pre_cursor.position
                    };
                    content.move_to(Cursor {
                        position,
                        selection: Some(action.post_cursor.position),
                    });
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

    pub fn redo(&mut self, content: &mut Content) {
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

fn paste_prev_selection(action: &EditorAction, content: &mut Content) -> bool {
    if let Some(selection) = &action.pre_selection_text {
        content.perform(Action::Edit(Edit::Paste(Arc::clone(selection))));
        content.move_to(action.pre_cursor);
        true
    } else {
        false
    }
}
