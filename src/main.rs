#![cfg_attr(feature = "cargo-clippy", allow(clippy::mutex_atomic))]
extern crate gotham;
#[macro_use]
extern crate gotham_derive;
extern crate hyper;
extern crate mime;

extern crate serde_json;

mod command;

mod command_input;
use command_input::{CommandInput, TwitchInput};

mod command_output;
use command_output::KeyboardOutput;

mod renderer;
use renderer::HTTPRenderer;

mod vote_system;
use vote_system::VoteSystem;

mod mediator;
mod vote_counter;
use mediator::Mediator;

mod control;
use control::Control;

mod tpp_config;
use tpp_config::TPPConfig;

use irc::client::prelude::*;

fn main() {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("settings"))
        .unwrap()
        .merge(config::Environment::with_prefix("TPP"))
        .unwrap();

    let tpp_config = settings.try_into::<TPPConfig>().unwrap();

    let twitch_input = TwitchInput::new(tpp_config.username, tpp_config.oauth_token).unwrap();

    let keyboard_output = KeyboardOutput::new();
    let http_renderer = HTTPRenderer::new();
    http_renderer.run_in_background();

    let mediator = Mediator::create(twitch_input, VoteSystem::Anarchy);

    let mut control = Control::new(mediator, keyboard_output, http_renderer);
    control.run();
}
