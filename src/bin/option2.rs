
fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices(){
        if c == needle {
            return Some(offset);
        }
    }
    None   
}


fn extension_explicit(file_name: &str) -> Option<&str>{
    match find(file_name, '.') {
        None => None,
        Some(i) => Some(&file_name[i+1..]),        
    }
}

fn extension(file_name: &str) -> Option<&str> {
    find(file_name, '.').map(|i| &file_name[i+1..])
}

fn main() {
    match extension("foo.rs") {
        None => println!("no extension"),
        Some(ext) => println!("{}",ext),
    }
}