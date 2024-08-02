use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

/// Attempts to list the directories in a given path.
pub fn list_directories(dir: &Path) -> Vec<PathBuf> {
    let entries = fs::read_dir(dir).unwrap();
    entries
        .into_iter()
        .map(|e| e.expect("Failed path read.").path())
        .filter(|x| x.is_dir())
        .collect()
}

/// List files in a directory.
pub fn list_files(dir: &Path) -> Vec<PathBuf> {
    let entries = fs::read_dir(dir).unwrap();
    entries
        .into_iter()
        .map(|e| e.expect("Failed path read.").path())
        .filter(|x| x.is_file())
        .collect()
}

/// Construct a hash map that containts logbook names as keys and their paths as values.
pub fn logbook_hash(logbook_dir: &Path) -> HashMap<String, PathBuf> {
    let logbook_dirs = list_directories(logbook_dir);
    let mut hm = HashMap::<String, PathBuf>::new();
    for dir in logbook_dirs.into_iter() {
        let key = dir
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();
        hm.insert(key, dir);
    }
    hm
}

/// Once you have selected a logbook this function lists all log files that need to be parsed.
pub fn list_log_files(logbook_dir: &Path) -> Vec<PathBuf> {
    // first we have the years that the logs were created.
    let years = list_directories(logbook_dir);
    let mut all_files: Vec<PathBuf> = Vec::with_capacity(20);
    for year in years.iter() {
        all_files.append(&mut list_files(year))
    }
    all_files
        .into_iter()
        .filter(|f| f.extension().unwrap() == "log")
        .collect()
}
