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
    type Output; 
    fn get_self(&self) -> &Self::Output;
}

impl ChessStyle for Chess<Elephant>{
    type Output = Elephant;

    fn get_self(&self) -> &Elephant{
        &self.inner
    }
}

impl ChessStyle for Chess<Horse>{
    type Output = Horse;

    fn get_self(&self) -> &Horse {
        &self.inner
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