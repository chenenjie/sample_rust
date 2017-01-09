//测试下if let是否也会像match会move掉里面的内容

struct Enjie{
    money: i32,
}
impl Enjie{
    fn new() -> Enjie{
        Enjie{
            money: 32,
        }
    }
}

fn main() {
    let num = Some(Enjie::new());

    //需要用ref才能编译通过 一样都是会move进去i里面
    if let Some(ref i) = num {
        println!("{}", i.money)
    };

    println!("{:?}", num.unwrap().money);
}