enum Color{
    Red,Blue,Green
}

fn main() {
    
    let color_a:Color=Color::Red;
    // println!("{}",colorA)

    match color_a{
        Color::Red=>println!("Red"),
        Color::Blue=>println!("Blue"),
        Color::Green=>println!("Green"),
       
    }

}
