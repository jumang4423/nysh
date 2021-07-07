use super::super::nysh_builtin::builtin_cd;
use super::super::nysh_builtin::builtin_dotdot;
use super::super::nysh_builtin::builtin_dream95;
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
    pub black_list: Vec<String> 
}

impl CommandRunner {
    pub fn constructor(command_object: parser::CommandParser) -> Result<Self, ()> {
        Ok(Self {
            commands: command_object,
            black_list: ["find".to_string()].to_vec()
        })
    }

    pub async fn run_command(&mut self) {
        if self.commands.command == "" {
            return;
        }

        for black in &self.black_list {

            if black.clone() == self.commands.command {
                println!("the command is blacklisted.");
                return;
            }

        }

        if is_builtin::is_builtin(&mut self.commands.command) {
            let _com: &str = &self.commands.command;
            match _com {
                "exit" => {
                    builtin_exit::builtin_exit();
                }
                "la" => {
                    builtin_la::builtin_la().await;
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
                "dream95" => {
                    builtin_dream95::builtin_dream95();
                }
                _ => return,
            }
            return;
        }

        self.async_runner().await;
    }

    pub async fn async_runner(&mut self) {
        let child = Command::new(&self.commands.command)
            .args(&self.commands.args)
            .stdout(Stdio::piped())
            .spawn();
        match child {
            Ok(mut _read) => {
                // stdout handler getter
                let stdout = _read.stdout.take().unwrap();
                // stdout output stream transfter
                let mut reader = FramedRead::new(stdout, LinesCodec::new());
                while let Some(line) = reader.next().await {
                    match line {
                        Ok(_read) => println!("{}", _read),
                        Err(err) => println!("process error => {}", err),
                    }
                }
            }
            Err(err) => println!("{}", err),
        }
    }
}
