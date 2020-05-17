use std::fmt;
use std::io;
//use std::io::prelude::*;

const BOARD_WIDTH: usize = 3;
const BOARD_HEIGHT: usize = 3;

#[derive(Copy, Clone)]
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


struct Game {
    board: [[Tile; BOARD_WIDTH]; BOARD_HEIGHT]
}

#[derive(Copy, Clone)]
struct Tile {
    row: u8,
    col: u8,
    state: CellState,
    active_player: u8,
}

impl Game {
    fn new() -> Game  {
        let mut game = Game{
            board: [[Tile{
                row: 0,
                col: 0,
                state: CellState::Empty,
                active_player: 1,
            }; BOARD_WIDTH]; BOARD_HEIGHT]
        };

        game
    }

    fn print(&self) {
        for y in 0..BOARD_HEIGHT {
            if y==0 {
                print!("   ");
                for i in 0..BOARD_WIDTH {
                    print!("{}  ", i+1);
                }
                print!("\n");
            }

            for x in 0..BOARD_WIDTH {
                if x == 0 {
                    print!("{}  ", y+1);
                }
                print!("{}  ", self.board[y][x].state);
            }
            print!("\n");
        }
    }

    fn get_input(&self) {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();//.expect("Failed to read user input");
        println!("input was \"{}\"", input.trim_end());
        match input.as_str() {
            "X" => println!("X"),
            "O" => println!("O"),
            _ => println!("unknown"),
        }
    }

    fn is_over(&self) -> bool {
        // TODO check for winner
        //let filtered = self.board.filter(|&filteredTiles| filteredTiles.CellState == CellState::O);
        //    false
        
        true
    }
}


fn main() {
    let game = Game::new();
    //game.print();

    let mut done = false;
    while !done {
        game.print();
        game.get_input();

        if game.is_over() {
            done = true;
        }
    }
    
}
