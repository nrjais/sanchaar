use std::{convert::identity, sync::Arc};

use iced::futures::FutureExt;
use iced::Task;
use rfd::{AsyncFileDialog, FileHandle};

pub fn open_folder_dialog(title: &str) -> Task<Option<Arc<FileHandle>>> {
    Task::perform(
        AsyncFileDialog::new()
            .set_title(title)
            .set_can_create_directories(true)
            .pick_folder()
            .map(|res| res.map(Arc::new)),
        identity,
    )
}

pub fn open_file_dialog(title: &str) -> Task<Option<FileHandle>> {
    Task::perform(
        AsyncFileDialog::new()
            .set_title(title)
            .set_can_create_directories(true)
            .pick_file(),
        identity,
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
