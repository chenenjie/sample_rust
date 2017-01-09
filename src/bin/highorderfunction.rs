// //一个普通函数 接受一个数字 返回一个加一
// fn add_one(x: i32) -> i32 {x + 1}

// //一个普通函数 接受一个函数和一个数字 参数函数接受一个数字返回一个数字
// fn apply<F>(f: F, y: i32) -> i32
//     where F: Fn(i32) -> i32 {
//         f(y) * y
// }

// //这里用box是因为fn<i32)->i32的trait 
// //trait是没用固定的长度 无法确定分配它需要内存的大小
// //所以这里的返回值要用box包裹着
// fn factory(x: i32) -> Box<Fn(i32) -> i32> {
//     Box::new(move |y| x + y)
// }


fn main() {
//     let transform: fn(i32) -> i32 = add_one;
//     let f0 = add_one(2i32) * 2;
//     let f1 = apply(add_one, 2);
//     let f2 = apply(transform, 2);

//     println!("{}, {}, {}", f0, f1, f2);


//     let closure = |x: i32| x + 1;
//     let c0 = closure(2i32) * 2;
//     let c1 = apply(closure, 2);
//     let c2 = apply(|x| x + 1, 2);
//     println!("{}, {}, {}", c0, c1, c2);


//     let box_fn = factory(1i32);
//     let b0 = box_fn(2i32) * 2;
//     let b1 = (*box_fn)(2i32) * 2;
//     let b2 = (&box_fn)(2i32) * 2;
//     println!("{}, {}, {}", b0, b1, b2);

//     let add_num = &(*box_fn);
//     let translate: &fn(i32) -> i32 = add_num;
//     let z0 = add_num(2i32) * 2;
//     let z1 = apply(add_num, 2);
//     let z2 = apply(translate, 2);
//     println!("{}, {}, {}", z0, z1, z2);
    println!("hello world");
}