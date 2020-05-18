use std::fmt;
use std::io;
use regex::Regex;

const BOARD_SIZE: usize = 3;

#[derive(Copy, Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
enum CellState {
    Empty,
    X,
    O
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CellState::Empty => write!(f, " "),
            CellState::X => write!(f, "X"),
            CellState::O => write!(f, "O"),
        }
    }
}

struct Player {
    symbol: CellState,
}

struct Game {
    board: [[Tile; BOARD_SIZE]; BOARD_SIZE],
    active_player: usize,
    players: [Player; 2],
}

#[derive(Copy, Clone)]
struct Tile {
    state: CellState,
}

impl Game {
    fn new() -> Game  {
        let game = Game{
            board: [[Tile{
                state: CellState::Empty,
            }; BOARD_SIZE]; BOARD_SIZE],
            players: [Player {
                symbol: CellState::X,
            }, Player {
                symbol: CellState::O,
            }],

            //active_player: players[0],
            active_player: 0,
        };

        game
    }

    fn print(&self) {
        for y in 0..BOARD_SIZE {
            if y==0 {
                print!("   ");
                for i in 0..BOARD_SIZE {
                    print!("{}  ", i+1);
                }
                print!("\n");
            }

            for x in 0..BOARD_SIZE {
                if x == 0 {
                    print!("{}  ", y+1);
                }
                print!("{}  ", self.board[y][x].state);
            }
            print!("\n");
        }
    }

    fn get_input(&mut self) {
        let mut done = false;
        while !done {
            let mut input = String::new();
            println!("[Player \'{}\'] Enter a coordinate:", 
                self.players[self.active_player].symbol);
            io::stdin().read_line(&mut input).unwrap();
            input = String::from(input.trim_end().to_uppercase());

            if !is_input_format_acceptable(&input) {
                println!("Invalid coordinate. Example: x,y like 2,3");
                continue;
            }

            let (x, y) = split_input_coordinate(&input);

            if !self.place_symbol_if_target_cell_available(y-1, x-1) {
                println!("That coordinate is already taken! Try again.");
                continue;
            }

            done = true;
        }
    }

    fn place_symbol_if_target_cell_available(&mut self, x: usize, y: usize) -> bool {
        let mut symbol_was_placed = false;

        if self.board[y][x].state == CellState::Empty {
            self.board[y][x].state = self.players[self.active_player].symbol;
            symbol_was_placed = true;
        }

        symbol_was_placed
    }

    fn take_turn(&mut self) {
        println!();
        println!();
        self.print();
        println!();
        self.get_input();
    }

    fn next_player(&mut self) {
        self.active_player = if self.active_player == 0 { 1 } else { 0 };
    }

    fn is_over(&self) -> bool {
        if self.horizontal_win() || self.vertical_win() || self.diagonal_win() {
            return true;
        }
        
        false
    }

    fn horizontal_win(&self) -> bool {
        let mut to_match: CellState = CellState::Empty;
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if x == 0 {
                    // set the symbol we need to match
                    to_match = self.board[y][x].state;
                    //println!("Horizontal cells need to match \'{}\'", to_match);
                    // ..however, if it's empty, just ignore the rest of the row
                    if to_match == CellState::Empty {
                        //println!("The first horizontal cell is empty.");
                        break;
                    }
                } else if to_match != self.board[y][x].state {
                    // for all the row after the first cell, 
                    // they must match or ignore the rest of the row
                    //println!("{} at {},{} does not match {}", self.board[y][x].state, y,x, to_match);
                    break;
                } else if x==BOARD_SIZE-1 {
                    println!("There is a horizontal win! Row {}", y+1);
                    return true;
                }
            }
        }

        false
    }

    fn vertical_win(&self) -> bool {
        let mut to_match: CellState = CellState::Empty;
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                if y == 0 {
                    // set the symbol we need to match
                    to_match = self.board[y][x].state;
                    //println!("Vertical cells need to match \'{}\'", to_match);
                    // ..however, if it's empty, just ignore the rest of the col
                    if to_match == CellState::Empty {
                        //println!("The first cell is empty.");
                        break;
                    }
                } else if to_match != self.board[y][x].state {
                        // for all the row after the first cell, 
                        // they must match or ignore the rest of the col
                        //println!("{} at {},{} does not match {}", self.board[y][x].state, y,x, to_match);
                        break;
                } else if y==BOARD_SIZE-1 {
                    println!("There is a vertical win! Col {}", x+1);
                    return true;
                }
            }
        }

        false
    }

    fn diagonal_win(&self) -> bool {
        return self.diagonal_win_top_left_to_bottom_right() || self.diagonal_win_top_right_to_bottom_left();
    }

    fn diagonal_win_top_left_to_bottom_right(&self) -> bool {
        let to_match: CellState = self.board[0][0].state;
        if to_match == CellState::Empty {
            return false;
        }

        for pos in 0..BOARD_SIZE {
            if self.board[pos][pos].state != to_match {
                return false;
            }
        }

        println!("There was a diagonal win! (top left to bottom right)");
        true
    }

    // formula: top right (3,1)  x-1,y+1 (2,2)  x-1,y+1 (1,3)
    fn diagonal_win_top_right_to_bottom_left(&self) -> bool {
        let to_match: CellState = self.board[0][BOARD_SIZE-1].state;
        if to_match == CellState::Empty {
            return false;
        }
        
        let mut y:usize = 0;
        for x in (0..BOARD_SIZE).rev() {
            if self.board[y][x].state != to_match {
                return false;
            }
            y += 1;
        }

        println!("There was a diagonal win! (top right to bottom left)");
        true
    }

}

fn is_input_format_acceptable(input: &str) -> bool {
    let re = Regex::new("^[0-3],[0-3]$").unwrap();
    re.is_match(&input)
}

fn split_input_coordinate(input: &str) -> (usize, usize) {
    let input_split = input.split(",");
    let coord_parts: Vec<&str> = input_split.collect();
    let x:usize = coord_parts[0].parse().unwrap();
    let y:usize = coord_parts[1].parse().unwrap();

    (x, y)
}

fn main() {
    let mut game = Game::new();

    let mut done = false;
    while !done {
        game.take_turn();

        if game.is_over() {
            done = true;
            game.print();
        } else {
            game.next_player();
        }
    }
}

#[test]
fn horizontal_win_top() {
    let mut game = Game::new();
    game.board[0][0].state = CellState::O;
    game.board[0][1].state = CellState::O;
    game.board[0][2].state = CellState::O;

    assert_eq!(true, game.horizontal_win());
}

#[test]
fn horizontal_win_middle() {
    let mut game = Game::new();
    game.board[1][0].state = CellState::O;
    game.board[1][1].state = CellState::O;
    game.board[1][2].state = CellState::O;

    assert_eq!(true, game.horizontal_win());
}

#[test]
fn horizontal_win_bottom() {
    let mut game = Game::new();
    game.board[2][0].state = CellState::O;
    game.board[2][1].state = CellState::O;
    game.board[2][2].state = CellState::O;

    assert_eq!(true, game.horizontal_win());
}

#[test]
fn vertical_win_left() {
    let mut game = Game::new();
    game.board[0][0].state = CellState::O;
    game.board[1][0].state = CellState::O;
    game.board[2][0].state = CellState::O;

    assert_eq!(true, game.vertical_win());
}

#[test]
fn vertical_win_middle() {
    let mut game = Game::new();
    game.board[0][1].state = CellState::O;
    game.board[1][1].state = CellState::O;
    game.board[2][1].state = CellState::O;

    assert_eq!(true, game.vertical_win());
}

#[test]
fn vertical_win_right() {
    let mut game = Game::new();
    game.board[0][2].state = CellState::O;
    game.board[1][2].state = CellState::O;
    game.board[2][2].state = CellState::O;

    assert_eq!(true, game.vertical_win());
}

#[test]
fn diagonal_win_top_left_to_bottom_right() {
    let mut game = Game::new();
    game.board[0][0].state = CellState::O;
    game.board[1][1].state = CellState::O;
    game.board[2][2].state = CellState::O;

    assert_eq!(true, game.diagonal_win_top_left_to_bottom_right());
}

#[test]
fn diagonal_win_top_right_to_bottom_left() {
    let mut game = Game::new();
    game.board[0][2].state = CellState::O;
    game.board[1][1].state = CellState::O;
    game.board[2][0].state = CellState::O;

    assert_eq!(true, game.diagonal_win_top_right_to_bottom_left());
}

#[test]
fn active_player_defaults_to_0() {
    let game = Game::new();

    assert_eq!(0, game.active_player);
}

#[test]
fn next_player_switches_from_1_to_0() {
    let mut game = Game::new();
    game.active_player = 0;
    game.next_player();
    assert_eq!(1, game.active_player);
}

#[test]
fn next_player_switches_from_0_to_1() {
    let mut game = Game::new();
    game.active_player = 1;
    game.next_player();
    assert_eq!(0, game.active_player);
}

#[test]
fn place_symbol_if_target_cell_available_should_do_it_if_so() {
    let mut game = Game::new();
    game.active_player = 0;
    game.players[0].symbol = CellState::X;
    let was_available = game.place_symbol_if_target_cell_available(0, 0);
    assert_eq!(CellState::X, game.board[0][0].state);
    assert_eq!(true, was_available);
}

#[test]
fn place_symbol_if_target_cell_available_should_return_false_if_not_available() {
    let mut game = Game::new();
    game.active_player = 0;
    game.players[0].symbol = CellState::X;
    game.board[0][0].state = CellState::O;
    let was_available = game.place_symbol_if_target_cell_available(0, 0);
    assert_eq!(CellState::O, game.board[0][0].state);
    assert_eq!(false, was_available);
}


#[test]
fn is_input_format_acceptable_test() {
    assert_eq!(false, is_input_format_acceptable(""));
    assert_eq!(false, is_input_format_acceptable("alksdjhf"));
    assert_eq!(false, is_input_format_acceptable("x,y"));
    assert_eq!(false, is_input_format_acceptable("1,,4"));
    assert_eq!(false, is_input_format_acceptable("4,1"));
    assert_eq!(true, is_input_format_acceptable("0,0"));
    assert_eq!(true, is_input_format_acceptable("3,3"));
}

#[test]
fn split_input_coordinate_test() {
    assert_eq!((1,2), split_input_coordinate("1,2"));
}