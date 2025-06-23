use rand::Rng;

pub fn clear_screen() {
    print!("\x1Bc");
}

pub fn input() -> String {
    let mut user_input = String::new();

    match std::io::stdin().read_line(&mut user_input) {
        Ok(_) => {}
        Err(_) => {
            println!("Warning: Unexpected error while taking input.");
            return input();
        }
    }
       

    String::from(user_input.trim())
}

pub fn string_to_i32(value: String) -> Result<i32, String> {
    let value: i32 = match value.trim().parse() {
        Ok(num) => num,
        Err(_) => return Err(String::from("Input isn't a number!")),
    };
    Ok(value)
}

pub fn random_in_range(smallest: i32, biggest: i32) -> i32 {
    let mut rng = rand::rng();
    rng.random_range(smallest..=biggest)
}

pub fn take_first_character(string: &String) -> Option<char> {
    string.chars().next()
}