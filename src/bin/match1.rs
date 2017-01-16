enum Direction{
    East,
    West,
    North,
    South,
}


fn main() {
    // let dire = Direction::South;
    // match dire {
    //     Direction::East => println!("East"),
    //     Direction::North | Direction::South => {
    //         println!("South or North");
    //     },
    //     _ => println!("West"),
    // };

        let d_west = Direction::West;
        let d_str = match d_west {
            Direction::East => "East",
            Direction::North | Direction::South => {
                panic!("South or North");
            },
            _ => "West",
        };

        //没有使用到里面的对象不会move
        match d_west {
            Direction::East => "East",
            Direction::North | Direction::South => {
                panic!("South or North");
            },
            _ => "West",
        };


        println!("{}", d_str);
}