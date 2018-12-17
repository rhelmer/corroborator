extern crate corroborator;
extern crate diff;
extern crate docopt;

use std::str;

use corroborator::checksum_dir;

use docopt::Docopt;

// compile catalog into binary.
const CATALOG: &'static str = include_str!("../data/catalog.txt");

const USAGE: &'static str = "
Usage:
  corroborator <appdir>
  corroborator (-h | --help)

Options:
  -h --help     Show this screen.
";

fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.parse())
        .unwrap_or_else(|e| e.exit());

    let appdir = args.get_str("<appdir>");

    let file_entries = checksum_dir(appdir);

    let left = file_entries.as_str();
    let right = CATALOG;

    if left != right {
        for diff in diff::lines(left, &right) {
            match diff {
                diff::Result::Left(l) => println!("-{}", l),
                diff::Result::Both(l, _) => println!(" {}", l),
                diff::Result::Right(r) => println!("+{}", r),
            }
        }
        panic!("does not match");
    }
}
