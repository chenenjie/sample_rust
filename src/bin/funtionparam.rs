fn main() {
    let xm = "xiaoming";
    let xh = "xiaohong";
    say_what(xm, hi);
    say_what(xh, hello);
}


fn hi(name: &str){
    println!("Hi, {}.", name);
}

fn hello(name: &str) {
    println!("hello, {}.", name);
}


fn say_what(name: &str, func: fn(&str)) {
    func(name);
}