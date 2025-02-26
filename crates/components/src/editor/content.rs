use std::borrow::Cow;
use std::cell::RefCell;
use std::fmt;
use std::sync::Arc;

use iced_core::text::editor::{Edit, Editor as _, Line, LineEnding, Motion};
use iced_core::text::{self};
pub use text::editor::Action;

use crate::editor::undo_stack::{EditorAction, UndoStack};

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

fn track_action<R: text::Renderer>(internal: &mut Internal<R>, edit: Edit) {
    let editor = &mut internal.editor;

    let (line, col) = editor.cursor_position();
    let pre_selection = editor.selection().map(Arc::new);
    let pre_selection_c = editor.selection_cursor();

    let (mut at, mut after) = editor
        .line(line)
        .map(|line| {
            let mut chars = line.text.chars();
            (chars.nth(col.saturating_sub(1)), chars.next())
        })
        .unwrap_or((None, None));
    if col == 0 {
        at = (line > 0).then_some('\n');
    }
    if after.is_none() {
        after = editor.line(line + 1).map(|_| '\n');
    }

    editor.perform(Action::Edit(edit.clone()));

    internal.undo_stack.push(EditorAction {
        edit,
        pre_selection_text: pre_selection,
        pre_cursor: (line, col),
        post_cursor: editor.cursor_position(),
        char_at_cursor: at,
        char_after_cursor: after,
        pre_selection: pre_selection_c,
        post_selection: editor.selection_cursor(),
    });
}

fn delete_action<R: text::Renderer>(mov: Motion, internal: &mut Internal<R>) {
    let selection = internal.editor.selection_cursor();
    if selection.is_some() {
        track_action(internal, Edit::Delete);
        return;
    };

    let editor = &mut internal.editor;
    let cursor = editor.cursor_position();
    editor.perform(Action::Move(mov));
    editor.perform(Action::SelectTo(cursor.0, cursor.1));
    track_action(internal, Edit::Delete);
}

/// An action that can be performed on the [`Content`] of a [`super::TextEditor`].
#[derive(Debug, Clone)]
pub enum ContentAction {
    Action(Action),
    Undo,
    Redo,
    Delete(Motion),
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
            ContentAction::Action(Action::Edit(edit)) => track_action(internal, edit),
            ContentAction::Action(action) => internal.editor.perform(action),
            ContentAction::Delete(motion) => delete_action(motion, internal),
            ContentAction::Undo => internal.undo_stack.undo(&mut internal.editor),
            ContentAction::Redo => internal.undo_stack.redo(&mut internal.editor),
        }
        internal.is_dirty = true;
    }

    /// Returns the amount of lines of the [`Content`].
    pub fn line_count(&self) -> usize {
        self.0.borrow().editor.line_count()
    }

    /// Returns the text of the line at the given index, if it exists.
    pub fn line(&self, index: usize) -> Option<Line<'_>> {
        let internal = self.0.borrow();
        let line = internal.editor.line(index)?;

        Some(Line {
            text: Cow::Owned(line.text.into_owned()),
            ending: line.ending,
        })
    }

    /// Returns an iterator of the text of the lines in the [`Content`].
    pub fn lines(&self) -> impl Iterator<Item = Line<'_>> {
        (0..)
            .map(|i| self.line(i))
            .take_while(Option::is_some)
            .flatten()
    }

    /// Returns the text of the [`Content`].
    pub fn text(&self) -> String {
        let mut contents = String::new();
        let mut lines = self.lines().peekable();

        while let Some(line) = lines.next() {
            contents.push_str(&line.text);

            if lines.peek().is_some() {
                contents.push_str(if line.ending == LineEnding::None {
                    LineEnding::default().as_str()
                } else {
                    line.ending.as_str()
                });
            }
        }

        contents
    }

    /// Returns the kind of [`LineEnding`] used for separating lines in the [`Content`].
    pub fn line_ending(&self) -> Option<LineEnding> {
        Some(self.line(0)?.ending)
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
