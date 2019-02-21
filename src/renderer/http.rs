use super::Renderer;
use crate::command::Command;
use crate::command_input::Input;
use crate::vote_system::VoteSystem;

use hyper::{Body, Response, StatusCode};

use serde::Serialize;

use gotham::handler::IntoResponse;
use gotham::helpers::http::response::create_response;
use gotham::middleware::state::StateMiddleware;
use gotham::pipeline::single::single_pipeline;
use gotham::pipeline::single_middleware;
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::{FromState, State};
use std::collections::vec_deque::VecDeque;
use std::thread;

use std::sync::{Arc, Mutex};

use stats::Frequencies;

#[derive(Clone, StateData)]
pub struct HTTPRenderer {
    pub last_inputs_vec: Arc<Mutex<VecDeque<Input>>>,
    pub last_vote_system: Arc<Mutex<Option<VoteSystem>>>,
    pub last_vote_system_percentage: Arc<Mutex<Option<f64>>>,
    pub last_vote_system_partial_results: Arc<Mutex<Option<Frequencies<Command>>>>,
    pub last_vote_system_elapsed_time: Arc<Mutex<u64>>,
    pub last_vote_system_change_remaining_secs: Arc<Mutex<u64>>,
}

#[derive(Serialize)]
struct RendererData {
    last_inputs: VecDeque<Input>,
    last_vote_system: Option<VoteSystem>,
    last_vote_system_percentage: Option<f64>,
    last_vote_system_partial_results: Option<Vec<(Command, u64)>>,
    last_vote_system_elapsed_time: u64,
    last_vote_system_change_remaining_secs: u64,
}

impl HTTPRenderer {
    fn response_json<T: ?Sized>(state: &State, s: &T) -> Response<Body>
    where
        T: Serialize,
    {
        create_response(
            state,
            StatusCode::OK,
            mime::APPLICATION_JSON,
            serde_json::to_string(s).unwrap(),
        )
    }

    fn last_inputs(state: State) -> (State, Response<Body>) {
        let response = {
            let this = HTTPRenderer::borrow_from(&state);

            HTTPRenderer::response_json(&state, &*this.last_inputs_vec.lock().unwrap())
        };

        (state, response)
    }

    fn data(state: State) -> (State, Response<Body>) {
        let response = {
            let this = HTTPRenderer::borrow_from(&state);
            let mut _results = this.last_vote_system_partial_results.lock().unwrap();
            let partial = _results.clone().map(|f| {
                f.most_frequent()
                    .iter()
                    .map(|&(k, v)| (k.clone(), v))
                    .collect()
            });

            let renderer_data = RendererData {
                last_inputs: this.last_inputs_vec.lock().unwrap().clone(),
                last_vote_system: this.last_vote_system.lock().unwrap().clone(),
                last_vote_system_percentage: this
                    .last_vote_system_percentage
                    .lock()
                    .unwrap()
                    .clone(),
                last_vote_system_partial_results: partial,
                last_vote_system_elapsed_time: *this.last_vote_system_elapsed_time.lock().unwrap(),
                last_vote_system_change_remaining_secs: *this
                    .last_vote_system_change_remaining_secs
                    .lock()
                    .unwrap(),
            };

            HTTPRenderer::response_json(&state, &renderer_data)
        };

        (state, response)
    }

    fn vote_system(state: State) -> (State, Response<Body>) {
        let response = {
            let this = HTTPRenderer::borrow_from(&state);

            HTTPRenderer::response_json(&state, &*this.last_vote_system.lock().unwrap())
        };

        (state, response)
    }

    fn router(&self) -> Router {
        let middleware = StateMiddleware::new(self.clone());
        let pipeline = single_middleware(middleware);
        let (chain, pipelines) = single_pipeline(pipeline);

        build_router(chain, pipelines, |route| {
            route.get("/last_inputs").to(HTTPRenderer::last_inputs);
            route.get("/vote_system").to(HTTPRenderer::vote_system);
            route.get("/data").to(HTTPRenderer::data);

            route.get("/").to_file("static/index.html");
            route.get("static/*").to_dir("static");
        })
    }

    pub fn new() -> Self {
        HTTPRenderer {
            last_inputs_vec: Arc::new(Mutex::new(VecDeque::new())),
            last_vote_system: Arc::new(Mutex::new(None)),
            last_vote_system_percentage: Arc::new(Mutex::new(None)),
            last_vote_system_partial_results: Arc::new(Mutex::new(None)),
            last_vote_system_elapsed_time: Arc::new(Mutex::new(0)),
            last_vote_system_change_remaining_secs: Arc::new(Mutex::new(0)),
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
        let mut _vec = self.last_inputs_vec.lock().unwrap();
        _vec.push_front(input);
        _vec.truncate(20);
    }

    fn new_command(&mut self, cmd: Command) {}

    fn new_vote_system(&mut self, vote_system: VoteSystem) {
        *self.last_vote_system.lock().unwrap() = Some(vote_system);
    }

    fn new_vote_system_percentage(&mut self, pct: Option<f64>) {
        *self.last_vote_system_percentage.lock().unwrap() = pct;
    }

    fn new_vote_system_democracy_partial_results(&mut self, t: u64, results: Frequencies<Command>) {
        *self.last_vote_system_partial_results.lock().unwrap() = Some(results);
        *self.last_vote_system_elapsed_time.lock().unwrap() = t;
    }

    fn new_vote_system_change_secs_remaining(&mut self, t: u64) {
        *self.last_vote_system_change_remaining_secs.lock().unwrap() = t;
    }
}
