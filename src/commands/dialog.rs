use std::sync::Arc;

use iced::advanced::graphics::futures::MaybeSend;
use iced::futures::FutureExt;
use iced::Task;
use rfd::{AsyncFileDialog, FileHandle};

pub fn open_folder_dialog<Message: MaybeSend + 'static>(
    title: &str,
    on_done: impl Fn(Option<Arc<FileHandle>>) -> Message + MaybeSend + 'static,
) -> Task<Message> {
    Task::perform(
        AsyncFileDialog::new()
            .set_title(title)
            .pick_folder()
            .map(|res| res.map(Arc::new)),
        on_done,
    )
}

pub fn open_file_dialog<Message: MaybeSend + 'static>(
    title: &str,
    on_done: impl Fn(Option<Arc<FileHandle>>) -> Message + MaybeSend + 'static,
) -> Task<Message> {
    Task::perform(
        AsyncFileDialog::new()
            .set_title(title)
            .pick_file()
            .map(|res| res.map(Arc::new)),
        on_done,
    )
}
