use std::io::Read;

use argparse::{ArgumentParser, Collect, Store, StoreTrue};
use regex::Regex;

const RED: &str = "\x1b[31;1m";
const PINK: &str = "\x1b[35m";
const RESET: &str = "\x1b[0m";

fn main() {
    //std::env::set_var("RUSTLOG", "debug");
    let options = parse_args();
    debug!("{:#?}", options);

    let re = Regex::new(&options.pattern).unwrap();
    let re_bytes = regex::bytes::Regex::new(&options.pattern).unwrap();
    if let Some(ref text) = options.text {
        search_text(text, &re, &options, 1, None);
    } else if let Some(ref files) = options.files {
        let num_files = files.len();
        for file in files {
            // if path is a directory, skip it
            if std::fs::metadata(file).unwrap().is_dir() {
                if options.recursive {
                    search_folder(&options, file, &re, &re_bytes);
                    continue;
                } else {
                    eprintln!("{file}: is a directory");
                    continue;
                }
            }
            search_file(file, &re, &re_bytes, num_files, &options);
        }
    }
}

fn search_text(text: &str, re: &Regex, options: &Options, num_files: usize, path: Option<&String>) {
    for line in text.lines() {
        if re.is_match(line) && !options.invert_match {
            let new_line = line.replace(
                &options.pattern,
                &format!("{RED}{}{RESET}", &options.pattern),
            );
            if num_files > 1 {
                if let Some(path) = path {
                    println!("{PINK}{path}{RESET}:{new_line}");
                } else {
                    println!("{new_line}");
                }
            } else {
                println!("{new_line}");
            }
        } else if !re.is_match(line) && options.invert_match {
            if num_files > 1 {
                if let Some(path) = path {
                    println!("{PINK}{path}{RESET}:{line}");
                } else {
                    println!("{line}");
                }
            } else {
                println!("{line}");
            }
        }
    }
}

fn search_folder(options: &Options, path: &String, re: &Regex, re_bytes: &regex::bytes::Regex) {
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            search_folder(options, &path.to_str().unwrap().to_string(), re, re_bytes);
        } else {
            search_file(
                &path.to_str().unwrap().to_string(),
                re,
                re_bytes,
                2,
                options,
            );
        }
    }
}

fn search_file(
    path: &String,
    re: &Regex,
    re_bytes: &regex::bytes::Regex,
    num_files: usize,
    options: &Options,
) {
    let mut text = String::new();
    let file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{path}: {e}");
            return;
        }
    };
    // if file is a binary file
    match std::io::BufReader::new(&file).read_to_string(&mut text) {
        Ok(_) => {
            // prelim search for pattern in text
            if re.is_match(&text) {
                search_text(&text, re, options, num_files, Some(path));
            }
            // search_text(&text, re, options, num_files, Some(path));
        }
        Err(_e) => {
            // read to u8 vector
            let mut buf = Vec::new();
            match std::io::BufReader::new(&file).read_to_end(&mut buf) {
                Ok(_) => {
                    // search for pattern in u8 vector
                    if re_bytes.is_match(&buf) {
                        println!("{path}: binary file matches");
                    }
                }
                Err(_e) => {
                    eprintln!("{path}: binary file cannot read to vec");
                    return;
                }
            }
        }
    }
}

fn parse_args() -> Options {
    let mut pattern = String::new();
    let mut files = Vec::<String>::new();
    let mut text = String::new();
    let mut recursive = false;
    let mut case_insensitive = false;
    let mut invert_match = false;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Rust grep");
        ap.refer(&mut pattern)
            .add_argument("pattern", Store, "Pattern to search for")
            .required();
        ap.refer(&mut files)
            .add_argument("files", Collect, "Files to search");
        ap.refer(&mut recursive)
            .add_option(&["-r", "--recursive"], StoreTrue, "Recursive search");
        ap.refer(&mut case_insensitive).add_option(
            &["-i", "--ignore-case"],
            StoreTrue,
            "Case insensitive search",
        );
        ap.refer(&mut invert_match).add_option(
            &["-v", "--invert-match"],
            StoreTrue,
            "Invert match",
        );
        ap.parse_args_or_exit();
    }
    if files.is_empty() {
        std::io::stdin().read_to_string(&mut text).unwrap();
    }
    Options {
        pattern,
        files: if files.is_empty() { None } else { Some(files) },
        text: if text.is_empty() { None } else { Some(text) },
        recursive,
        case_insensitive,
        invert_match,
    }
}

#[derive(Debug, Clone)]
struct Options {
    pattern: String,
    files: Option<Vec<String>>,
    text: Option<String>,
    recursive: bool,
    case_insensitive: bool,
    invert_match: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let options = Options {
            pattern: String::new(),
            files: None,
            text: None,
            recursive: false,
            case_insensitive: false,
            invert_match: false,
        };
        assert_eq!(options.pattern, String::new());
        assert_eq!(options.files, None);
        assert_eq!(options.text, None);
        assert_eq!(options.recursive, false);
        assert_eq!(options.case_insensitive, false);
        assert_eq!(options.invert_match, false);
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        const BLUE: &str = "\x1b[34m";
        const RESET: &str = "\x1b[0m";
        if std::env::var("RUSTLOG") == Ok("debug".to_string()) {
            eprintln!("{BLUE}DEBUG{RESET} > {}", format!($($arg)*));
        }
    }
}
