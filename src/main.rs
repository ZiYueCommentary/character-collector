use clap::{Arg, Command};
use glob::glob;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let matches = Command::new("Character Collector")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .num_args(1)
                .action(clap::ArgAction::Append)
                .required(true)
                .help("Input file(s), directory, or wildcard pattern"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .num_args(1)
                .required(false)
                .help("Output file"),
        )
        .arg(
            Arg::new("recursive")
                .short('r')
                .long("recursive")
                .num_args(0)
                .help("Recursively search subdirectories for files"),
        )
        .arg(
            Arg::new("silence")
                .short('s')
                .long("silence")
                .num_args(0)
                .help("Disable general log prints"),
        )
        .arg(
            Arg::new("suppress")
                .short('S')
                .long("suppress")
                .num_args(0)
                .help("Suppress warnings and error logs"),
        )
        .help_template(
            "{name} - {version}
{about-section}
Collecting characters from text files.
Made by ZiYueCommentary & EasyT_T in Rust.
https://github.com/ZiYueCommentary/character-collector

{usage-heading}
{tab}character_collector -i <input>... -o <output> [-r]

{all-args}",
        )
        .get_matches();

    let input_patterns: Vec<String> = matches
        .get_many::<String>("input")
        .unwrap()
        .map(|s| s.to_owned())
        .collect();

    let recursive = matches.get_flag("recursive");
    let silence = matches.get_flag("silence");
    let suppress = matches.get_flag("suppress");

    let mut input_files: Vec<String> = Vec::new();
    for pattern in input_patterns {
        let path = Path::new(&pattern);
        if path.is_file() {
            input_files.push(pattern.clone());
        } else if path.is_dir() {
            if recursive {
                for entry in WalkDir::new(&pattern).into_iter().filter_map(|e| e.ok()) {
                    if entry.file_type().is_file() {
                        input_files.push(entry.path().to_string_lossy().to_string());
                    }
                }
            } else {
                for entry in fs::read_dir(&pattern).unwrap() {
                    let entry = entry.unwrap();
                    if entry.path().is_file() {
                        input_files.push(entry.path().to_string_lossy().to_string());
                    }
                }
            }
        } else {
            for entry in glob(&pattern).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => {
                        if path.is_file() {
                            input_files.push(path.to_string_lossy().to_string());
                        }
                    }
                    Err(e) => {
                        if !suppress {
                            eprintln!("Glob error: {}", e)
                        }
                    }
                }
            }
        }
    }

    if input_files.is_empty() {
        if !suppress {
            eprintln!("No input files found.");
        }
        std::process::exit(1);
    }

    let mut set: HashSet<char> = HashSet::new();

    for file_path in input_files {
        let mut f = match File::open(&file_path) {
            Ok(f) => f,
            Err(e) => {
                if !suppress {
                    eprintln!("Error opening file {}: {}", file_path, e);
                }
                continue;
            }
        };
        let mut buf = [0u8; 1024];
        let n = match f.read(&mut buf) {
            Ok(n) => n,
            Err(e) => {
                if !suppress {
                    eprintln!("Error reading file {}: {}", file_path, e);
                }
                continue;
            }
        };
        if std::str::from_utf8(&buf[..n]).is_err() {
            if !silence {
                println!("Skipping non-text or binary file: {}", file_path);
            }
            continue;
        }

        if !silence {
            println!("Processing {}", &file_path);
        }
        let file = match File::open(&file_path) {
            Ok(f) => f,
            Err(e) => {
                if !suppress {
                    eprintln!("Error opening file {}: {}", file_path, e);
                }
                continue;
            }
        };
        let reader = BufReader::new(file);

        reader.lines().for_each(|line| {
            if let Ok(line) = line {
                line.chars().for_each(|c| {
                    if c != '\n' && c != '\r' {
                        set.insert(c);
                    }
                })
            } else if let Err(e) = line
                && !suppress
            {
                eprintln!("Error reading line: {}", e);
            }
        });
    }

    let mut chars: Vec<char> = set.iter().cloned().collect();
    chars.sort_unstable();

    let output_dir = matches.get_one::<String>("output");
    if output_dir == None {
        for x in &chars {
            print!("{}", x);
        }
        return;
    }

    let output = match File::create(output_dir.unwrap().to_owned()) {
        Ok(f) => f,
        Err(e) => panic!("Error creating output file: {}", e),
    };
    let mut writer = BufWriter::new(&output);

    for x in &chars {
        match writer.write(x.to_string().as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                if !suppress {
                    eprintln!("Error writing to output file: {}", e)
                }
            }
        }
    }

    match writer.flush() {
        Ok(_) => (),
        Err(e) => panic!("Error flushing output file: {}", e),
    }
}
