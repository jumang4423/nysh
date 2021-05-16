use super::super::nysh_builtin::builtin_cd;
use super::super::nysh_builtin::builtin_dotdot;
use super::super::nysh_builtin::builtin_exit;
use super::super::nysh_builtin::builtin_la;
use super::super::nysh_builtin::builtin_help;
use super::super::nysh_builtin::is_builtin;
use super::parser;
// command line tools
use colored::*;
use std::process::Command;

pub struct CommandRunner {
    pub commands: parser::CommandParser,
}

impl CommandRunner {
    pub fn constructor(command_object: parser::CommandParser) -> Result<Self, ()> {
        Ok(Self {
            commands: command_object,
        })
    }

    pub fn run_command(&mut self) {
        if self.commands.command == "" {
            return;
        }

        if is_builtin::is_builtin(&self.commands.command) {
            let _com: &str = &self.commands.command;
            match _com {
                "exit" => {
                    builtin_exit::builtin_exit();
                }
                "la" => {
                    builtin_la::builtin_la();
                }
                "cd" => {
                    match builtin_cd::builtin_cd(&self.commands) {
                        Ok(()) => println!(" ↓"),
                        Err(d) => println!("-! {}", d.red()),
                    };
                    // TODO make cd command
                }
                ".." => {
                    builtin_dotdot::builtin_dotdot(&mut self.commands);
                    match builtin_cd::builtin_cd(&self.commands) {
                        Ok(()) => println!(" ↓"),
                        Err(d) => println!("-! {}", d.red()),
                    };
                    // TODO make cd command
                }
                "help" => {
                    
                    match builtin_help::builtin_help() {
                        Ok(()) => return,
                        Err(d) => println!("-! {}", d.red()),
                    };
                }
                _ => return,
            }

            return;
        }

        // wait
        let mut _output = Command::new(&self.commands.command)
            .args(&self.commands.args)
            .output();
        match _output {
            Ok(d) => print!("{}", String::from_utf8_lossy(&d.stdout)),
            Err(_) => println!("-! {} ", "ops! command not found...".red()),
        }
    }
}
