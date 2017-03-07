extern crate tempdir;

use std::fs::File;
use std::io::Write;
use tempdir::TempDir;


fn main() {
    let tmp_dir = TempDir::new("example").expect("crate temp dir");
    let file_path = tmp_dir.path().join("my-temporary-note.txt");
    let mut tmp_file = File::create(file_path).expect("create emp file");
    writeln!(tmp_file, "Brian was here. Briefly.").expect("write temp file");


    drop(tmp_file);
    tmp_dir.close().expect("delete temp dir");
}