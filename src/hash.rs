use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::path::Path;

use color_eyre::eyre::{bail, Result};
use sha2::{Digest, Sha256};

pub fn hash_to_str<T: Hash>(t: &T) -> String {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    let bytes = s.finish();
    format!("{bytes:x}")
}

pub fn file_hash_sha256(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
}

pub fn ensure_checksum_sha256(path: &Path, checksum: &str) -> Result<()> {
    let actual = file_hash_sha256(path)?;
    if actual != checksum {
        bail!(
            "Checksum mismatch for file {:?}:\nExpected: {}\nActual: {}",
            path,
            checksum,
            actual
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;

    #[test]
    fn test_hash_to_str() {
        assert_eq!(hash_to_str(&"foo"), "3e8b8c44c3ca73b7");
    }

    #[test]
    fn test_hash_sha256() {
        let path = Path::new(".test-tool-versions");
        let hash = file_hash_sha256(path).unwrap();
        assert_snapshot!(hash);
    }
}
