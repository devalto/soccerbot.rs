
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Player {
    pub name: String
}

impl Player {
    pub fn new(player_name: String) -> Self {
        Player {
            name: player_name
        }
    }
}

impl Clone for Player {
    fn clone(&self) -> Player {
        Player {
            name: self.name.clone()
        }
    }
}

pub trait Shuffle {
    fn shuffle(&self) -> Self;
}

impl Shuffle for Vec<Player> {
    fn shuffle(&self) -> Vec<Player> {
        let mut players = self.clone().into_boxed_slice();
        let mut rng = thread_rng();
        rng.shuffle(&mut players);

        let mut vector_players: Vec<Player> = vec![];
        for player in players.into_iter() {
            vector_players.push(player.clone());
        }

        vector_players
    }
}

#[cfg(test)]
mod tests {

    use super::Player;
    use super::Shuffle;

    #[test]
    fn test_create_player_with_name() {
        let player = Player::new("player name".to_string());

        assert_eq!("player name".to_string(), player.name);
    }

    #[test]
    fn test_clone_player_has_same_name() {
        let player = Player {
            name: "name".to_string()
        };

        let cloned = player.clone();

        assert_eq!("name".to_string(), cloned.name);
    }

    #[test]
    fn test_shuffle() {
        let players: Vec<Player> = vec![
            Player::new("player name".to_string()),
            Player::new("player name 2".to_string())
        ];

        let shuff: Vec<Player> = players.shuffle();

        return
    }

}
