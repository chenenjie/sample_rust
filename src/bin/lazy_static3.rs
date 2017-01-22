#[macro_use]
extern crate lazy_static;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
//放入lazy_static map的trait必须死实现了static和pub
lazy_static! {
    pub static ref Map: Arc<Mutex<HashMap<(i32, i32), Box<ChessStyle>>>> = Arc::new(Mutex::new(HashMap::new()));
}

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

pub trait StepRule:'static + Send + Sync {
    fn next_step(&self);
}

struct Horse;

impl StepRule for Horse {
    fn next_step(&self) {

    }
}

pub trait ChessStyle:'static + Sync + Send {
    fn set_position(&mut self) -> &StepRule;
}

impl<T: StepRule> ChessStyle for Chess<T> {
    //本意返回role的对象
    //有 type 类型的 就是含有泛型,无法转成traitobject
    fn set_position(&mut self) -> &StepRule {
        &self.role
    }
}



