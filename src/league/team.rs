use super::player::Player;

pub struct Team {
    pub attack: Player,
    pub defense: Player
}

impl Team {
    pub fn new(players: &mut Vec<Player>) -> Team {
        Team {
            attack: players.pop().unwrap(),
            defense: players.pop().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Team;
    use super::super::player::Player;

    #[test]
    fn test_build_team() {
        let mut players = vec![
            Player::new("player 1".to_string()),
            Player::new("player 2".to_string()),
            Player::new("player 3".to_string())
        ];

        let team = Team::new(&mut players);

        assert_eq!("player 3".to_string(), team.attack.name);
        assert_eq!("player 2".to_string(), team.defense.name);

        assert_eq!(1, players.len());
    }
}
