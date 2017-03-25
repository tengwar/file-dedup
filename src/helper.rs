use std::fs::{DirEntry, File, read_dir};
use std::io::{BufReader, Read};
//use std::iter::Iterator;
use std::path::Path;
use errors::*;

pub fn visit_dirs(dir: &Path, command: &mut FnMut(&DirEntry) -> Result<()>) -> Result<()> {
	if dir.is_dir() {
		for entry in read_dir(dir)
			.chain_err(|| format!("Cannot read the directory \"{}\"", dir.display()))?
		{
			let entry = entry.chain_err(|| "Cannot read a path from the directory.")?;
			let path = entry.path();
			if path.is_dir() {
				visit_dirs(&path, command)
					.chain_err(|| format!("Cannot read the \"{}\" subdirectory.", path.display()))?;
			} else {
				command(&entry)?
			}
		}
		Ok(())
	} else {
		bail!(format!("The path \"{}\" is not a directory.", dir.display()));
	}
}

pub fn are_files_identical(path1: &Path, path2: &Path) -> Result<bool> {
	if path1.is_file() && path2.is_file() {
		let file1 = File::open(path1)
			.chain_err(|| format!("Cannot open the file \"{}\".", path1.display()))?;
		let file2 = File::open(path2)
			.chain_err(|| format!("Cannot open the file \"{}\".", path2.display()))?;

		let mut bytes_or_err1 = BufReader::new(file1).bytes();
		let mut bytes_or_err2 = BufReader::new(file2).bytes();

		loop {
			match (bytes_or_err1.next(), bytes_or_err2.next()) {
				(None, None) => return Ok(true),
				(None, _) | (_, None) => return Ok(false),
				(Some(byte_or_err1), Some(byte_or_err2)) => {
					let byte1 = byte_or_err1
						.chain_err(|| format!("Can't read next byte from file \"{}\".", path1.display()))?;
					let byte2 = byte_or_err2
						.chain_err(|| format!("Can't read next byte from file \"{}\".", path2.display()))?;
					if byte1 != byte2 {
						return Ok(false)
					}
				}
			}
		}
	} else if path1.is_dir() && path2.is_dir() {
		bail!("Expected 2 files for comparison, got 2 directories. (\"{}\" and \"{}\".)",
			path1.display(), path2.display());
	} else {
		let (file, dir) = if path1.is_file() && path2.is_dir() {
			(path1, path2)
		} else {
			(path2, path1)
		};
		bail!("Expected 2 files for comparison, got a file (\"{}\") and a directory (\"{}\").",
			file.display(), dir.display());
	}
}