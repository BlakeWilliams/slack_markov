extern crate hyper;
extern crate slack_api;

use self::hyper::Client;
use self::slack_api::search;

pub fn all_messages<F>(token: &str, username: &str, mut handler: F)
    where F: FnMut(&str) {
    let client = Client::new();
    let query = format!("from:@{}", username);
    let mut page = 0;

    loop {
        let result = search::messages(
            &client,
            &token,
            &query,
            None,
            None,
            None,
            Some(1000),
            Some(page)
        );

        match result {
            Ok(search::MessagesResponse { messages, .. }) => {
                let paging = messages.paging;
                println!("Importing page {} of {}", paging.page, paging.pages);

                for message in messages.matches {
                    handler(&message.text);
                }

                if page == paging.pages {
                    break;
                } else {
                    page = paging.page + 1;
                }
            }
            Err(e) => panic!(e)
        }
    }
}
