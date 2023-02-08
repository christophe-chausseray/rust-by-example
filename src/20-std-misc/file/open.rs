// The File struct represents a file that has been opened (it wraps a file descriptor),
// and gives read and/or write access to the underlying file.

// Since many things can go wrong when doing file I/O, all the File methods return the io::Result<T> type,
// which is an alias for Result<T, io::Error>.

// The open function can be used to open a file in read-only mode.

// A File owns a resource, the file descriptor and takes care of closing the file when it is droped.

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let path = Path::new("./src/20-std-misc/file/hello.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}

