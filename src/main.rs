/* ------------------------------------------------------------------------------------------------
   RRFM CLI Tool
      © 2026 Remus Rigo
         v1.0 20260709
------------------------------------------------------------------------------------------------ */

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut source: Option<PathBuf> = None;
    let mut search: Option<PathBuf> = None;

    for arg in args.iter().skip(1) {
        if let Some(val) = arg.strip_prefix("/source:")
        {
            source = Some(PathBuf::from(val));
        }
        if let Some(val) = arg.strip_prefix("/search:")
        {
            search = Some(PathBuf::from(val));
        }
    }

    let source = source.unwrap_or_else(|| {
        eprintln!("Missing /source:filename parameter");
        exit(1);
    });

    let search = search.unwrap_or_else(|| {
        eprintln!("Missing /search:path parameter");
        exit(1);
    });

    if !source.is_file() {
        eprintln!("Source file not found: {}", source.display());
        exit(1);
    }

    if !search.is_dir() {
        eprintln!("Search path not found: {}", search.display());
        exit(1);
    }

    let target_name = source.file_name().unwrap_or_else(|| {
        eprintln!("Could not determine filename from /source: value");
        exit(1);
    });

    // Resolve the source's canonical path so we never overwrite it with itself
    let source_full = fs::canonicalize(&source).unwrap_or_else(|_| source.clone());

    let mut found = 0usize;
    let mut replaced = 0usize;

    search_and_replace(&search, target_name, &source, &source_full, &mut found, &mut replaced);

    println!();
    println!("Found: {found}, Replaced: {replaced}");
}

fn search_and_replace(
    dir: &Path,
    target_name: &std::ffi::OsStr,
    source: &Path,
    source_full: &Path,
    found: &mut usize,
    replaced: &mut usize,
) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(err) => {
            eprintln!("Cannot read {}: {}", dir.display(), err);
            return;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_dir() {
            search_and_replace(&path, target_name, source, source_full, found, replaced);
        } else if path.file_name() == Some(target_name) {
            *found += 1;

            let path_full = fs::canonicalize(&path).unwrap_or_else(|_| path.clone());
            if path_full == *source_full {
                println!("Skipping source file itself: {}", path.display());
                continue;
            }

            match fs::copy(source, &path) {
                Ok(_) => {
                    println!("Replaced: {}", path.display());
                    *replaced += 1;
                }
                Err(err) => eprintln!("Failed to replace {}: {}", path.display(), err),
            }
        }
    }
}