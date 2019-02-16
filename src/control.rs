use crate::command_input::CommandInput;
use crate::command_output::CommandOutput;

pub struct Control<I: CommandInput, O: CommandOutput> {
    input: I,
    output: O
}

impl<I, O> Control<I, O>
where
    I: CommandInput,
    O: CommandOutput {
    pub fn new(input: I, output: O) -> Self {
        Control {
            input,
            output
        }
    }

    pub fn run(&mut self) {
        let rx_command = self.input.create_receiver();

        loop {
            let cmd = rx_command.recv().unwrap();
            println!("control: got {:?} command.", cmd);
            self.output.emit(cmd);
        }
    }
}