use iced::Task;
use rfd::AsyncFileDialog;

pub fn select_file_to_load() -> Task<Option<String>> {
    Task::future(async {
        let file = AsyncFileDialog::new()
            .add_filter("toml", &["toml"])
            .pick_file()
            .await;

        file.and_then(|handle| handle.path().to_str().map(|s| s.to_string()))
    })
}

pub fn select_files_to_load() -> Task<Option<Vec<String>>> {
    Task::future(async {
        AsyncFileDialog::new()
            .add_filter("toml", &["toml"])
            .pick_files()
            .await
            .map(|paths| {
                paths
                    .into_iter()
                    .filter_map(|p| p.path().to_str().map(|s| s.to_string()))
                    .collect::<Vec<_>>()
            })
    })
}

pub fn save_file_location() -> Task<Option<String>> {
    Task::future(async {
        let file = AsyncFileDialog::new()
            .add_filter("toml", &["toml"])
            .save_file()
            .await;

        file.and_then(|handle| handle.path().to_str().map(|s| s.to_string()))
    })
}

pub fn save_folder_location() -> Task<Option<String>> {
    Task::future(async {
        let folder = AsyncFileDialog::new().pick_folder().await;

        folder.and_then(|handle| handle.path().to_str().map(|s| s.to_string()))
    })
}
