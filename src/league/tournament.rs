extern crate rand;

use super::player::Shuffle;
use super::player::Player;
use super::game::Game;
use super::game::NUMBER_PLAYER_PER_GAME;

pub struct Tournament {
    players: Vec<Player>
}

impl Tournament {
    pub fn number_of_games_playable(&self) -> u32 {
        let c = self.players.len() as u32;
        ((c - (c % super::game::NUMBER_PLAYER_PER_GAME)) / super::game::NUMBER_PLAYER_PER_GAME).clone()
    }

    pub fn number_of_player(&self) -> u32 {
        (self.players.len() as u32).clone()
    }

    pub fn create_games(&self) -> Vec<Game> {
        let mut games: Vec<Game> = vec![];

        if self.players.len() < NUMBER_PLAYER_PER_GAME as usize {
            return games;
        }

        let players = self.players.shuffle();
        let chunks = players.chunks(NUMBER_PLAYER_PER_GAME as usize);
        let mut remainders: Vec<Player> = vec![];
        let mut playing: Vec<Player> = vec![];


        for chunk in chunks {
            if chunk.len() == NUMBER_PLAYER_PER_GAME as usize {
                games.push(
                    Game::new([
                        chunk[0].clone(),
                        chunk[1].clone(),
                        chunk[2].clone(),
                        chunk[3].clone()
                    ])
                );
                playing.push(chunk[0].clone());
                playing.push(chunk[1].clone());
                playing.push(chunk[2].clone());
                playing.push(chunk[3].clone());
            } else {
                for player in chunk {
                    remainders.push(player.clone());
                }
            }
        }

        // If remainders is not empty, complete the set to fit inside
        // a game and push a new game in the array.
        if 0 < remainders.len() {
            let reshuffle_playing = playing.shuffle();
            let missing_player_count = NUMBER_PLAYER_PER_GAME as usize - remainders.len();
            for x in 0..missing_player_count {
                remainders.push(reshuffle_playing[x].clone());
            }
            games.push(
                Game::new([
                    remainders[0].clone(),
                    remainders[1].clone(),
                    remainders[2].clone(),
                    remainders[3].clone()
                ])
            );
        }

        games
    }

}

pub struct TournamentBuilder {
    players: Vec<String>
}

impl TournamentBuilder {
    pub fn new() -> TournamentBuilder {
        TournamentBuilder {
            players: vec!()
        }
    }

    pub fn add(&mut self, player_name: &str) -> &mut TournamentBuilder {
        self.players.push(player_name.to_string());
        self
    }

    pub fn finalize(&self) -> Tournament {
        let mut tournament_players = vec![];

        for name in self.players.iter() {
            tournament_players.push(Player { name: name.clone() });
        }

        Tournament { players: tournament_players }
    }
}

#[cfg(test)]
mod tests {
    use super::TournamentBuilder;

    #[test]
    fn test_builder_should_return_empty_tournament() {
        let t = TournamentBuilder::new().finalize();

        let ng = t.number_of_games_playable();
        let np = t.number_of_player();

        assert_eq!(0, ng);
        assert_eq!(0, np);
    }

    #[test]
    fn test_builder_should_return_zero_number_of_game_playable_with_one_player() {
        let t = TournamentBuilder::new()
            .add("sylvain")
            .finalize();

        let ng = t.number_of_games_playable();
        let np = t.number_of_player();

        assert_eq!(0, ng);
        assert_eq!(1, np);
    }

    #[test]
    fn test_builder_should_return_one_number_of_game_playable_with_5_player() {
        let t = TournamentBuilder::new()
            .add("sylvain")
            .add("sarah")
            .add("mathieu")
            .add("cedric")
            .add("christian")
            .finalize();

        let ng = t.number_of_games_playable();
        let np = t.number_of_player();

        assert_eq!(1, ng);
        assert_eq!(5, np);
    }

    #[test]
    fn test_create_games_with_remainders_complete() {
        let t = TournamentBuilder::new()
            .add("sylvain")
            .add("sarah")
            .add("mathieu")
            .add("cedric")
            .add("christian")
            .add("eugénie")
            .finalize();
        let games = t.create_games();

        assert_eq!(2, games.len());
    }

    #[test]
    fn test_create_games_with_not_enough_players() {
        let t = TournamentBuilder::new()
            .add("sylvain")
            .add("sarah")
            .add("mathieu")
            .finalize();
        let games = t.create_games();

        assert_eq!(0, games.len());
    }
}
