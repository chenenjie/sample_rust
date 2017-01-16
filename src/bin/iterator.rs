use std::collections::HashMap;
fn main() {
    let m = (1..5).fold(1u64, |mul, x| mul * x);

    println!("{}", m);

    let n = (1..20).map(|x| x+1).collect::<Vec<_>>();
    println!("{:?}", n);

    let v: Vec<_> = (1..20).filter(|x| x%2 == 0).collect();
    println!("{:?}", v);


    let vi = vec![1, 2, 3, 4, 5, 6];
    let vi_take = vi.iter()
        .cloned()
        .take(2)
        .collect::<Vec<_>>();
    
    println!("vi_take :{:?}", vi_take);

    let v_skip: Vec<_> = vi.iter()
        .cloned()
        .skip(2)
        .collect();
    println!("vi_skip :{:?}", v_skip);

    let names = vec!["WaySLOG", "Mike", "Elton"];
    let scores = vec![60, 80, 100];

    let score_map: HashMap<_, _> = names.iter()
        .zip(scores.iter())
        .collect();
    println!("score_map:{:?}", score_map);


    let vt = vec![1u64, 2, 3, 4, 5, 6];
    let val = vt.iter()
        .enumerate()
        .filter(|&(idx, _)| idx % 2 == 0)
        .map(|(idx, val)| val)
        .fold(0u64, |sum, acm| sum + acm);
    
    println!("val : {}", val);

    let find_result = vec![1i32, 2, 3, 4, 5, 6];
    let val1 = find_result.iter()
        // .find(|&&item| item/2 == 1);
        .find(|item| **item/3==1);
    println!("val1 = {:?}", val1);

    let find_result = vec![1i32, 2, 3, 4, 5, 6];
    //这是返回索引值
    let val2 = find_result.iter()
        .position(|&item| item == 6);
    println!("val2 = {:?}", val2);

    // all和any的值正好相反
    let find_result = vec![1i32, 2, 3, 4, 5, 6];
    let val3 = find_result.iter()
        .all(|&item| item < 7);
    println!("val3 = {:?}", val3);

    let find_result = vec![1i32, 2, 3, 4, 5, 6];
    let val4 = find_result.iter()
        .any(|&item| item < 0);
    println!("val4 = {:?}", val4);


    let find_result = vec![1i32, 2, 3, 4, 5, 6];
    let val5 = find_result.iter()
        .max();
    println!("val5 = {:?}", val5);

    let find_result = vec![1i32, 2, 3, 4, 5, 6];
    let val6 = find_result.iter()
        .min();
    println!("val6 = {:?}", val6);
}
