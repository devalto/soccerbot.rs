extern crate rand;

pub mod league;

use league::tournament::TournamentBuilder;

fn main() {
    let tournament = TournamentBuilder::new()
        .add("alexandre")
        .add("yannick")
        .add("sarah")
        .add("nain")
        .add("eugénie")
        .add("alexandre")
        .add("yannick")
        .add("sarah")
        .add("nain")
        .add("eugénie")
        .finalize();

    println!("{:?}", tournament.number_of_games_playable());
}
