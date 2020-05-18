use std::fmt;
use std::io;
use regex::Regex;

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
    active_player: usize,
    players: [Player; 2],
}

#[derive(Copy, Clone)]
struct Tile {
    state: CellState,
}

impl Game {
    fn new() -> Game  {
        let mut game = Game{
            board: [[Tile{
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

    fn get_input(&mut self) {
        let mut done = false;
        while !done {
            let mut input = String::new();
            println!("[Player {} - {}] Enter a coordinate.", 
                self.players[self.active_player].id, self.players[self.active_player].symbol);
            io::stdin().read_line(&mut input).unwrap();
            input = String::from(input.trim_end().to_uppercase());

            // was the input a valid coordinate?
            let re = Regex::new("^[0-3],[0-3]$").unwrap();
            if !re.is_match(&input) {
                println!("Invalid coordinate. Example: 1,1 or 2,3");
                continue;
            }

            // parse x & y
            let input_split = input.split(",");
            let coord_parts: Vec<&str> = input_split.collect();
            let x:usize = coord_parts[0].parse().unwrap();
            let y:usize = coord_parts[1].parse().unwrap();

            // check if cell is empty
            match self.board[x-1][y-1].state {
                CellState::Empty => self.board[x-1][y-1].state = self.players[self.active_player].symbol,
                _ => {
                    println!("That coordinate already is already taken! Try again.");
                    continue;
                }
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
