extern crate rand;

pub mod league;

use league::tournament::TournamentBuilder;
use std::io;

fn main() {
    let mut builder = TournamentBuilder::new();

    let mut input = String::new();
    let stdin = io::stdin();

    while stdin.read_line(&mut input).unwrap() > 0 {
        builder.add(&input.trim());

        input.clear();
    }

    let tournament = builder.finalize();

    let games = tournament.create_games();
    let mut game_counter = 1;
    for game in games {
        println!("Game {}", game_counter);
        println!("------");
        println!("");
        println!("Red team");
        println!("Attack: {}", game.red.attack.name);
        println!("Defense: {}", game.red.defense.name);
        println!("");
        println!("Blue team");
        println!("Attack: {}", game.blue.attack.name);
        println!("Defense: {}", game.blue.defense.name);
        println!("");

        game_counter = game_counter + 1;
    }

}
