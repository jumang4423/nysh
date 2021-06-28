use super::super::nysh_builtin::builtin_cd;
use super::super::nysh_builtin::builtin_dotdot;
use super::super::nysh_builtin::builtin_exit;
use super::super::nysh_builtin::builtin_help;
use super::super::nysh_builtin::builtin_la;
use super::super::nysh_builtin::is_builtin;
use super::parser;
// command line tools
use colored::*;
// use std::process::Command;

// tokio runner
use futures::prelude::*;
use std::process::Stdio;
use tokio::process::Command;
use tokio_util::codec::{FramedRead, LinesCodec};

pub struct CommandRunner {
    pub commands: parser::CommandParser,
}

impl CommandRunner {
    pub fn constructor(command_object: parser::CommandParser) -> Result<Self, ()> {
        Ok(Self {
            commands: command_object,
        })
    }

    pub async fn run_command(&mut self) {
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
                }
                ".." => {
                    builtin_dotdot::builtin_dotdot(&mut self.commands);
                    match builtin_cd::builtin_cd(&self.commands) {
                        Ok(()) => println!(" ↓"),
                        Err(d) => println!("-! {}", d.red()),
                    };
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

        let mut child = Command::new(&self.commands.command)
            .args(&self.commands.args)
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to start sleep");
        // stdout handler getter
        let stdout = child.stdout.take().unwrap();
        // stdout output stream transfter
        let mut reader = FramedRead::new(stdout, LinesCodec::new());

        while let Some(line) = reader.next().await {
            println!("{}", line.unwrap());
        }
    }
}
