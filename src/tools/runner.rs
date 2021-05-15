use super::parser;
use super::super::nysh_builtin::is_builtin;
use super::super::nysh_builtin::builtin_exit;
use super::super::nysh_builtin::builtin_la;
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
            return
        }

        if is_builtin::is_builtin(&self.commands.command) {
            if self.commands.command == "exit" {
                builtin_exit::builtin_exit();
            }
            else if self.commands.command == "la" {
                builtin_la::builtin_la();
            }
            return
        }

        // wait
        let output = Command::new(&self.commands.command)
            .args(&self.commands.args)
            .output()
            .expect("nysh: command not found");
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }
  }