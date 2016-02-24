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

        let re = regex::Regex::new(r"^team-generator-4").unwrap();
        let (s,_) = re.find(&text.to_string()).unwrap();

        // We have a match !
        if s == 0 {
            let mut builder = league::tournament::TournamentBuilder::new();
            let players = text.split_whitespace().skip(1);
            for player in players {
                builder.add(player);
            }
            let t = builder.finalize();
            let games = t.create_games();

            let mut output = String::new();
            let mut game_counter = 1;
            for game in games {
                output.push_str(&format!("Game {}\n", game_counter)[..]);
                output.push_str(&format!("------\n")[..]);
                output.push_str(&format!("\n")[..]);
                output.push_str(&format!("Red team\n")[..]);
                output.push_str(&format!("Attack: {}\n", game.red.attack.name)[..]);
                output.push_str(&format!("Defense: {}\n", game.red.defense.name)[..]);
                output.push_str(&format!("\n")[..]);
                output.push_str(&format!("Blue team\n")[..]);
                output.push_str(&format!("Attack: {}\n", game.blue.attack.name)[..]);
                output.push_str(&format!("Defense: {}\n", game.blue.defense.name)[..]);
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
