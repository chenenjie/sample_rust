
#[derive(Debug)]
struct Person {
    name: Option<String>,
}

fn main() {
//    let name = "Steve".to_string();
//    let x: Option<Person> = Some(Person{name: Some(name)});

//    match x {
//        //ref a 代理 Some的字串
//        Some(Person { name: ref a @ Some(_), ..}) => println!("{:?}", a),
//        _ => println!("none"),
//    }
    let x = 4;
    let y = false;

    match x {
        4 | 5 if y => println!("yes"),
        _ => println!("no"),
    }

}