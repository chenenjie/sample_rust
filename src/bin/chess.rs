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


fn main(){
    let hor = Horse;
    let ele = Elephant;

    let h_chess = Chess::new(2i32, hor);
    let e_chess = Chess::new(3i32, ele);    
    
    let map :HashMap<&str, Chess<StepRule>> = HashMap::new();
    map.insert("3", h_chess);    
    map.insert("4", e_chess);
}