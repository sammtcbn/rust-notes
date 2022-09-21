/*
ref to https://stackoverflow.com/questions/26076005/how-can-i-list-files-of-a-directory-in-rust
*/

use std::fs;

fn main() {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}

/* Result:

$ ./filelist
Name: ./hello.rs
Name: ./filelist.rs
Name: ./filelist

*/
