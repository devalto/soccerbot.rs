extern crate rand;

pub mod league;
pub mod bot;

use league::tournament::TournamentBuilder;
use bot::SoccerBot;
use std::env;

fn main() {
    let key = "SLACK_TOKEN";
    let token = match env::var(key) {
        Ok(val) => val,
        Err(e) => {
            println!("SLACK_TOKEN environment variable must be set");
            return
        }
    };

    let bot = SoccerBot::new(token.to_string());
    let _ = bot.start();
}
