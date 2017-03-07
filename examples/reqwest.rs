extern crate reqwest;

use std::collections::HashMap;
use std::io::{BufRead, BufReader};

fn main(){
    let resp = reqwest::get("https://www.rust-lang.org").unwrap();
    assert!(resp.status().is_success());


    let lines = BufReader::new(resp)
                            .lines()
                            .filter_map(|l| l.ok())
                            .take(10);

    for line in lines {
        println!("{}", line);
    }

    let client = reqwest::Client::new().unwrap();
    let res = client.post("http://httpbin.org/post")
        .body("the exact body that is sent")
        .send();
    
    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    let client = reqwest::Client::new().unwrap();
    let res = client.post("http://httpbin.org/post")
        .json(&map)
        .send();
}