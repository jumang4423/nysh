use super::parser;
// command line tools
use std::process::Command;

pub struct CommandRunner {
    pub commands: parser::CommandParser
  }
  
  impl CommandRunner {
    pub fn constructor(command_object: parser::CommandParser) -> Self {
      Self {
        commands: command_object,
      }
    }

    pub fn run_command(&mut self) {


        if self.commands.command == "" {
            println!("owari");
        }

        // wait
        let output = Command::new(&self.commands.command)
            .args(&self.commands.args)
            .output()
            .expect("nysh: command not found");
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }
  }