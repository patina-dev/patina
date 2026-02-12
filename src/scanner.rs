use std::path::PathBuf;

const SUPPORTED_EXTENSIONS: &[&str] = &["js", "jsx", "ts", "tsx"];

pub fn scan_files(path: &PathBuf) -> Vec<PathBuf> {
    let _ = (path, SUPPORTED_EXTENSIONS);
    Vec::new()
}
