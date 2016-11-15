# Slack Markov

This is a Rust program that connects to Slack, uses the Slack message search API
to find all messages that a user has posted them and to add them to a [markov
chain]. After the messages are parsed the program connects to Slack as a bot
user and responds to `@` mentions to it with a phrase generated from the markov
chain.

[markov chain]: https://en.wikipedia.org/wiki/Markov_chain

## Building

On OS X OpenSSL may not compile without exporting some environment variables. If
you run into issues with it you can run the following:

```sh
export OPENSSL_INCLUDE_DIR=/usr/local/opt/openssl/include
export OPENSSL_LIB_DIR=/usr/local/opt/openssl/lib
export DEP_OPENSSL_INCLUDE=/usr/local/opt/openssl/include
```

Make sure the program builds by running `cargo build`.

## Running

The bot expects 3 environment variables to be present:

* `BOT_TOKEN` - Token for the bot integration
* `API_TOKEN` - Token that gives access to the search API. You can get this
  token from [here](https://api.slack.com/docs/oauth-test-tokens)
* `SLACK_USERNAME` - Username of whose messages you want input into the markov
  chain

With these environment variables set you can run `cargo run --release`
