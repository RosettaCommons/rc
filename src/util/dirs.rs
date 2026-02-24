use camino::Utf8PathBuf;
use home::home_dir;

pub fn cache_root() -> Utf8PathBuf {
    let path = home_dir().unwrap().join(".cache/rosettacommons/rc");
    Utf8PathBuf::from_path_buf(path).expect("path is not valid UTF-8")
}
