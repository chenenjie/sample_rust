use std::collections::HashMap;

trait StepRule{
    fn get_step();
}
struct Horse;

impl StepRule for Horse{
    fn get_step(){
        
    }
}

struct Elephant;

impl StepRule for Elephant{
    fn get_step(){

    }
}


struct Chess<T: StepRule>{
    inner: T,
    group: i32,
}


impl<T: StepRule> Chess<T>{
    fn new(group: i32, inner: T) -> Chess<T> {
        Chess{
            group: group,
            inner: inner,
        }
    }
}

trait ChessStyle{
    fn get_self(&mut self) -> i32;
}

impl<T: StepRule> ChessStyle for Chess<T>{
    fn get_self(&mut self) -> i32{
        32i32
    }
}

fn main(){
    let hor = Horse;
    let ele = Elephant;

    let h_chess = Box::new(Chess::new(2i32, hor)) as Box<ChessStyle>;
    let e_chess = Box::new(Chess::new(3i32, ele)) as Box<ChessStyle>;
    
    let mut map :HashMap<&str, Box<ChessStyle>> = HashMap::new();
    map.insert("3", h_chess);    
    map.insert("4", e_chess);
}