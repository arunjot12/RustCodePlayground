fn switch_case() {
    enum Colors{
        Blue,
        White,
        Black
    }
    
    let expected_color = Colors::Black;
    
    match expected_color {
        Colors::Blue => println!("Color is Blue"),
        Colors::Black => println!("Black is the right option"),
        _ =>  return ()
    };
}