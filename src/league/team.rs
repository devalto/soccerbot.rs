use super::player::Player;

#[derive(Debug)]
pub struct TeamOfTwo {
    pub attack: Player,
    pub defense: Player
}

impl TeamOfTwo {
    pub fn new(players: &mut Vec<Player>) -> TeamOfTwo {
        TeamOfTwo {
            attack: players.pop().unwrap(),
            defense: players.pop().unwrap()
        }
    }
}

pub struct TeamOfOne {
    pub player: Player
}

impl TeamOfOne {
    pub fn new(players: &mut Vec<Player>) -> TeamOfOne {
        TeamOfOne {
            player: players.pop().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {

    use super::TeamOfTwo;
    use super::super::player::Player;

    #[test]
    fn test_build_team() {
        let mut players = vec![
            Player::new("player 1".to_string()),
            Player::new("player 2".to_string()),
            Player::new("player 3".to_string())
        ];

        let team = TeamOfTwo::new(&mut players);

        assert_eq!("player 3".to_string(), team.attack.name);
        assert_eq!("player 2".to_string(), team.defense.name);

        assert_eq!(1, players.len());
    }
}
