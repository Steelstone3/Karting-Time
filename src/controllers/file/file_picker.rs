use rfd::FileDialog;

pub fn select_file_to_load() -> String {
    let file = FileDialog::new().add_filter("toml", &["toml"]).pick_file();

    match file {
        Some(path_buf) => match path_buf.into_os_string().into_string() {
            Ok(file_path) => file_path.to_string(),
            Err(_) => "".to_string(),
        },
        None => "".to_string(),
    }
}

pub fn select_files_to_load() -> Vec<String> {
    let mut valid_file_paths: Vec<String> = vec![];

    let files = FileDialog::new().add_filter("toml", &["toml"]).pick_files();

    if let Some(path_bufs) = files {
        for path in path_bufs {
            if let Ok(file_path) = path.into_os_string().into_string() {
                valid_file_paths.push(file_path)
            }
        }
    };

    valid_file_paths
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

#[cfg(test)]
mod file_picker_should {
    
}