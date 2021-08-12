use super::errors::{invalid_input_error, std_error};
use crate::core::template::TempPath;
use std::io::Error;
use std::path::{Path, PathBuf};

pub fn pathbuf_to_string(path: PathBuf) -> String {
    path.as_os_str().to_str().unwrap().to_string()
}

pub fn valid_directory_path(directory: &str) -> Result<(), Error> {
    if directory.contains(r"\") || directory.ends_with("/") {
        return Err(invalid_input_error("Invalid directory path."));
    }
    Ok(())
}

pub fn remove_dir_prefix(path: TempPath, directory: &str) -> Result<TempPath, Error> {
    let prefix = format!("{}/", directory);
    let new_path = std_error(path.buf.strip_prefix(prefix))?.to_path_buf();
    Ok(TempPath {
        buf: new_path,
        path_type: path.path_type,
    })
}

pub fn format_path_namespace(path: PathBuf) -> PathBuf {
    Path::new(&pathbuf_to_string(path).replace(r"\", "/")).to_path_buf()
}
