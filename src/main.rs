mod tools;

use rand::{random, random_range};
use tools::*;
use basic_battle::*;

fn take_coordinate_input(game_board: &Board) -> (i32, i32) {
    println!("Please enter the x coordinate now.");
    let x: i32 = loop {
        match string_to_i32(input()) {
            Ok(val) => {
                if val < 1 || val > game_board.x_length {
                    println!("This x coordinate is not valid. Please enter again.");
                    continue;
                } else {
                    break val;
                }
            },
            Err(_) => {
                println!("Please enter a number!");
                continue;
            },
        }
    };

    println!("Please enter the y coordinate now.");
    let y : i32 = loop {
        match string_to_i32(input()) {
            Ok(val) => {
                if val < 1 || val > game_board.y_length {
                    println!("This y coordinate is not valid. Please enter again.");
                    continue;
                } else {
                    break val;
                }
            },
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        }
    };
    (x, y)
}

fn main() {
    let mut game_board = Board::new(6, 3);
    let mut points: i32 = 15;
    let mut turn: i32 = 0;

    //Placing a starter soldier.
    game_board.place(Soldier::new_trash(), 1, 1, Side::Down).expect("Not guaranteed to be Ok(x) (main, 1)");

    loop {
        turn += 1;
        clear_screen();
        println!("Turn: {}", turn);
        game_board.view();


        match game_board.is_anyone_left() {
            true => {}
            false => {
                println!("There isn't anyone left! You survived {} turns.", turn);
                std::process::exit(0);
            }
        }
        // points += random_range(1..=3);
        Soldier::list_soldiers(true);
        println!("You have {points} points. Please enter the letter of the soldier you want to put.");
        println!("Enter 'E' to skip this turn.");
        let soldier: Soldier = loop { // This part works for sure
            let letter: String = input();
            if letter.len() > 0 {
                let letter = letter.chars().next().expect("Not guaranteed to be Some(char) (main, 1)");
                match Soldier::letter_to_soldier(letter) {
                    Some(soldier) => {
                        if points < soldier.point {
                            println!("You don't have enough points. You can't put that soldier. Please enter again.");
                            continue;
                        } else {
                            break soldier;
                        }
                    },
                    None => {
                        if letter == 'E' {
                            break Soldier::new_nothing();
                        }
                    }
                }
            } else {
                println!("Please enter again!");
                continue;
            }
        };
        points -= soldier.point;
        
        if soldier.character != ' ' {
        let (x, y) = loop {
            let (x, y) = take_coordinate_input(&game_board);

            match game_board.get(x, y, &Side::Down).expect("Not guaranteed to be Some(Soldier) (main, 2)").character {
                ' ' => {
                    // No problem here.
                    break (x, y);
                }
                _ => {
                    println!("Are you sure you want to replace the soldier at ({x}, {y}) with {} (Y/anything else)", soldier.character);
                    match take_first_character(&input()) {
                        Some(val) => {
                            if val == 'Y' || val == 'y' {
                                // No problem here either.
                                break (x, y);
                            } else {
                                continue;
                            }
                        }
                        None => {
                            println!("Input is not valid!");
                        }
                    }
                }
            }
        };
            game_board.change(soldier, x, y, Side::Down).expect("Not always guaranteed to be Ok(())");
        }
        // Time for the computer to "play".

        game_board.computers_move();
        game_board.view();
        input();
        
        game_board.attack_scan();
        game_board.dead_scan();
        game_board.place_scan();
    }
}