// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate multimap;
extern crate clap;

// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
	// Create the Error, ErrorKind, ResultExt, and Result types
	error_chain! { }
}
mod helper;

use std::fs::DirEntry;
use std::path::PathBuf;
use multimap::MultiMap;
use clap::Arg;
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
fn run() -> errors::Result<()> {
	let mut same_length_files: MultiMap<u64, PathBuf> = MultiMap::new();
	let mut identical_files: Vec<Vec<PathBuf>> = Vec::new();

	// Process cmdline
	let clap_matches = clap::App::new("file-dedup")
		.version("0.1")
		.about("Finds identical files.")
		.author("Grzegorz G.")
		.arg(Arg::with_name("verbose")
			.short("v")
			.long("verbose")
			.multiple(true)
			.help("Sets the level of verbosity"))
		.arg(Arg::with_name("DIR")
			.multiple(true)
			.required(true)
			.help("Paths to directories that will be searched for duplicate files"))
		.get_matches();

	let input_paths: Vec<PathBuf> = clap_matches.values_of_os("DIR")
		.expect("DIR is required, so at least one is always present.")
		.map(|os_str| PathBuf::from(os_str))
		.collect();

	let verbosity_level = clap_matches.occurrences_of("verbose");

	// Find files with the exact same size.
	for path in &input_paths {
		helper::visit_dirs(path, &mut |dir_entry: &DirEntry| -> Result<()> {
			let dir_pathbuf = dir_entry.path();
			if verbosity_level > 0 {
				println!("{}", dir_pathbuf.display());
			}
			let metadata = dir_entry.metadata()
			                        .chain_err(|| format!("Couldn't get metadata of \"{}\"", dir_pathbuf.display()))?;
			same_length_files.insert(metadata.len(), dir_pathbuf);
			Ok(())
		})?;
	}

	// Check same-size files to find identical ones.
	for (size, paths) in same_length_files {
		if paths.len() > 1 {
			let mut some_dupes = &mut helper::find_duplicates(paths) // TODO: use human-readable sizes below.
				.chain_err(|| format!("Error finding duplicates among files with the size of {} bytes.", size))?;
			identical_files.append(some_dupes);
			/*while !paths.is_empty() {
				let i: u64 = 0;
				while i < paths.len() {
					// TODO: maybe try without mutable state?
				}
			}*/
			/*let identical = helper::are_files_identical(duplicate, dir_pathbuf.as_path())
				.chain_err(|| format!("Couldn't compare these files: \"{}\" and \"{}\"",
									  duplicate.display(),
									  dir_pathbuf.display()))?;
			if identical {
				println!("Duplicate files: \"{}\" and \"{}\"", duplicate.display(), dir_pathbuf.display());
			}*/
		}
	}

	// Print duplicate files.
	for group in identical_files {
		for pathbuf in group {
			println!("{}", &pathbuf.display());
			// TODO: maybe warn if the files are identical because they are empty?
		}
		println!();
	}

	Ok(())
}
