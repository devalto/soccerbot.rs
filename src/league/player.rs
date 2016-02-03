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

#[cfg(test)]
mod tests {

    use super::Player;

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

}
