use super::Renderer;
use crate::command::Command;

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
    pub last_commands_vec: Arc<Mutex<Vec<Command>>>
}

impl HTTPRenderer {
    fn last_commands(state: State) -> (State, Response<Body>) {
        let response = {
            let this = HTTPRenderer::borrow_from(&state);

            create_response(
                &state,
                StatusCode::OK,
                mime::APPLICATION_JSON,
                serde_json::to_string(&*this.last_commands_vec.lock().unwrap()).unwrap(),
            )
        };

        (state, response)
    }

    fn router(&self) -> Router {
        let middleware = StateMiddleware::new(self.clone());
        let pipeline = single_middleware(middleware);
        let (chain, pipelines) = single_pipeline(pipeline);

        build_router(chain, pipelines, |route| {
            route.get("/last_commands").to(HTTPRenderer::last_commands);

            route.get("/").to_file("static/index.html");
            route.get("static/*").to_dir("static");
        })
    }

    pub fn new() -> Self {
        HTTPRenderer {
            last_commands_vec: Arc::new(Mutex::new(Vec::new()))
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
    fn new_command(&mut self, command: Command) {
        self.last_commands_vec.lock().unwrap().push(command);
    }
}
