extern crate sha2;
extern crate walkdir;

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
/// Return a string with each filename and shasum delimited by newline, e.g.
/// "checksum1 file1\checksum2 file2"
/// This format is intended to be compatible with the output of the `shasum` command-line tool.
pub fn checksum_dir(appdir: &str) -> String {
    let mut file_entries: String = "".to_owned();

    for entry in WalkDir::new(appdir) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            if let Ok(mut file) = File::open(&path) {
                let result = &process::<Sha512, _>(&mut file);
                let entry = path.strip_prefix(appdir).unwrap();

                file_entries = format!(
                    "{}{} {}\n",
                    file_entries,
                    result.to_owned(),
                    entry.to_str().unwrap(),
                );
            }
        }
    }

    file_entries
}
