use std::{fs, io};
use dialoguer::{Select, theme::ColorfulTheme};

pub fn select_file() -> io::Result<String> {
    let files = fs::read_dir(".")?
        .filter_map(|entry| entry.ok()
            .and_then(|e| if e.file_type().ok()?.is_file() {
            e.file_name().into_string().ok()
        } else { None }))
        .collect::<Vec<_>>();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a file to read")
        .default(0)
        .items(&files)
        .interact_opt();

    match selection {
        Ok(Some(index)) => Ok(files[index].clone()),
        Ok(None) => Err(io::Error::new(io::ErrorKind::NotFound, "No file selected")),
        Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Failed to interact with user")),
    }
}
