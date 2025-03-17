use chrono::{DateTime, Local};
use glob::glob;
use regex::Regex;
use std::env;
use std::fs::{self};
use std::io;
use std::path::Path;
use std::process;

// Get the timestamp the file was last modified.
fn get_modified_timestamp(file_path: &Path) -> Result<String, io::Error> {
    let metadata = fs::metadata(file_path)?;
    let modified_date = metadata.modified()?;
    let datetime: DateTime<Local> = modified_date.into();
    let formatted = datetime.format("%Y%m%d_%H%M").to_string();
    Ok(formatted)
}

// Process files that match a file pattern.
// dryrun flag will show you what would happen without doing it.
fn append_creation_date_to_filename(file_pattern: &str, dryrun: bool) {
    match glob(file_pattern) {
        Ok(paths) => {
            let mut matched_files = 0;
            for path in paths {
                match path {
                    Ok(path) => {
                        if path.is_file() {
                            matched_files += 1;
                            println!("Matched file: {}", path.display());
                            process_file(&path, dryrun);
                        }
                    }
                    Err(e) => eprintln!("Error processing path: {}", e),
                }
            }

            if matched_files == 0 {
                println!("No files matched the pattern: {}", file_pattern);
            } else {
                println!(
                    "There were {} files matching the pattern {}.",
                    matched_files, file_pattern
                )
            }
        }
        Err(e) => eprintln!("Error matching glob pattern: {}", e),
    }
}

// Process file will extract the last modified timestamp from the file metadata.
// If the timestamp is already in the filename in the expected position, it won't add it.
// If the dryrun flag is set to true, it won't add it either.
// Otherwise it will add the last modified timestamp to the filename, right before the file extension.
fn process_file(path: &Path, dryrun: bool) {
    match get_modified_timestamp(path) {
        Ok(date_str) => {
            println!("File creation date: {}", date_str);
            let file_stem = path.file_stem().unwrap_or_default().to_string_lossy();
            let extension = path.extension().unwrap_or_default().to_string_lossy();
            let re = Regex::new(r"_(\d{8}_\d{4})$").unwrap();
            if re.is_match(&file_stem) {
                println!("Skipping file (date already present): {}\n", path.display());
                return;
            }
            let new_file_name = format!("{}_{}.{}", file_stem, date_str, extension);
            if dryrun {
                println!("Would rename: {} -> {}", path.display(), new_file_name);
            } else if let Err(e) = fs::rename(&path, path.with_file_name(new_file_name)) {
                eprintln!("Error renaming file: {}", e);
            }
        }
        Err(e) => {
            eprintln!(
                "Failed to get creation date for file: {}: {}",
                path.display(),
                e
            );
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        eprintln!(
            "{} version {}\nA program to add a the creation datestamp to the filename\nUsage: {} <filepattern in quotes> [--dryrun]",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            args[0]
        );
        process::exit(1);
    }

    let file_pattern = &args[1];
    let dryrun = if args.len() == 3 && args[2] == "--dryrun" {
        true
    } else {
        false
    };

    append_creation_date_to_filename(file_pattern, dryrun);
}

// ---

// fn get_creation_date_from_ls(file_path: &Path) -> Option<String> {
//     // Run `ls -hl` on the file
//     let output = Command::new("ls")
//         .arg("-hl") // `-h` for human-readable and `-l` for long listing format
//         .arg(file_path)
//         .output();

//     match output {
//         Ok(output) => {
//             if !output.stdout.is_empty() {
//                 let output_str = String::from_utf8_lossy(&output.stdout);
//                 let line = output_str.lines().next();

//                 if let Some(line) = line {
//                     let parts: Vec<&str> = line.split_whitespace().collect();
//                     if parts.len() >= 6 {
//                         let month_str = parts[5];
//                         let day_str = parts[6];
//                         let year_str = parts[7];

//                         let month = match month_str {
//                             "Jan" => 1,
//                             "Feb" => 2,
//                             "Mar" => 3,
//                             "Apr" => 4,
//                             "May" => 5,
//                             "Jun" => 6,
//                             "Jul" => 7,
//                             "Aug" => 8,
//                             "Sep" => 9,
//                             "Oct" => 10,
//                             "Nov" => 11,
//                             "Dec" => 12,
//                             _ => return None,
//                         };

//                         let day = match day_str.parse::<u32>() {
//                             Ok(d) => d,
//                             Err(_) => return None,
//                         };
//                         let year = match year_str.parse::<u32>() {
//                             Ok(y) => y,
//                             Err(_) => return None,
//                         };
//                         let date = NaiveDate::from_ymd(year as i32, month, day);
//                         let date_str = date.format("%Y%m%d").to_string();
//                         if parts.len() >= 9 {
//                             let time_str = parts[8]; // "10:35" for example
//                             let time_parts: Vec<&str> = time_str.split(':').collect();
//                             if time_parts.len() == 2 {
//                                 let hour = time_parts[0];
//                                 let minute = time_parts[1];
//                                 return Some(format!("{}_{}{}", date_str, hour, minute));
//                             }
//                         }

//                         // If no time part is found, default to 0000
//                         return Some(format!("{}_0000", date_str));
//                     }
//                 }
//             }
//         }
//         Err(e) => {
//             eprintln!("Error running ls: {}", e);
//         }
//     }

//     None
// }
