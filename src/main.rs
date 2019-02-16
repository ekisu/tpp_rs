mod command;

mod command_input;
use command_input::{TwitchInput, CommandInput};

mod command_output;
use command_output::{KeyboardOutput};

mod control;
use control::Control;

mod tpp_config;
use tpp_config::TPPConfig;

use irc::client::prelude::*;

fn main() {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("settings")).unwrap()
        .merge(config::Environment::with_prefix("TPP")).unwrap();
    
    let tpp_config = settings.try_into::<TPPConfig>().unwrap();

    let twitch_input = TwitchInput::new(
        tpp_config.username,
        tpp_config.oauth_token
    ).unwrap();

    let keyboard_output = KeyboardOutput::new();

    let mut control = Control::new(twitch_input, keyboard_output);
    control.run();
}
