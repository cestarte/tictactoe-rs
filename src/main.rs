use std::fmt;
use std::io;
//use std::io::prelude::*;

const BOARD_SIZE: usize = 3;

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

struct Player {
    id: u8,
    symbol: CellState,
}

struct Game {
    board: [[Tile; BOARD_SIZE]; BOARD_SIZE],
    //active_player: Player,
    active_player: usize,
    players: [Player; 2],
}

#[derive(Copy, Clone)]
struct Tile {
    row: u8,
    col: u8,
    state: CellState,
}

impl Game {
    fn new() -> Game  {
        let mut game = Game{
            board: [[Tile{
                row: 0,
                col: 0,
                state: CellState::Empty,
            }; BOARD_SIZE]; BOARD_SIZE],
            players: [Player {
                id: 1,
                symbol: CellState::X,
            }, Player {
                id: 2,
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

    fn get_input(&self) {
        let mut input = String::new();
        let mut done = false;
        while !done {
            println!("Player {} ({}), enter a coordinate.", 
            //    self.active_player.id, self.active_player.symbol);
                self.players[self.active_player].id, self.players[self.active_player].symbol);
            io::stdin().read_line(&mut input).unwrap();
            input = String::from(input.trim_end().to_uppercase());
            println!("input was \"{}\"", input);

            // need to get coordinate

            // check if cell is empty

            // add player's response to board

            match input.as_str() {
                "X" => println!("X"),
                "O" => println!("O"),
                _ => println!("Invalid coordinate. Example: 1,1 or 2,3"),
            }

            done = true;
        }
    }

    fn take_turn(&mut self) {
        self.print();
        self.get_input();
    }

    fn next_player(&mut self) {
        self.active_player = if self.active_player == 0 { 1 } else { 0 };
        //self.active_player = if self.active_player.id == self.players[0].id { self.players[1] } else { self.players[0] };
    }

    fn is_over(&self) -> bool {
        // TODO check for winner
        //let filtered = self.board.filter(|&filteredTiles| filteredTiles.CellState == CellState::O);
        //    false
        
        true
    }
}


fn main() {
    let mut game = Game::new();
    //game.print();

    let mut done = false;
    while !done {
        game.take_turn();

        if game.is_over() {
            done = true;
        } else {
            game.next_player();
        }
    }
    
}
