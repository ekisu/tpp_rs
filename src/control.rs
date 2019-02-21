use crate::command::Command;
use crate::command_input::Input;
use crate::command_output::CommandOutput;
use crate::mediator::{MediatedDecision, MediatorUpdate, MediatorUpdateReceiver};
use crate::renderer::Renderer;
use crate::vote_system::VoteSystem;
use stats::Frequencies;

pub struct Control<O: CommandOutput, R: Renderer> {
    rx_update: MediatorUpdateReceiver,
    output: O,
    renderer: R,
}

impl<O, R> Control<O, R>
where
    O: CommandOutput,
    R: Renderer,
{
    pub fn new(rx_update: MediatorUpdateReceiver, output: O, renderer: R) -> Self {
        Control {
            rx_update,
            output,
            renderer,
        }
    }

    fn on_decision(&mut self, decision: MediatedDecision) {
        match decision {
            MediatedDecision::Command(cmd) => match cmd {
                Command::Action(button) => {
                    println!("control: got {:?} command from Mediator.", button);
                    self.output.emit(button);
                    self.renderer.new_command(cmd);
                }
                x => {
                    unreachable!(format!(
                        "on_decision: decision should always be a Command, but it was {:?}",
                        x
                    ));
                }
            },
        }
    }

    fn on_vote_system_percentage_change(&mut self, pct: Option<f64>) {
        println!("control: got {:?} VoteSystemPercentageChange", pct);
        self.renderer.new_vote_system_percentage(pct);
    }

    fn on_vote_system_change(&mut self, system: VoteSystem) {
        println!("control: got {:?} VoteSystem", system);
        self.renderer.new_vote_system(system);
    }

    fn on_vote_system_partial_results(&mut self, t: u64, results: Frequencies<Command>) {
        self.renderer
            .new_vote_system_democracy_partial_results(t, results);
    }

    fn on_vote_system_change_secs_remaining(&mut self, t: u64) {
        self.renderer.new_vote_system_change_secs_remaining(t);
    }

    fn on_input(&mut self, input: Input) {
        println!("control: got {:?} Input", input);
        self.renderer.new_input(input);
    }

    pub fn run(&mut self) {
        loop {
            use crate::mediator::MediatorUpdate::*;

            let update = self.rx_update.recv().unwrap();
            match update {
                Decision(decision) => self.on_decision(decision),
                VoteSystemPercentageChange(p) => self.on_vote_system_percentage_change(p),
                VoteSystemChange(system) => self.on_vote_system_change(system),
                VoteSystemChangeSecsRemaining(secs) => self.on_vote_system_change_secs_remaining(secs),
                VoteSystemDemocracyPartialResults(t, partial) => {
                    self.on_vote_system_partial_results(t, partial)
                }
                Input(input) => self.on_input(input),
            }
        }
    }
}
