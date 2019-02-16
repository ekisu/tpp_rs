use crate::command_input::CommandInput;
use crate::command_output::CommandOutput;
use crate::renderer::Renderer;

pub struct Control<I: CommandInput, O: CommandOutput, R: Renderer> {
    input: I,
    output: O,
    renderer: R
}

impl<I, O, R> Control<I, O, R>
where
    I: CommandInput,
    O: CommandOutput,
    R: Renderer {
    pub fn new(input: I, output: O, renderer: R) -> Self {
        Control {
            input,
            output,
            renderer
        }
    }

    pub fn run(&mut self) {
        let rx_command = self.input.create_receiver();

        loop {
            let cmd = rx_command.recv().unwrap();
            println!("control: got {:?} command from {}.", cmd.button, cmd.user);
            self.output.emit(cmd.button);
            self.renderer.new_command(cmd);
        }
    }
}