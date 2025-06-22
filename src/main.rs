mod tools;

use tools::*;
use basic_battle::*;
fn main() {

    //Testing attack.
    let mut game_board = Board::new(4, 5);

    for x in 0..=5 {
        game_board.change(Soldier::new_op_ranged(), x, 2, Side::Up);
        game_board.change(Soldier::new_op_ranged(), x, 3, Side::Up);
        game_board.change(Soldier::new_op_ranged(), x, 4, Side::Up);
        game_board.change(Soldier::new_tank(), x, 2, Side::Down);
        game_board.change(Soldier::new_tank(), x, 1, Side::Down);
        game_board.change(Soldier::new_op_ranged(), x, 0, Side::Down);
    }
    loop {
        game_board.view();
        input();
        game_board.attack_scan();
        game_board.dead_scan();
        game_board.place_scan();
    }

}