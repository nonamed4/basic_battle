pub fn clear_screen() {
    print!("\x1Bc");
}

pub fn input() -> String {
    let mut user_input = String::new();

    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Unexpected error while taking input.");

    String::from(user_input.trim())
}

pub fn string_to_i32(value: String) -> Result<i32, String> {
    let value: i32 = match value.trim().parse() {
        Ok(num) => num,
        Err(_) => return Err(String::from("Input isn't a number!")),
    };
    Ok(value)
}
