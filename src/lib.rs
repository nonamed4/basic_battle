use rand::random_range;

// For special moves
type SpecialMove = Option<fn(&mut Board, side: Side, x: i32, y: i32)>;

pub const SOLDIERS : [char; 7] = [' ', 'n', 'r', 's', 't', 'i', 'd'];

#[derive(PartialEq)]
pub enum Side {
    Down,
    Up,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Soldier {
    pub character: char,
    attack_range: i32,
    damage: i32,
    pub hp: i32,
    maxhp: i32,
    pub point: i32,
    limit: bool
}

pub struct Board {
    pub x_length: i32,
    pub y_length: i32,
    up_side: Vec<Soldier>, // Goes like x1y1, x2y1, x3y1, ... x1y2, ... xlenylen.
    down_side: Vec<Soldier>, // Same thing here.
    // Our get function's coordinates start from x1y1, which is the top left corner.
}

impl Board {
    pub fn new(x_length: i32, y_length: i32) -> Board {
        Board {
            x_length,
            y_length,
            up_side: vec![Soldier::new_nothing(); (x_length * y_length) as usize],
            down_side: vec![Soldier::new_nothing(); (x_length * y_length) as usize],
        }
    }
    pub fn get(&self, x: i32, y: i32, side: &Side) -> Option<&Soldier> {
        if x > self.x_length || y > self.y_length || 1 > x || 1 > y {
            return None;
        }
        if *side == Side::Up {
            self.up_side.get(((y - 1) * self.x_length + x - 1) as usize)
        } else {
            self.down_side.get(((y - 1) * self.x_length + x - 1) as usize)
        }
    }
    pub fn view(&self) {
        print!("    ");
        for column_number in 1..=self.x_length {
            print!("{column_number:<3}");
        }
        println!();
        for y in 1..=self.y_length {
            print!("{y:>2} ");
            for i in 1..=self.x_length {
                let soldier = self.get(i, y, &Side::Up).expect("Not guaranteed to be Ok(x) (Board, view, 1)");
                if soldier.character == ' ' {
                    print!("|  ");
                } else {
                    print!("|{}{}", soldier.character, soldier.hp); // Same thing here.
                }
            }
            print!("|\n");
        }
        print!("----");
        for _ in 1..=self.x_length {
            print!("---");
        }
        println!();
        for y in 1..=self.y_length {
            print!("{y:>2} ");
            for i in 1..=self.x_length {
                let soldier = self.get(i, y, &Side::Down).expect("Not guaranteed to be Ok(x) (Board, view, 2)");
                if soldier.character == ' ' {
                    print!("|  ");
                } else {
                    print!("|{}{}", soldier.character, soldier.hp); // Same thing here.
                }
            }
            print!("|\n");
        }
    }
    pub fn place(&mut self, soldier: Soldier, x: i32, y: i32, side: Side) -> Result<(), ()> {
        let get_result : &Soldier = match self.get(x, y, &side) {
            Some(val) => val,
            None => return Err(()),
        };

        if get_result.character == ' ' {
            let index = (y - 1) * (self.x_length) + x - 1;
            if side == Side::Up {
                self.up_side[index as usize] = soldier;
                Ok(())
            } else {
                self.down_side[index as usize] = soldier;
                Ok(())
            }
        } else {
            Err(())
        }
    }
    pub fn change(&mut self, soldier: Soldier, x: i32, y: i32, side: Side) -> Result<(), ()> {
        if self.get(x, y, &side) == None {
            return Err(());
        }
        if side == Side::Up {
            self.up_side[((y - 1) * self.x_length + x - 1) as usize] = soldier.clone();
        } else {
            self.down_side[((y - 1) * self.x_length + x - 1) as usize] = soldier.clone();
        }
        Ok(())
    }
    pub fn dead_scan(&mut self) {
        for c in 0..(self.x_length * self.y_length) {
            if self.up_side[c as usize].hp < 1 {
                self.up_side[c as usize] = Soldier::new_nothing();
            }
            if self.down_side[c as usize].hp < 1 {
                self.down_side[c as usize] = Soldier::new_nothing();
            }
        }
    }
    pub fn place_scan(&mut self) {
        // Checks for self.up_side
        for x in 1..=self.x_length {
            for y in (2..=self.y_length).rev() {
                let down = self.get(x, y, &Side::Up).expect("Not guaranteed to be Some(x) (Board, place_scan, 1)");
                let up = self.get(x, y-1, &Side::Up).expect("Not guaranteed to be Some(x) (Board, place_scan, 2)");
                if up.character != ' ' && down.character == ' ' {
                    self.change(up.clone(), x, y, Side::Up).expect("Not guaranteed to be Ok(x) (Board, place_scan, 1)");
                    self.change(Soldier::new_nothing(), x, y-1, Side::Up).expect("Not guaranteed to be Ok(x) (Board, place_scan, 2)");
                }
            }
        }
        // Checks for self.down_side
        for x in 1..=self.x_length {
            for y in 2..=self.y_length {
                let down = self.get(x, y, &Side::Down).expect("Not guaranteed to be Some(x) (Board, place_scan, 3)");
                let up= self.get(x, y-1, &Side::Down).expect("Not guaranteed to be Some(x) (Board, place_scan, 4)");
                if down.character != ' ' && up.character == ' ' {
                    self.change(down.clone(), x, y-1, Side::Down).expect("Not guaranteed to be Ok(x) (Board, place_scan, 3)");
                    self.change(Soldier::new_nothing(), x, y, Side::Down).expect("Not guaranteed to be Ok(x) (Board, place_scan, 4)")
                }
            }
        }
    }
    pub fn attack_scan(&mut self) {
        for x in 1..=self.x_length {
            for y in 1..=self.y_length {
                self.get(x, y, &Side::Up)
                    .expect("Not guaranteed to be Some(x) (Board, attack_scan, 1)")
                    .clone().attack(self, Side::Up, x, y);
                self.get(x, y, &Side::Down)
                    .expect("Not guaranteed to be Some(x) (Board, attack_scan, 2)")
                    .clone().attack(self, Side::Down, x, y);
            }
        }
    }
    pub fn is_anyone_left(&self) -> bool {
        for c in 0..self.x_length*self.y_length {
            if self.down_side[c as usize].character != ' ' {
                return true;
            } else {
                continue;
            }
        }
        false
    }
    pub fn is_any_space_left(&self) -> bool {
        for c in 0..self.x_length*self.y_length {
            if self.up_side[c as usize].character == ' ' {
                return true;
            } else {
                continue;
            }
        }
        false
    }
    pub fn computers_move(&mut self) {
        loop {
            let computer_x: i32 = random_range(1..=self.x_length);
            let computer_y: i32 = random_range(1..=self.y_length);
            let computer_soldier: Soldier =
                Soldier::letter_to_soldier(SOLDIERS[random_range(1..=6) as usize])
                    .expect("Not guaranteed to be Some(Soldier) (main, 3)");
            if self.get(computer_x, computer_y, &Side::Up).expect("Not guaranteed to be Some(Soldier) (main, 4)").character != ' ' {
                if self.is_any_space_left() == false {
                    break;
                }
            } else {
                self.change(computer_soldier, computer_x, computer_y, Side::Up).expect("Not guaranteed to be Ok(()) (main, 1)");
                break;
            }
        }
    }
}

impl Soldier {
    pub fn new(
        character: char,
        attack_range: i32,
        damage: i32,
        hp: i32,
        maxhp: i32,
        point: i32,
        limit: bool
    ) -> Soldier {
        Soldier {
            character,
            attack_range,
            damage,
            hp,
            maxhp,
            point,
            limit,
        }
    }
    pub fn new_normal() -> Soldier {
        Soldier::new('n', 1, 1, 2, 2, 2, true)
    }
    pub fn new_trash() -> Soldier {
        Soldier::new('s', 1, 1, 1, 1, 1, true)
    }
    pub fn new_ranged() -> Soldier {
        Soldier::new('r', 2, 1, 1, 1, 2, true)
    }
    pub fn new_nothing() -> Soldier {
        Soldier::new(' ', 0, 0, 0, 0, 0, true)
    }
    pub fn new_tank() -> Soldier {
        Soldier::new('t', 1, 1, 4, 4, 3, true)
    }
    pub fn new_op_ranged() -> Soldier {
        Soldier::new('i', 3, 1, 1, 1,  3,false)
    }
    pub fn new_dummy() -> Soldier {
        Soldier::new('d', 0, 0, 6, 6, 2,  true)
    }
    pub fn clone(&self) -> Soldier {
        Soldier::new(
            self.character,
            self.attack_range,
            self.damage,
            self.hp,
            self.maxhp,
            self.point,
            self.limit,
        )
    }
    pub fn attack(&self, game_board: &mut Board, side: Side, x: i32, y: i32) {
        if self.character == ' ' {
            return;
        }
        if self != game_board.get(x, y, &side).expect("There is a problem: Attacking soldier does not exist. (Soldier, attack, 1)") {
            panic!("There is a problem: wrong coordinates given to the function (Soldier, attack, 1)");
        }
        if side == Side::Up {
            let mut effect: i32 = self.attack_range - game_board.y_length + y;
            if effect > game_board.y_length {
                effect = game_board.y_length;
            } else if 1 > effect {
                return;
            }
            for i in 1..=effect {
                let soldier = game_board.get(x, i, &Side::Down).expect("Not guaranteed to be Some(x) (Soldier, attack)");
                if soldier.character == ' ' {
                    continue;
                } else if soldier.hp < 1 {
                    continue;
                } else {
                    game_board.down_side[((i-1) * (game_board.x_length) + x - 1) as usize].hp -= self.damage;
                    if self.limit == true {
                        return;
                    } else {
                        continue;
                    }

                }
            }
        } else {
            let mut effect: i32 = self.attack_range - y + 1;
            if effect > game_board.y_length {
                effect = game_board.y_length;
            } else if 1 > effect {
                return;
            }
            for i in 0..effect {
                let soldier = game_board.get(x, game_board.y_length-i, &Side::Up).expect("Not guaranteed to be Some(x) (Soldier, attack)");
                if soldier.character == ' ' {
                    continue;
                } else if soldier.hp < 1 {
                    continue;
                } else {
                    game_board.up_side[((game_board.y_length-i-1) * game_board.x_length + x - 1) as usize].hp -= self.damage;
                    if self.limit == true {
                        return;
                    } else {
                        continue;
                    }

                }
            }
        }
    }
    pub fn letter_to_soldier(letter: char) -> Option<Soldier> {
        match letter {
            ' ' => Some(Soldier::new_nothing()),
            'n' => Some(Soldier::new_normal()),
            'r' => Some(Soldier::new_ranged()),
            's' => Some(Soldier::new_trash()),
            't' => Some(Soldier::new_tank()),
            'i' => Some(Soldier::new_op_ranged()),
            'd' => Some(Soldier::new_dummy()),
            _ => None,
        }
    }
    pub fn list_soldiers(detailed: bool) {
        // Please remember to update here.
        println!("Soldiers are:");
        
        for i in 1..=3 {
            let soldier = Soldier::letter_to_soldier(SOLDIERS[i]).expect("Not guaranteed to be Some(x) (Soldier, list_soldiers, 1)");
            println!("{}: damage:{}, hp:{}, range:{}, max hp:{}, point value:{}.", soldier.character, soldier.damage, soldier.hp, soldier.attack_range, soldier.maxhp, soldier.point);
        }
        
        if detailed == true {
            for i in 4..=6 {
                let soldier = Soldier::letter_to_soldier(SOLDIERS[i]).expect("Not guaranteed to be Some(x) (Soldier, list_soldiers, 1)");
                println!("{}: damage:{}, hp:{}, range:{}, max hp:{}, point value:{}.", soldier.character, soldier.damage, soldier.hp, soldier.attack_range, soldier.maxhp, soldier.point);
            }
        }
    }
}



