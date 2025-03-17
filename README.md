# append-file-date

Say you have a bunch of old files (e.g. jpg files) where the file modified date is correct in the metadata that the OS knows about.

Before moving these files to some other hard drive, you want to make sure you preserve the date the file was created, which can be lost in the transfer if you're not careful.

This utility will extract that datestamp and append it to the filename before the file extension, so that even if the file metadata changes, the filename will not change.


## Usage

```
$ append-file-date 
append-file-date version 0.1.0
A program to add a the creation datestamp to the filename
Usage: append-file-date <filepattern in quotes> [--dryrun]
```

## Installation

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. clone the repo and cd into the dir
3. `cargo build --release`
4. `cp -p target/release/append-file-date ~/bin`


