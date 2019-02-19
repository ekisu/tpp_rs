use super::Renderer;
use crate::command::Command;
use crate::vote_system::VoteSystem;
use crate::command_input::Input;

use hyper::{Body, Response, StatusCode};

use gotham::handler::IntoResponse;
use gotham::helpers::http::response::create_response;
use gotham::middleware::state::StateMiddleware;
use gotham::pipeline::single::single_pipeline;
use gotham::pipeline::single_middleware;
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::{FromState, State};
use std::thread;

use std::sync::{Arc, Mutex};

#[derive(Clone, StateData)]
pub struct HTTPRenderer {
    pub last_inputs_vec: Arc<Mutex<Vec<Input>>>,
    pub last_vote_system: Arc<Mutex<Option<VoteSystem>>>,
    pub last_vote_system_percentage: Arc<Mutex<Option<f64>>>,
}

impl HTTPRenderer {
    fn last_inputs(state: State) -> (State, Response<Body>) {
        let response = {
            let this = HTTPRenderer::borrow_from(&state);

            create_response(
                &state,
                StatusCode::OK,
                mime::APPLICATION_JSON,
                serde_json::to_string(&*this.last_inputs_vec.lock().unwrap()).unwrap(),
            )
        };

        (state, response)
    }

    fn vote_system(state: State) -> (State, Response<Body>) {
        let response = {
            let this = HTTPRenderer::borrow_from(&state);

            create_response(
                &state,
                StatusCode::OK,
                mime::APPLICATION_JSON,
                serde_json::to_string(&*this.last_vote_system.lock().unwrap()).unwrap(),
            )
        };

        (state, response)
    }

    fn router(&self) -> Router {
        let middleware = StateMiddleware::new(self.clone());
        let pipeline = single_middleware(middleware);
        let (chain, pipelines) = single_pipeline(pipeline);

        build_router(chain, pipelines, |route| {
            route.get("/last_inputs").to(HTTPRenderer::last_inputs);
            route
                .get("/vote_system")
                .to(HTTPRenderer::vote_system);

            route.get("/").to_file("static/index.html");
            route.get("static/*").to_dir("static");
        })
    }

    pub fn new() -> Self {
        HTTPRenderer {
            last_inputs_vec: Arc::new(Mutex::new(Vec::new())),
            last_vote_system: Arc::new(Mutex::new(None)),
            last_vote_system_percentage: Arc::new(Mutex::new(None)),
        }
    }

    pub fn run_in_background(&self) {
        let routes = self.router();

        thread::spawn(move || {
            gotham::start("127.0.0.1:8080", routes);
        });
    }
}

impl Renderer for HTTPRenderer {
    fn new_input(&mut self, input: Input) {
        self.last_inputs_vec.lock().unwrap().push(input);
    }

    fn new_command(&mut self, cmd: Command) {}

    fn new_vote_system(&mut self, vote_system: VoteSystem) {
        *self.last_vote_system.lock().unwrap() = Some(vote_system);
    }

    fn new_vote_system_percentage(&mut self, pct: f64) {
        *self.last_vote_system_percentage.lock().unwrap() = Some(pct);
    }
}
