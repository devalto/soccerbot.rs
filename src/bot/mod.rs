extern crate slack;
extern crate regex;

use league;
use self::slack::error::Error;

struct Message {
    text: String,
    channel: String
}

pub struct SoccerBot {
    api_key: String
}

impl SoccerBot {
    pub fn new(api_key: String) -> SoccerBot {
        SoccerBot { api_key: api_key }
    }

    pub fn start(&self) -> Result<(), Error> {
        let mut handler = SoccerBotHandler;
        let mut cli = slack::RtmClient::new(&self.api_key);

        cli.login_and_run::<SoccerBotHandler>(&mut handler)
    }
}

struct SoccerBotHandler;

impl SoccerBotHandler {
    fn get_message_text_from_result(&mut self, event: Result<&slack::Event, slack::Error>) -> Option<Message> {
        match event {
            Ok(event) => self.get_message_text_from_event(&event),
            _ => None
        }
    }

    fn get_message_text_from_event(&mut self, event: &slack::Event) -> Option<Message> {
        match event {
            &slack::Event::Message(ref m) => self.get_message_text_from_message_enum(m),
            _ => None
        }
    }

    fn get_message_text_from_message_enum(&mut self, message: &slack::Message) -> Option<Message> {
        match message {
            &slack::Message::Standard {
                text: ref text,
                ts: _,
                channel: ref channel,
                user: _,
                is_starred: _,
                pinned_to: _,
                reactions: _,
                edited: _,
                attachments: _
            } => Some(Message { channel: channel.clone().unwrap(), text: text.clone().unwrap() }),
            _ => None
        }
    }
}

impl slack::EventHandler for SoccerBotHandler {
    fn on_event(&mut self, cli: &mut slack::RtmClient, event: Result<&slack::Event, slack::Error>, raw_json: &str) {
        let t = self.get_message_text_from_result(event);
        let message = match t {
            Some(m) => m,
            _ => return
        };

        let text = message.text;

        let re = regex::Regex::new(r"^team-of-4").unwrap();
        let r = re.find(&text.to_string());

        let s = match r {
            None => return,
            Some((s, _)) => s
        };

        // We have a match !
        if s == 0 {
            let mut builder = league::tournament::TournamentBuilder::new();
            let players = text.split_whitespace().skip(1);
            for player in players {
                builder.add(player);
            }
            let t = builder.finalize();
            let games = t.create_games();

            if games.len() == 0 {
                let _ = cli.send_message(&message.channel[..], "Not enough players to generate games of 4 players...");
                return
            }

            let mut output = String::new();
            let mut game_counter = 1;
            for game in games {
                if game_counter > 1 {
                    output.push_str(&format!("----\n")[..]);
                }
                output.push_str(&format!(":red_circle: :{}: {} (A)\n", game.red.attack.name, game.red.attack.name)[..]);
                output.push_str(&format!(":red_circle: :{}: {} (D)\n", game.red.defense.name, game.red.defense.name)[..]);
                output.push_str(&format!(":large_blue_circle: :{}: {} (A)\n", game.blue.attack.name, game.blue.attack.name)[..]);
                output.push_str(&format!(":large_blue_circle: :{}: {} (D)\n", game.blue.defense.name, game.blue.defense.name)[..]);
                output.push_str(&format!("\n")[..]);
                game_counter = game_counter + 1;
            }

            let _ = cli.send_message(&message.channel[..], &output[..]);
        }

    }

    fn on_ping(&mut self, cli: &mut slack::RtmClient) {
        //println!("on_ping");
    }

    fn on_close(&mut self, cli: &mut slack::RtmClient) {
        //println!("on_close");
    }

    fn on_connect(&mut self, cli: &mut slack::RtmClient) {
        //println!("on_connect");
        // Do a few things using the api:
        // send a message over the real time api websocket
        //let _ = cli.send_message("#general", "Hello world! (rtm)");
        // post a message as a user to the web api
        //let _ = cli.post_message("#general", "hello world! (postMessage)", None);
        // set a channel topic via the web api
        // let _ = cli.set_topic("#general", "bots rule!");
    }
}

#[cfg(test)]
mod tests {

    use super::SoccerBotHandler;
    use super::slack;

    #[test]
    fn test_get_message_text_from_result_with_ok_result() {
        let e = slack::Event::Message(slack::Message::Standard {
            ts: "date".to_string(),
            channel: Some("#channel".to_string()),
            user: Some("user".to_string()),
            text: Some("a text message".to_string()),
            is_starred: Some(true),
            pinned_to: None,
            reactions: None,
            edited: None,
            attachments: None
        });
        let r: Result<_, slack::error::Error> = Ok(&e);

        let mut handler = SoccerBotHandler;
        let m = handler.get_message_text_from_result(r).unwrap();

        assert_eq!(m.text, "a text message");
        assert_eq!(m.channel, "#channel");
    }

    #[test]
    fn test_get_message_text_from_event_with_any_other_event_returns_null() {
        let e = slack::Event::Hello;

        let mut handler = SoccerBotHandler;
        let m = handler.get_message_text_from_event(&e);

        assert_eq!(m.is_none(), true);
    }

    #[test]
    fn test_get_message_text_from_event_with_message_event() {
        let e = slack::Event::Message(slack::Message::Standard {
            ts: "date".to_string(),
            channel: Some("#channel".to_string()),
            user: Some("user".to_string()),
            text: Some("a text message".to_string()),
            is_starred: Some(true),
            pinned_to: None,
            reactions: None,
            edited: None,
            attachments: None
        });

        let mut handler = SoccerBotHandler;
        let m = handler.get_message_text_from_event(&e).unwrap();

        assert_eq!(m.text, "a text message");
        assert_eq!(m.channel, "#channel");
    }

    #[test]
    fn test_get_message_text_from_message_enum_with_user_normal_message() {
        let mut handler = SoccerBotHandler;
        let msg = slack::Message::Standard {
            ts: "date".to_string(),
            channel: Some("#channel".to_string()),
            user: Some("user".to_string()),
            text: Some("a text message".to_string()),
            is_starred: Some(true),
            pinned_to: None,
            reactions: None,
            edited: None,
            attachments: None
        };

        let m = handler.get_message_text_from_message_enum(&msg).unwrap();

        assert_eq!(m.text, "a text message");
        assert_eq!(m.channel, "#channel");
    }

    #[test]
    fn test_get_message_text_from_message_enum_with_other_message() {
        let mut handler = SoccerBotHandler;
        let msg = slack::Message::MeMessage {
            ts: "date".to_string(),
            channel: "#channel".to_string(),
            user: "user".to_string(),
            text: "a text message".to_string()
        };

        let m = handler.get_message_text_from_message_enum(&msg);

        assert_eq!(m.is_none(), true);
    }
}
