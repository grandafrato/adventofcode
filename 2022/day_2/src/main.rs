use std::env;
use std::fs;

use crate::game::Game;
use crate::round::Rps;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_data = get_file_data(&args[1]);

    let first_puzzle_moves = letters_are_moves(&file_data);
    let second_puzzle_moves = letters_are_results(file_data);

    let game1 = game_from_moves(first_puzzle_moves);
    let game2 = game_from_moves(second_puzzle_moves);

    println!(
        "If the letters are moves, {:?} is the final score!\n\n",
        game1.my_plays //game1.get_score()
    );
    println!(
        "If the letters are intended results, {:?} is the final score!",
        game2.oponent_plays
    );
}

fn get_file_data(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

enum GameMove {
    MyMove(Rps),
    OponentMove(Rps),
}

fn letters_are_moves(data: &str) -> Vec<GameMove> {
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

fn letters_are_results(data: String) -> Vec<GameMove> {
    let mut moves = Vec::new();
    let characters = data.split_whitespace();
    let characters_skipped = data.split_whitespace().skip(1);

    for (oponent_character, my_character) in characters.zip(characters_skipped).step_by(2) {
        let oponent_rps = interperet_oponent(oponent_character).unwrap();
        moves.push(GameMove::OponentMove(oponent_rps));

        match my_character {
            "X" => moves.push(GameMove::MyMove(round::rps_to_lose(oponent_rps))),
            "Y" => moves.push(GameMove::MyMove(oponent_rps)),
            "Z" => moves.push(GameMove::MyMove(round::rps_to_win(oponent_rps))),
            _ => (),
        }
    }

    moves
}

fn interperet_oponent(character: &str) -> Option<Rps> {
    match character {
        "A" => Some(Rps::Rock),
        "B" => Some(Rps::Paper),
        "C" => Some(Rps::Scissors),
        _ => None,
    }
}

fn game_from_moves(moves: Vec<GameMove>) -> Game {
    let mut my_plays = Vec::new();
    let mut oponent_plays = Vec::new();

    for game_move in moves.into_iter() {
        match game_move {
            GameMove::OponentMove(play) => oponent_plays.push(play),
            GameMove::MyMove(play) => my_plays.push(play),
        }
    }

    Game::new(my_plays, oponent_plays)
}

mod game;
mod round;
