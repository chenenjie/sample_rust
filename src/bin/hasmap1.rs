
// hashmap类型 key类型可哈希 value类型编译可知道大小 
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Key {
    a: i32,
    b: i32,
}

fn main() {
    let mut come_from = HashMap::new();

    come_from.insert("WaySLOG", "HeBei");
    come_from.insert("Marisa", "U.S");
    come_from.insert("Mike", "HuoGuo");


    if !come_from.contains_key("elton") {
        println!("Oh, 我们查到了{}个人， 但是可怜的elton猫还是无家可归", come_from.len());
    }
    come_from.remove("Mike");

    let who = ["MoGu", "Marisa"];
    for person in &who {
        match come_from.get(person) {
            Some(location) => println!("{} 来自: {}", person, location),
            None => println!("{} 也无家可归", person),
        }
    }

    for (name, location) in &come_from {
        println!("{} 来自: {}", name, location);
    }
}