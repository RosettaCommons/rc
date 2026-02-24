use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ops::ShlAssign;

use anyhow::Result;
use camino::Utf8Path;
use yansi::Paint;

use crate::util::PaintExt;

const DIR_SIGNATURE_FILE: &str = ".signature.rc";

#[derive(Debug)]
pub enum DirState {
    UpToDate,
    Built,
    Rebuilt,
}

pub fn ensure_dir_signature<F, S>(
    dir: &Utf8Path,
    signature_parts: &[S],
    builder: F,
) -> Result<DirState>
where
    S: AsRef<[u8]>,
    F: FnOnce(&Utf8Path) -> Result<()>,
{
    let signature = hash_slices(signature_parts);
    let signature_path = dir.join(DIR_SIGNATURE_FILE);

    let state = if dir.exists() {
        if signature_matches(&signature_path, &signature)? {
            return Ok(DirState::UpToDate);
        }
        println!(
            "Directory {:?} is outdated, removing old content...",
            dir.orange()
        );
        std::fs::remove_dir_all(dir)?;
        DirState::Rebuilt
    } else {
        DirState::Built
    };
    println!("Building directory {:?}...", dir.green());

    std::fs::create_dir_all(dir)?;
    builder(dir)?;
    write_signature(&signature_path, &signature)?;

    Ok(state)
}

fn hash_slices<S: AsRef<[u8]>>(data: &[S]) -> String {
    let mut hasher = blake3::Hasher::new();
    for s in data {
        hasher.update(s.as_ref());
    }

    hasher.finalize().to_string()
}

fn signature_matches(signature_path: &Utf8Path, signature: &str) -> Result<bool> {
    if !signature_path.exists() {
        return Ok(false);
    }
    let content = std::fs::read_to_string(signature_path)?;
    Ok(content == signature)
}

fn write_signature(signature_path: &Utf8Path, signature: &str) -> Result<()> {
    std::fs::write(signature_path, signature)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::TempDir;
    use camino::Utf8PathBuf;
    use std::fs;
    use std::io::Sink;

    fn setup_test_dir() -> TempDir {
        TempDir::new().unwrap()
    }

    fn to_utf8_path(temp: &TempDir, sub: &str) -> Utf8PathBuf {
        let path = temp.path().join(sub);
        Utf8PathBuf::from_path_buf(path).expect("path is not valid UTF-8")
    }

    #[test]
    fn test_hash_slices_empty_signature() {
        let empty: Vec<&str> = vec![];
        let hash1 = hash_slices(&empty);
        let hash2 = hash_slices(&empty);

        assert_eq!(hash1, hash2);
        assert!(!hash1.is_empty());
    }

    #[test]
    fn test_hash_slices_produces_consistent_hash() {
        let data1 = vec!["hello", "world"];
        let data2 = vec!["hello", "world"];

        let hash1 = hash_slices(&data1);
        let hash2 = hash_slices(&data2);

        assert_eq!(hash1, hash2);
        assert!(!hash1.is_empty());
    }

    #[test]
    fn test_hash_slices_different_data_different_hash() {
        let data1 = vec!["hello", "world"];
        let data2 = vec!["hello", "rust"];

        let hash1 = hash_slices(&data1);
        let hash2 = hash_slices(&data2);

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_hash_slices_order_matters() {
        let data1 = vec!["hello", "world"];
        let data2 = vec!["world", "hello"];

        let hash1 = hash_slices(&data1);
        let hash2 = hash_slices(&data2);

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_signature_matches_nonexistent_file() {
        let temp = setup_test_dir();
        let sig_path = to_utf8_path(&temp, "nonexistent.sig");

        let result = signature_matches(&sig_path, "any_signature").unwrap();
        assert!(!result);
    }

    #[test]
    fn test_signature_matches_correct_signature() {
        let temp = setup_test_dir();
        let sig_path = to_utf8_path(&temp, "test.sig");
        let signature = "test_signature_12345";

        fs::write(&sig_path, signature).unwrap();

        let result = signature_matches(&sig_path, signature).unwrap();
        assert!(result);
    }

    #[test]
    fn test_signature_matches_detects_signature_mismatch() {
        let temp = setup_test_dir();
        let sig_path = to_utf8_path(&temp, "test.sig");

        fs::write(&sig_path, "old_signature").unwrap();

        let result = signature_matches(&sig_path, "new_signature").unwrap();
        assert!(!result);
    }

    #[test]
    fn test_write_signature() {
        let temp = setup_test_dir();
        let sig_path = to_utf8_path(&temp, "test.sig");
        let signature = "test_signature";

        write_signature(&sig_path, signature).unwrap();

        let content = fs::read_to_string(&sig_path).unwrap();
        assert_eq!(content, signature);
    }

    #[test]
    fn test_ensure_dir_signature_builds_new_dir() {
        let temp = setup_test_dir();
        let target = to_utf8_path(&temp, "new_dir");
        let mut builder_called = false;

        let result = ensure_dir_signature(&target, &["signature", "data"], |path| {
            builder_called = true;
            fs::write(path.join("test.txt"), "content")?;
            Ok(())
        })
        .unwrap();

        assert!(builder_called);
        assert!(target.exists());
        assert!(target.join("test.txt").exists());
        assert!(target.join(DIR_SIGNATURE_FILE).exists());
        assert!(matches!(result, DirState::Built));
    }

    #[test]
    fn test_ensure_dir_signature_up_to_date() {
        let temp = setup_test_dir();
        let target = to_utf8_path(&temp, "existing_dir");

        let signature = ["signature", "data"];

        // First build
        ensure_dir_signature(&target, &signature, |path| {
            fs::write(path.join("test.txt"), "content")?;
            Ok(())
        })
        .unwrap();

        // Second call with same signature
        let mut builder_called = false;
        let result = ensure_dir_signature(&target, &signature, |_path| {
            builder_called = true;
            Ok(())
        })
        .unwrap();

        assert!(
            !builder_called,
            "Builder should not be called when up-to-date"
        );
        assert!(matches!(result, DirState::UpToDate));
    }

    #[test]
    fn test_ensure_dir_signature_rebuilds_on_signature_change() {
        let temp = setup_test_dir();
        let target = to_utf8_path(&temp, "rebuild_dir");

        // Create initial structure with nested directories
        ensure_dir_signature(&target, &["old", "signature"], |path| {
            fs::create_dir(path.join("subdir"))?;
            fs::write(path.join("subdir/nested.txt"), "nested content")?;
            fs::write(path.join("old.txt"), "old content")?;
            Ok(())
        })
        .unwrap();

        assert!(target.join("old.txt").exists());
        assert!(target.join("subdir").exists());

        // Rebuild with different signature
        let mut builder_called = false;
        let result = ensure_dir_signature(&target, &["new", "signature"], |path| {
            builder_called = true;
            fs::write(path.join("new.txt"), "new content")?;
            Ok(())
        })
        .unwrap();

        assert!(builder_called);
        assert!(matches!(result, DirState::Rebuilt));
        assert!(
            !target.join("old.txt").exists(),
            "Old files should be removed"
        );
        assert!(
            !target.join("subdir").exists(),
            "Old subdirectories should be removed"
        );
        assert!(target.join("new.txt").exists());
        assert!(target.join(DIR_SIGNATURE_FILE).exists());
    }

    #[test]
    fn test_ensure_dir_signature_builder_error_propagates() {
        let temp = setup_test_dir();
        let target = to_utf8_path(&temp, "error_dir");

        let result = ensure_dir_signature(&target, &["sig"], |_path| {
            anyhow::bail!("Builder failed intentionally")
        });

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Builder failed intentionally")
        );
    }

    #[test]
    fn test_ensure_dir_signature_handles_bytes() {
        let temp = setup_test_dir();
        let target = to_utf8_path(&temp, "bytes_dir");

        let byte_data: Vec<Vec<u8>> = vec![vec![0, 1, 2, 3], vec![255, 254, 253]];

        let result = ensure_dir_signature(&target, &byte_data, |path| {
            fs::write(path.join("data.bin"), [0u8, 1, 2])?;
            Ok(())
        })
        .unwrap();

        assert!(matches!(result, DirState::Built));
        assert!(target.exists());
    }
}
