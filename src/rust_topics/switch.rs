pub fn show_switch() {
    println!("--- Match Statement (Switch) Demo ---");
    #[allow(dead_code)]
    enum Colors {
        Blue,
        White,
        Black,
    }

    let expected_color = Colors::Black;

    match expected_color {
        Colors::Blue => println!("Color is Blue"),
        Colors::Black => println!("Black is the right option"),
        _ => return (),
    };
}
