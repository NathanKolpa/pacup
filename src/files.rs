use std::env::var_os;
use std::path::PathBuf;

fn get_first_valid_path<T: Iterator<Item=PathBuf>>(paths: T) -> Option<PathBuf> {
    for path in paths {
        if path.exists() {
            return Some(path);
        }
    }

    None
}

pub fn get_packagelist_file_path() -> Option<PathBuf> {
    let xdg_path = var_os("XDG_CONFIG_HOME")
        .map(|home| {
            let mut path = PathBuf::from(home);
            path.push("pacup");
            path.push("packagelist");
            path
        });

    let home_path = var_os("HOME")
        .map(|home| {
            let mut path = PathBuf::from(home);
            path.push(".packagelist");
            path
        });

    let global_path = Some(PathBuf::from("/etc/pacup/packagelist"));

    let files = [
        xdg_path,
        home_path,
        global_path
    ];

    get_first_valid_path(files.into_iter().filter_map(|f| f))
}