// SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>
//
// SPDX-License-Identifier: GPL-3.0-only

use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

const SUPPORTED_EXTENSIONS: &[&str] = &["js", "jsx", "ts", "tsx"];

pub fn scan_files(path: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    if path.is_file() {
        if has_supported_extension(path) {
            files.push(path.to_path_buf());
        }
        return files;
    }

    for entry in WalkBuilder::new(path).build() {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Warning: {e}");
                continue;
            }
        };
        let path = entry.path();
        if path.is_file() && has_supported_extension(path) {
            files.push(path.to_path_buf());
        }
    }

    files.sort();
    files
}

fn has_supported_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| SUPPORTED_EXTENSIONS.contains(&ext))
}
