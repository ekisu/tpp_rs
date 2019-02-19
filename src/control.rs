use crate::command::Command;
use crate::command_output::CommandOutput;
use crate::vote_system::VoteSystem;
use crate::mediator::{MediatedDecision, MediatorUpdate, MediatorUpdateReceiver};
use crate::renderer::Renderer;

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
                _ => {
                    println!("uhh");
                }
            }
        }
    }

    fn on_vote_system_percentage_change(&mut self, pct: f64) {
        println!("control: got {} VoteSystemPercentageChange", pct);
        self.renderer.new_vote_system_percentage(pct);
    }

    fn on_vote_system_change(&mut self, system: VoteSystem) {
        println!("control: got {:?} VoteSystem", system);
        self.renderer.new_vote_system(system);
    }

    pub fn run(&mut self) {
        loop {
            let update = self.rx_update.recv().unwrap();
            match update {
                MediatorUpdate::Decision(decision) => self.on_decision(decision),
                MediatorUpdate::VoteSystemPercentageChange(p) => self.on_vote_system_percentage_change(p),
                MediatorUpdate::VoteSystemChange(system) => self.on_vote_system_change(system)
            }
        }
    }
}
