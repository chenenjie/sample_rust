
fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices(){
        if c == needle {
            return Some(offset);
        }
    }
    None   
}


fn main() {
    let file_name = "foobar.rs";
    match find(file_name, '.') {
        None => println!("No file extension found."),
        Some(i) => println!("file extension: {}", &file_name[i+1..]),
    }
}