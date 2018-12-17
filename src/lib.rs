extern crate sha2;
extern crate walkdir;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use sha2::{Digest, Sha512};
use walkdir::WalkDir;

const BUFFER_SIZE: usize = 1024;

/// Compute digest value for given `Reader` and print it
/// On any error simply return without doing anything
fn process<D: Digest + Default, R: Read>(reader: &mut R) -> (String) {
    let mut sh = D::default();
    let mut buffer = [0u8; BUFFER_SIZE];

    let mut result: String = "".to_owned();

    loop {
        let n = match reader.read(&mut buffer) {
            Ok(n) => n,
            Err(_) => return result,
        };
        sh.input(&buffer[..n]);
        if n == 0 || n < BUFFER_SIZE {
            break;
        }
    }

    for byte in sh.result() {
        result = format!("{}{:02x}", result, byte);
    }

    result
}

/// Walk directory and generate shasum for each.
/// Return a HashMap with filename as key and SHA-512 as value.
pub fn checksum_dir(appdir: &str) -> HashMap<String, String> {
    let mut file_entries = HashMap::new();

    for entry in WalkDir::new(appdir) {
        let entry = entry.unwrap();
        let path = entry.path().to_owned();

        if path.is_file() {
            if let Ok(mut file) = File::open(&path) {
                let result = &process::<Sha512, _>(&mut file);
                let entry = path.strip_prefix(appdir).unwrap().to_owned();

                file_entries.insert(entry.to_str().unwrap().to_owned(), result.to_owned());
            }
        }
    }

    file_entries
}
