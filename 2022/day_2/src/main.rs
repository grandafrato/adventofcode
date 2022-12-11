use std::env;
use std::fs;

use crate::game::Game;
use crate::round::Rps;

fn main() {
    let args: Vec<String> = env::args().collect();

    let moves = parse_file_data(get_file_data(&args[1]));

    let game = game_from_moves(moves);

    println!(
        "Using all the moves, {} is the final score!",
        game.get_score()
    );
}

fn get_file_data(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

enum GameMove {
    MyMove(Rps),
    OponentMove(Rps),
}

fn parse_file_data(data: String) -> Vec<GameMove> {
    let mut moves = Vec::new();

    for character in data.split_whitespace() {
        match character {
            "A" => moves.push(GameMove::OponentMove(Rps::Rock)),
            "B" => moves.push(GameMove::OponentMove(Rps::Paper)),
            "C" => moves.push(GameMove::OponentMove(Rps::Scissors)),
            "X" => moves.push(GameMove::MyMove(Rps::Rock)),
            "Y" => moves.push(GameMove::MyMove(Rps::Paper)),
            "Z" => moves.push(GameMove::MyMove(Rps::Scissors)),
            _ => (),
        }
    }

    moves
}

fn game_from_moves(moves: Vec<GameMove>) -> Game {
    let mut game = Game::new();
    for game_move in moves.into_iter() {
        match game_move {
            GameMove::OponentMove(play) => game.add_oponent_play(play),
            GameMove::MyMove(play) => game.add_my_play(play),
        }
    }

    game
}

mod game;
mod round;
