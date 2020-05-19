use std::io;
use regex::Regex;
extern crate game;
use game::components::Game;
//use game::components::Game;

pub fn get_input(mut g: &mut Game) {
    let mut done = false;
    while !done {
        let mut input = String::new();
        println!("[Player \'{}\'] Enter a coordinate:", 
            g.players[g.active_player].symbol);
        io::stdin().read_line(&mut input).unwrap();
        input = String::from(input.trim_end().to_uppercase());

        if !is_input_format_acceptable(&input) {
            println!("Invalid coordinate. Example: x,y like 2,3");
            continue;
        }

        let (x, y) = split_input_coordinate(&input);

        if !g.place_symbol_if_target_cell_available(y-1, x-1) {
            println!("That coordinate is already taken! Try again.");
            continue;
        }

        done = true;
    }
}

pub fn is_input_format_acceptable(input: &str) -> bool {
    let re = Regex::new("^[0-3],[0-3]$").unwrap();
    re.is_match(&input)
}

pub fn split_input_coordinate(input: &str) -> (usize, usize) {
    let input_split = input.split(",");
    let coord_parts: Vec<&str> = input_split.collect();
    let x:usize = coord_parts[0].parse().unwrap();
    let y:usize = coord_parts[1].parse().unwrap();

    (x, y)
}