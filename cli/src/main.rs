extern crate glob;
extern crate prettify;
extern crate prettify_markdown;

mod lib;

use glob::glob;
use lib::{format_by_language, get_elapsed_string, get_language_from_filename};
use std::env;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    let args: Vec<String> = env::args().collect();
    let glob_pattern_index = if cfg!(debug_assertions) { 2 } else { 1 };
    let default_glob_pattern = "**/*.*";
    let glob_pattern = match args.get(glob_pattern_index) {
        Some(path) => path,
        None => default_glob_pattern,
    };
    let files = match glob(glob_pattern) {
        Ok(files) => files,
        Err(e) => panic!("Error: {}", e),
    };
    let mut file_count = 0;
    for entry in files {
        match entry {
            Ok(path) => {
                if path.is_file() {
                    let language =
                        get_language_from_filename(path.file_name().unwrap().to_str().unwrap());
                    match language {
                        Some(matched_language) => {
                            let start_format_time = Instant::now();
                            let contents = std::fs::read_to_string(&path).unwrap();
                            let formatted = format_by_language(&contents, matched_language);
                            std::fs::write(&path, formatted).unwrap();
                            let elapsed = start_format_time.elapsed();
                            print!(
                                "\nFormatted ./{} in {}",
                                path.display(),
                                get_elapsed_string(elapsed)
                            );
                            file_count += 1;
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
    let elapsed = start_time.elapsed();
    let elapsed_string = get_elapsed_string(elapsed);
    println!(
        "\n\nDone!\nFormatted {} files in {}",
        file_count, elapsed_string
    );
}
