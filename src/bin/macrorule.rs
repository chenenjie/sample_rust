macro_rules! creat_function {
    ($func_name:ident)=>(
        fn $func_name(){
            println!("function {:?} is called", stringify!($func_name))
        }        
    )
}

fn main() {
    creat_function!(foo);
    foo();
}