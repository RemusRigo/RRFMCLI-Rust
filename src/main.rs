// ------------------------------------------------------------------------------------------------
// RRFMCLI - Remus Rigo File Manager Command Line Interface
//    © 2026 Remus Rigo
//       v1.0.20260728
//-------------------------------------------------------------------------------------------------

use std::env;
use std::fs;
use std::path::Path;

struct Args {
    flag_updatefile: bool,
    source_file: Option<String>,
    search_path: Option<String>,
}

// Parse command line arguments and return an Args struct
fn parse_args(args: &[String]) -> Args {
    let mut flag_updatefile = false;
    let mut source_file = None;
    let mut search_path = None;

    // /UF update file
    if args.len() > 1 && args[1].to_uppercase() == "/UF" {
        flag_updatefile = true;

        // /source:<path/sourcefile>
        if args.len() > 2 {
            let param2 = &args[2];
            if param2.to_lowercase().starts_with("/source:") {
                if let Some((_, value)) = param2.split_once(':') {
                    source_file = Some(value.trim_matches('"').to_string());
                }
            }
       }

        // /search:<path>
        if args.len() > 3 {
            let param3 = &args[3];
            if param3.to_lowercase().starts_with("/search:") {
                if let Some((_, value)) = param3.split_once(':') {
                    search_path = Some(value.trim_matches('"').to_string());
                }
            }
        }
    }

    Args { flag_updatefile, source_file, search_path }
}

//-------------------------------------------------------------------------------------------------
// Update files
fn update_files(
    current_scanned_path: &Path,
    search_filename: &std::ffi::OsStr,
    source_path: &str,
    source_path_absolute: &Path,
    updated: &mut usize,
) {
    let entries = match fs::read_dir(current_scanned_path) {
        Ok(e) => e,
        Err(err) => {
            eprintln!("Cannot read {}: {}", current_scanned_path.display(), err);
            return;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_dir() {
            update_files(&path, search_filename, source_path, source_path_absolute, updated);
        } else if path.file_name() == Some(search_filename) {
            //let target_full_path = fs::canonicalize(&path).unwrap_or_else(|_| path.clone());
            let path_full = fs::canonicalize(&path).unwrap_or_else(|_| path.clone());

            // skip if the found file IS the source file
            /*
            if target_full_path == *source_path_absolute {
                println!("Skip source: {}", target_full_path.display());
                continue;
            }
            */
            if path_full == *source_path_absolute {
                println!("Skip source: {}", path.display());
                continue;
            }   

            match fs::copy(source_path, &path) {
                Ok(_) => {
                    //println!("Updated: {}", target_full_path.display());
                    println!("Updated: {}", path.display());
                    *updated += 1;
                }
                Err(err) => eprintln!("Failed to update {}: {}", path.display(), err),
            }
        }
    }
}
//-------------------------------------------------------------------------------------------------
// print information about the command line arguments
fn print_info() {
    println!("RRFMCLI v1.0.20260728");
    println!("   © 2026 Remus Rigo");
    println!();
    println!("RRFMCLI /UF /source:<sourcefile> /search:<searchpath>");
    println!("/UF - Update Files");
    println!("/source:<sourcefile> - Specify the source file (path included)");
    println!("/search:<searchpath> - Specify the search path");
}

//-------------------------------------------------------------------------------------------------
// main function
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_info();
        std::process::exit(0);
    }

    let parsed = parse_args(&args);

    if parsed.flag_updatefile {
        let src_file_valid = match &parsed.source_file {
            Some(f) => Path::new(f).is_file(),
            None => false,
       };

        if !src_file_valid {
            println!("Error: source file not found: {:?}", parsed.source_file);
            println!();
            print_info();
            std::process::exit(1);
        }
    
        let src_path_valid = match &parsed.search_path {
            Some(p) => Path::new(p).is_dir(),
            None => false,
        };

        if !src_path_valid {
            println!("Error: search path not found: {:?}", parsed.search_path);
            println!();
            print_info();
            std::process::exit(1);
        }

        let source_file = parsed.source_file.unwrap();
        let search_path = parsed.search_path.unwrap();
        let search_filename = Path::new(&source_file).file_name().unwrap();
        let source_path_absolute = fs::canonicalize(&source_file).unwrap();
        
        let mut updated: usize = 0;

        update_files(Path::new(&search_path), search_filename, &source_file, &source_path_absolute, &mut updated);

        println!();
        println!("Total files updated: {}", updated);
    }
}
