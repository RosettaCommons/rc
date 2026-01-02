use std::path::PathBuf;

use home::home_dir;

pub fn cache_root() -> PathBuf {
    home_dir().unwrap().join(".cache/rosettacommons/rc")
}
