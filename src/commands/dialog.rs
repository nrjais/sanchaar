use std::sync::Arc;

use iced::advanced::graphics::futures::MaybeSend;
use iced::futures::FutureExt;
use iced::Command;
use rfd::{AsyncFileDialog, FileHandle};

pub fn open_folder_dialog<Message>(
    title: &str,
    on_done: impl FnOnce(Option<Arc<FileHandle>>) -> Message + MaybeSend + 'static,
) -> Command<Message> {
    Command::perform(
        AsyncFileDialog::new()
            .set_title(title)
            .pick_folder()
            .map(|res| res.map(Arc::new)),
        on_done,
    )
}
