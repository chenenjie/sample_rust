use std::collections::HashMap;

trait StepRule{
    fn getRoad();
}


struct Chess<T: StepRule>{
    inner_chess: T,
    group : i32,
}

struct Horse;

impl StepRule for Horse{
    fn getRoad() {
    
    }
}

struct Elephant;

impl StepRule for Elephant{
    fn getRoad(){
        
    }
}
impl<T: StepRule> Chess<T> {
    fn new(inner: T, group: i32) -> Chess<T> {
        Chess{
            inner_chess: inner,
            group: group
        }
    }
}


fn main(){
    // let horse = Horse;
    // let chess = Chess::new(horse, 23i32);

    // let ele = Elephant;
    // let e_chess = Chess::new(ele, 23i32);

    // let mut map = HashMap::new();
    // map.insert("dfj", chess);
    // map.insert("dfdfjj", e_chess);
}