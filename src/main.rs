extern crate sha2;
extern crate walkdir;

use std::fs::File;
use std::io::prelude::*;

use sha2::{Digest, Sha512};
use walkdir::WalkDir;

const BUFFER_SIZE: usize = 1024;

/// Print digest result as hex string and name pair
fn print_result(sum: &[u8], name: &str) {
    for byte in sum {
        print!("{:02x}", byte);
    }
    println!("\t{}", name);
}

/// Compute digest value for given `Reader` and print it
/// On any error simply return without doing anything
fn process<D: Digest + Default, R: Read>(reader: &mut R, name: &str) {
    let mut sh = D::default();
    let mut buffer = [0u8; BUFFER_SIZE];
    loop {
        let n = match reader.read(&mut buffer) {
            Ok(n) => n,
            Err(_) => return,
        };
        sh.input(&buffer[..n]);
        if n == 0 || n < BUFFER_SIZE {
            break;
        }
    }
    print_result(&sh.result(), name);
}

fn main() {
    for entry in WalkDir::new("/Applications/Firefox.app") {
        let entry = entry.unwrap();
        let path = entry.path();

        if !path.is_dir() {
            println!("blah");

            if let Ok(mut file) = File::open(&path) {
                process::<Sha512, _>(&mut file, "blah");
            }
        }
    }
}
