extern crate slack;

use self::slack::Event;
use self::slack::Message;

pub struct Bot {
    chain: super::chain::Chain,
}

impl Bot {
    pub fn new(chain: super::chain::Chain) -> Bot {
        Bot {
            chain: chain
        }
    }
}

#[allow(unused_variables)]
impl slack::EventHandler for Bot {
    #[allow(unused_assignments)]
    fn on_event(&mut self, cli: &mut slack::RtmClient, event: Result<slack::Event, slack::Error>, raw_json: &str) {
        let user_id = cli.get_id().expect("could not get slack id");

        match event.unwrap() {
            Event::Message(Message::Standard { text, channel, .. }) => {
                let channel = channel.unwrap();
                let text = text.unwrap();

                if text.starts_with(&format!("<@{}>", user_id)) {
                    let message = self.chain.sentence();
                    let _ = cli.send_message(&channel, &message);
                }
            }
            _ => ()
        }
    }

    fn on_ping(&mut self, cli: &mut slack::RtmClient) {}
    fn on_close(&mut self, cli: &mut slack::RtmClient) {}
    fn on_connect(&mut self, cli: &mut slack::RtmClient) {}
}
