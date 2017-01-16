use std::collections::HashMap;


fn main() {
    
    let mut letters = HashMap::new();
    
    for ch in "a short treatise on funqi".chars(){
        let counter = letters.entry(ch).or_insert(0);
        *counter += 1;
    }


    for (name, count) in &letters {
        println!("{} count is {}", name, count);
    }
}