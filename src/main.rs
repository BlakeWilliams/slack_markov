extern crate slack;

mod bot;
mod chain;
mod messages;

use std::env;
use chain::Chain;

fn main() {
    let bot_token = env::var("BOT_TOKEN").expect("BOT_TOKEN not in env");
    let api_token = env::var("API_TOKEN").expect("API_TOKEN not in env");
    let username = env::var("SLACK_USERNAME").expect("SLACK_USERNAME not in env");

    let mut chain = Chain::new(2);

    messages::all_messages(&api_token, &username, |message| {
        chain.parse(message);
    });

    let mut handler = bot::Bot::new(chain);
    let mut cli = slack::RtmClient::new(&bot_token);

    match cli.login_and_run::<bot::Bot>(&mut handler) {
        Ok(_) => println!("Connecting to Slack..."),
        Err(err) => panic!("Could not connect to slack: {}", err),
    }
}
