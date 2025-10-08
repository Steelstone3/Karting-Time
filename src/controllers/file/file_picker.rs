use iced::Task;
use rfd::{AsyncFileDialog, FileDialog};

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

pub fn save_file_location() -> String {
    let file = FileDialog::new().add_filter("toml", &["toml"]).save_file();

    match file {
        Some(path_buf) => match path_buf.into_os_string().into_string() {
            Ok(file_path) => file_path.to_string(),
            Err(_) => "".to_string(),
        },
        None => "".to_string(),
    }
}

pub fn save_folder_location() -> String {
    let file = FileDialog::new().pick_folder();

    match file {
        Some(path_buf) => match path_buf.into_os_string().into_string() {
            Ok(file_path) => file_path.to_string(),
            Err(_) => "".to_string(),
        },
        None => "".to_string(),
    }
}
