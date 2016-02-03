use super::team::Team;
use super::player::Player;

pub const NUMBER_PLAYER_PER_GAME: u32 = 4;

pub struct Game {
    pub red: Team,
    pub blue: Team
}

impl Game {
    pub fn new(players: [Player; 4]) -> Self {
        let mut vec_players = players.to_vec();
        Game {
            red: Team::new(&mut vec_players),
            blue: Team::new(&mut vec_players)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Game;
    use super::super::player::Player;

    #[test]
    fn test_create_game_from_players_slice() {
        let players = [
            Player::new("player 1".to_string()),
            Player::new("player 2".to_string()),
            Player::new("player 3".to_string()),
            Player::new("player 4".to_string())
        ];

        let game = Game::new(players);

        assert_eq!("player 4".to_string(), game.red.attack.name);
        assert_eq!("player 3".to_string(), game.red.defense.name);
        assert_eq!("player 2".to_string(), game.blue.attack.name);
        assert_eq!("player 1".to_string(), game.blue.defense.name);
    }

}
