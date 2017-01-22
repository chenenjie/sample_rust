#[macro_use]
extern crate lazy_static;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

fn main() {
            
    let map: Arc<Mutex<HashMap<(i32, i32), Box<ChessStyle>>>> = Arc::new(Mutex::new(HashMap::new()));
    
}

struct Chess<T:StepRule>{
    position: i32,
    role: T,
}

impl<T:StepRule> Chess<T>{
    fn new(position: i32, role: T) -> Chess<T>{
        Chess{
            position: position,
            role: role,
        }
    }
}

trait StepRule {
    fn next_step(&self);
}

struct Horse;

impl StepRule for Horse {
    fn next_step(&self) {

    }
}

trait ChessStyle {
    fn set_position(&mut self, target: i32);
}

impl<T: StepRule> ChessStyle for Chess<T> {
    fn set_position(&mut self, target: i32) {

    }
}



