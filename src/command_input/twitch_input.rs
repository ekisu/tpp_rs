use super::{CommandInput, Input};
use crate::command::{Button, Command};
use irc::client::prelude::IrcClient;
use irc::error::IrcError;
use std::default::Default;
use std::sync::mpsc::{channel, Receiver};
use std::thread;

pub struct TwitchInput {
    client: IrcClient,
}

impl TwitchInput {
    pub fn new(username: String, oauth_token: String) -> Result<Self, IrcError> {
        use irc::client::prelude::*;

        let config = Config {
            nickname: Some(username.clone()),
            server: Some("irc.chat.twitch.tv".to_owned()),
            port: Some(6697),
            password: Some(oauth_token),
            channels: Some(vec![format!("#{}", username).to_owned()]),
            use_ssl: Some(true),
            ..Default::default()
        };

        let client = IrcClient::from_config(config)?;
        client.identify()?;

        Ok(TwitchInput { client })
    }
}

impl CommandInput for TwitchInput {
    fn create_receiver(&self) -> Receiver<Input> {
        use irc::client::prelude::Command as IrcCommand;
        use irc::client::Client;

        let (tx, rx) = channel();
        let thread_client = self.client.clone();
        thread::spawn(move || {
            thread_client
                .for_each_incoming(|message| {
                    println!("twitch_input: {:?}", message);

                    if let IrcCommand::PRIVMSG(ref _target, ref msg) = message.command {
                        if let Some(command) = Command::from_string(msg.to_owned()) {
                            let user = message
                                .source_nickname()
                                .unwrap_or("unknown user")
                                .to_owned();
                            println!("twitch_input: got {:?} command, from {}.", command, user);

                            tx.send(Input(command, user)).unwrap();
                        }
                    }
                })
                .unwrap();
        });

        rx
    }
}
