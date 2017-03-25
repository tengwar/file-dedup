// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
	// Create the Error, ErrorKind, ResultExt, and Result types
	error_chain! { }
}
mod helper;

use std::fs::DirEntry;
use std::path::{Path,PathBuf};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use errors::*;

fn main() {
	if let Err(ref e) = run() {
		use ::std::io::Write;
		let stderr = &mut ::std::io::stderr();
		let errmsg = "Error writing to stderr";

		writeln!(stderr, "error: {}", e).expect(errmsg);

		for e in e.iter().skip(1) {
			writeln!(stderr, "caused by: {}", e).expect(errmsg);
		}

		// The backtrace is not always generated. Try to run this example
		// with `RUST_BACKTRACE=1`.
		if let Some(backtrace) = e.backtrace() {
			writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
		}

		::std::process::exit(1);
	}
}

// Most functions will return the `Result` type, imported from the
// `errors` module. It is a typedef of the standard `Result` type
// for which the error type is always our own `Error`.
fn run() -> Result<()> {
	let paths = [Path::new("/home/elgregor/Obrazy"), Path::new("/home/elgregor/testing_folder2")];
	let mut files_by_length: HashMap<u64, PathBuf> = HashMap::new();

	for path in &paths {
		helper::visit_dirs(path, &mut |dir_entry: &DirEntry| -> Result<()> {
			let dir_pathbuf = dir_entry.path();
			//println!("{}", dir_pathbuf.display()); // TODO: Remove this debug println.
			let metadata = dir_entry.metadata()
			                        .chain_err(|| format!("Couldn't get metadata of \"{}\"", dir_pathbuf.display()))?;
			match files_by_length.entry(metadata.len()) {
				Entry::Occupied(occupied_entry) => {
					let duplicate = occupied_entry.get();
					let identical = helper::are_files_identical(duplicate, dir_pathbuf.as_path())
						.chain_err(|| format!("Couldn't compare these files: \"{}\" and \"{}\"",
						                      duplicate.display(),
						                      dir_pathbuf.display()))?;
					if identical {
						println!("Duplicate files: \"{}\" and \"{}\"", duplicate.display(), dir_pathbuf.display());
					}
				},
				Entry::Vacant(entry) => { entry.insert(dir_entry.path()); }
			}
			Ok(())
		})?;
	}

	Ok(())
}
