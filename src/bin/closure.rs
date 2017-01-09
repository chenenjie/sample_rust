// 静态派发
fn call_with_one<F>(some_closure: F) -> i32
        where F : Fn(i32) -> i32{
    some_closure(1)
}

//动态派发
fn call_with_on_dynamic<F>(some_closure: &F) -> i32
        where F: Fn(i32) -> i32{
    some_closure(1)
}

fn add_one(i: i32) -> i32 {
    i + 1
}

fn factory() -> Box<Fn(i32) -> i32> {
    let num = 5;

    Box::new(move|x| x + num)
}

fn main(){
    let answer = call_with_one(|x| x+2);
    assert_eq!(3, answer);

    let another_answer =call_with_on_dynamic(&|x| x+2); 
    assert_eq!(3, another_answer);


    let f = add_one;
    let fanswer = call_with_on_dynamic(&f);
    assert_eq!(2, fanswer);
    
    let f2answer = call_with_on_dynamic(&add_one);
    assert_eq!(2, f2answer);


    let fa = factory();
    let faanswer = fa(1);
    assert_eq!(6, faanswer);
}