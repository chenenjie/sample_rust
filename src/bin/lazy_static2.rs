#[macro_use]
extern crate lazy_static;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

lazy_static!{
    pub static ref MAP:Arc<Mutex<HashMap<(i32, i32), Box<Chess>>>> = Arc::new(Mutex::new(HashMap::new()));
}
fn main() {
    // let map:Arc<Mutex<HashMap<(i32, i32), Box<Chess>>>> = Arc::new(Mutex::new(HashMap::new()));
}

//这里直接存traitobject对象的指针
//这里Chess就不是泛型 直接放map里面都可以
pub struct Chess{
    position: i32,
    role: Box<StepRule>,
}

impl Chess{
    fn new<T:StepRule>(position: i32, role: T) -> Chess{
        Chess{
            position: position,
            role: Box::new(role) as Box<StepRule>,
        }
    }
}

trait StepRule:Send + Sync + 'static{
    fn next_step(&self);
}

struct Horse;

impl StepRule for Horse {
    fn next_step(&self) {

    }
}
