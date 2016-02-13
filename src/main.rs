extern crate rand;

pub mod league;

use league::tournament::TournamentBuilder;

fn main() {
    let tournament = TournamentBuilder::new()
        .add("sylvain")
        .add("sarah")
        .add("mathieu")
        .add("cédric")
        .add("christian")
        .add("alexandre")
        .add("stéphane")
        .add("françois")
        .add("yannick")
        .add("eugénie")
        .finalize();

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
