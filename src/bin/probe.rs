
struct Chess {
    position: i32,
}

trait Probe {
    fn handle(&mut self, func: fn(&mut Chess));
}

impl Probe for Chess {
    fn handle(&mut self, func: fn(&mut Chess)){
        func(self);
    }
}

fn handl_chess(chess: &mut Chess){
    println!("position: {}", chess.position);
}

fn handl_ches(chess: &mut Chess){
    println!("position: {}", chess.position);
}

fn main() {
    let mut probe = Box::new(Chess{position: 8i32}) as Box<Probe>;
    probe.handle(handl_chess);
    probe.handle(handl_ches);
}