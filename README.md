# file-dedup
A simple program that finds duplicate files, written in Rust.

## How it works
* Finds groups of files with the exact same size.
* Within each group it compares pairs of files¹ byte-by-byte until the first differing byte.
* Prints resulting groups of identical files.

¹ No, if A == B and B == C, it doesn't compare A with C. :)

## Why it works this way
For some reason if you ask people about file deduplication, they respond without thinking: "hash every file and compare hashes". But this approach has some downsides.

First, why do we even compare files that have different filesizes? Files can only be identical if they have the exact same size. So size acts as kind of a weak hash that we don't have to compute. Why not use it then?

Second, if there are big files with the same size, hashing them would mean we have to read (and process) whole files - that's a lot of work. But if the difference is in the first few bytes then we could just load a tiny part and bail early.

Unfortunately my approach has a small downside of its own: if user has many files that have same size and are identical (or they differ, but only very late in the file), then we do more reads than naive hasing would do on these files. But I don't think it happens very often.

## Usage
    USAGE:
        file-dedup [FLAGS] <DIR>...

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information
        -v, --verbose    Sets the level of verbosity

    ARGS:
        <DIR>...    Paths to directories that will be searched for duplicate files
