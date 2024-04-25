use std::cell::RefCell;
use std::fmt;
use std::sync::Arc;

use iced_core::text::editor::{Edit, Editor as _, Motion};
use iced_core::text::{self};
pub use text::editor::Action;

use crate::text_editor::undo_stack::{EditorAction, UndoStack};

/// The content of a [`super::TextEditor`].
pub struct Content<R = iced::Renderer>(pub(super) RefCell<Internal<R>>)
where
    R: text::Renderer;

pub(super) struct Internal<R>
where
    R: text::Renderer,
{
    pub(super) editor: R::Editor,
    pub(super) is_dirty: bool,
    undo_stack: UndoStack,
}

/// An action that can be performed on the [`Content`] of a [`super::TextEditor`].
#[derive(Debug, Clone)]
pub enum ContentAction {
    Action(Action),
    Undo,
    Redo,
    DeleteNextWord,
    DeletePreviousWord,
    DeleteTillLineStart,
}

impl ContentAction {
    /// Returns `true` if the [`ContentAction`] is an edit.
    pub fn is_edit(&self) -> bool {
        match self {
            Self::Action(Action::Edit(_)) => true,
            Self::Undo => true,
            _ => false,
        }
    }
}

impl<R> Content<R>
where
    R: text::Renderer,
{
    /// Creates an empty [`Content`].
    pub fn new() -> Self {
        Self::with_text("")
    }

    /// Creates a [`Content`] with the given text.
    pub fn with_text(text: &str) -> Self {
        Self(RefCell::new(Internal {
            editor: R::Editor::with_text(text),
            is_dirty: true,
            undo_stack: UndoStack::new(),
        }))
    }

    /// Performs an [`ContentAction`] on the [`Content`].
    pub fn perform(&mut self, action: ContentAction) {
        let internal = self.0.get_mut();

        match action {
            ContentAction::Action(action) => {
                Self::track_action(internal, action);
            }
            ContentAction::DeleteNextWord => {
                let cursor = internal.editor.cursor_position();
                internal.editor.perform(Action::Move(Motion::WordRight));
                internal
                    .editor
                    .perform(Action::SelectTo(cursor.0, cursor.1));
                Self::track_action(internal, Action::Edit(Edit::Delete));
            }
            ContentAction::DeletePreviousWord => {
                let cursor = internal.editor.cursor_position();
                internal.editor.perform(Action::Move(Motion::WordLeft));
                internal
                    .editor
                    .perform(Action::SelectTo(cursor.0, cursor.1));
                Self::track_action(internal, Action::Edit(Edit::Delete));
            }
            ContentAction::DeleteTillLineStart => {
                let cursor = internal.editor.cursor_position();
                internal.editor.perform(Action::Move(Motion::Home));
                internal
                    .editor
                    .perform(Action::SelectTo(cursor.0, cursor.1));
                Self::track_action(internal, Action::Edit(Edit::Delete));
            }
            ContentAction::Undo => {
                internal.undo_stack.undo(&mut internal.editor);
            }
            ContentAction::Redo => {
                internal.undo_stack.redo(&mut internal.editor);
            }
        }
        internal.is_dirty = true;
    }

    fn track_action(internal: &mut Internal<R>, action: Action) {
        let cursor_position = internal.editor.cursor_position();
        let pre_selection = internal.editor.selection().map(Arc::new);
        let (at, after) = internal
            .editor
            .line(cursor_position.0)
            .map(|line| {
                let mut chars = line.chars();
                (chars.nth(cursor_position.1.saturating_sub(1)), chars.next())
            })
            .unwrap_or((None, None));

        internal.editor.perform(action.clone());

        internal.undo_stack.push(EditorAction {
            action,
            pre_selection,
            post_selection: internal.editor.selection().map(Arc::new),
            pre_cursor_position: cursor_position,
            post_cursor_position: internal.editor.cursor_position(),
            char_at_cursor: at,
            char_after_cursor: after,
        });
    }

    /// Returns the amount of lines of the [`Content`].
    pub fn line_count(&self) -> usize {
        self.0.borrow().editor.line_count()
    }

    /// Returns the text of the line at the given index, if it exists.
    pub fn line(&self, index: usize) -> Option<impl std::ops::Deref<Target = str> + '_> {
        std::cell::Ref::filter_map(self.0.borrow(), |internal| internal.editor.line(index)).ok()
    }

    /// Returns an iterator of the text of the lines in the [`Content`].
    pub fn lines(&self) -> impl Iterator<Item = impl std::ops::Deref<Target = str> + '_> {
        struct Lines<'a, Renderer: text::Renderer> {
            internal: std::cell::Ref<'a, Internal<Renderer>>,
            current: usize,
        }

        impl<'a, Renderer: text::Renderer> Iterator for Lines<'a, Renderer> {
            type Item = std::cell::Ref<'a, str>;

            fn next(&mut self) -> Option<Self::Item> {
                let line =
                    std::cell::Ref::filter_map(std::cell::Ref::clone(&self.internal), |internal| {
                        internal.editor.line(self.current)
                    })
                    .ok()?;

                self.current += 1;

                Some(line)
            }
        }

        Lines {
            internal: self.0.borrow(),
            current: 0,
        }
    }

    /// Returns the text of the [`Content`].
    ///
    /// Lines are joined with `'\n'`.
    pub fn text(&self) -> String {
        let mut text = self
            .lines()
            .enumerate()
            .fold(String::new(), |mut contents, (i, line)| {
                if i > 0 {
                    contents.push('\n');
                }

                contents.push_str(&line);

                contents
            });

        if !text.ends_with('\n') {
            text.push('\n');
        }

        text
    }

    /// Returns the selected text of the [`Content`].
    pub fn selection(&self) -> Option<String> {
        self.0.borrow().editor.selection()
    }

    /// Returns the current cursor position of the [`Content`].
    pub fn cursor_position(&self) -> (usize, usize) {
        self.0.borrow().editor.cursor_position()
    }
}

impl<Renderer> Default for Content<Renderer>
where
    Renderer: text::Renderer,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Renderer> fmt::Debug for Content<Renderer>
where
    Renderer: text::Renderer,
    Renderer::Editor: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let internal = self.0.borrow();

        f.debug_struct("Content")
            .field("editor", &internal.editor)
            .field("is_dirty", &internal.is_dirty)
            .finish()
    }
}
