extern crate wasm_bindgen;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn collect(file_path: &str) -> Result<String, String> {
    let mut f = match File::open(&file_path) {
        Ok(f) => f,
        Err(e) => return Err(format!("Error opening file {}: {}", &file_path, &e)),
    };
    let mut buf = [0u8; 1024];
    let n = match f.read(&mut buf) {
        Ok(n) => n,
        Err(e) => {
            return Err(format!("Error reading file {}: {}", file_path, e));
        }
    };
    if std::str::from_utf8(&buf[..n]).is_err() {
        return Err(format!("Skipping non-text or binary file: {}", file_path));
    }

    println!("Processing {}", &file_path);
    let file = match File::open(&file_path) {
        Ok(f) => f,
        Err(e) => {
            return Err(format!("Error opening file {}: {}", file_path, e));
        }
    };
    let reader = BufReader::new(file);
    let mut set = HashSet::new();

    reader.lines().for_each(|line| {
        if let Ok(line) = line {
            line.chars().for_each(|c| {
                if c != '\n' && c != '\r' {
                    set.insert(c);
                }
            })
        } else if let Err(e) = line {
            eprintln!("Error reading line: {}", e);
        }
    });

    let mut chars: Vec<char> = set.iter().cloned().collect();
    chars.sort_unstable();

    Ok(chars.iter().collect())
}

#[wasm_bindgen]
pub fn merge(prev: &str, new: &str) -> String {
    let mut set = HashSet::new();
    prev.chars().for_each(|c| {
        set.insert(c);
    });

    new.chars().for_each(|c| {
        set.insert(c);
    });

    let mut chars: Vec<char> = set.iter().cloned().collect();
    chars.sort_unstable();

    chars.iter().collect()
}
