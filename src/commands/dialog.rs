use std::{convert::identity, sync::Arc};

use iced::futures::FutureExt;
use iced::Task;
use rfd::{AsyncFileDialog, FileHandle};

pub fn open_file_dialog(title: &str) -> Task<Option<Arc<FileHandle>>> {
    let title = title.to_string();
    Task::perform(
        async move {
            AsyncFileDialog::new()
                .set_title(&title)
                .pick_file()
                .await
                .map(Arc::new)
        },
        |handle| handle,
    )
}

pub fn create_file_dialog(title: &str) -> Task<Option<Arc<FileHandle>>> {
    Task::perform(
        AsyncFileDialog::new()
            .set_title(title)
            .set_can_create_directories(true)
            .save_file()
            .map(|res| res.map(Arc::new)),
        identity,
    )
}
