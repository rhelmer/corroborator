extern crate corroborator;
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

    if file_entries.len() != CATALOG.lines().count() {
        panic!(
            "actual entries do not match expected, {} vs. {}",
            file_entries.len(),
            CATALOG.lines().count()
        );
    }

    for expected_entry in CATALOG.lines() {
        let mut iter = expected_entry.split_whitespace();
        let expected_checksum = iter.next().unwrap();
        let filename = iter.next().unwrap();

        let actual_checksum = file_entries.get(filename).unwrap();

        assert_eq!(expected_checksum, actual_checksum, "{}", filename);
    }
}
