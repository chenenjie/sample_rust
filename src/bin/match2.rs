
enum Action{
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}


fn main(){
    let action = Action::Say("Hello Rust".to_string());
    match action {
        Action::Say(s) => {
            println!("{}", s);
        },
        Action::MoveTo(x, y) => {
            println!("point from (0, 0) move to ({}, {})", x, y);
        },
        Action::ChangeColorRGB(r, g, _) => {
            println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored", r, g,)
        }
    }
}