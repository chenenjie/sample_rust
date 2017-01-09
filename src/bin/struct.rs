struct Enity{
    inner: Inner,  
}

struct Inner{
    num : i32,
}

impl Inner {
    fn new(i: i32) -> Inner {
        Inner{
            num: i
        }
    }
}

//可以修改struct里面的子属性
fn main(){
    let inner =  Inner::new(2);
    let mut enity: Enity = Enity{
        inner: inner,
    };

    enity.inner = Inner::new(3);
    {
        //这样就move进去
        // let Enity{inner: i0, ..} = enity;
        // println!("{}", i0.num);

        let Enity{inner: ref i0, ..} = enity;
        println!("{}", i0.num);

        //match也是move进去
        match enity {
            Enity{inner: ref inner, ..} => {
                println!("{}", inner.num);
            }
        }
    }
    
    println!("{}", enity.inner.num);
}