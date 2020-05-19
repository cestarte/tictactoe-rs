// use std::fmt;
extern crate game;
//pub mod game::components;
//use game::components::Game;
use game::components::Game;
use game::components::BOARD_SIZE;
mod input;


fn main() {
    let mut game = Game::new();

    let mut done = false;
    while !done {
        take_turn(&mut game);

        if game.is_over() {
            done = true;
            print_board(&game);
        } else {
            game.next_player();
        }
    }
}


fn print_board(g: &Game) {
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
            print!("{}  ", g.board[y][x].state);
        }
        print!("\n");
    }
}

fn take_turn(g: &mut Game) {
    println!();
    println!();
    print_board(&g);
    println!();
    input::get_input(&mut g);
}