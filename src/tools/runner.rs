use super::super::nysh_builtin::builtin_cd;
use super::super::nysh_builtin::builtin_dotdot;
use super::super::nysh_builtin::builtin_dream95;
use super::super::nysh_builtin::builtin_exit;
use super::super::nysh_builtin::builtin_help;
use super::super::nysh_builtin::builtin_la;
use super::super::nysh_builtin::builtin_nsd;
use super::super::nysh_builtin::builtin_nywer;
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
    pub black_list: Vec<String>,
}

impl CommandRunner {
    pub fn constructor(command_object: parser::CommandParser) -> Result<Self, ()> {
        Ok(Self {
            commands: command_object,
            black_list: ["find".to_string(), "emacs".to_string()].to_vec(),
        })
    }

    pub async fn run_command(&mut self) {
        if self.commands.command.is_empty() {
            return;
        }

        for black in &self.black_list {
            if black.clone() == self.commands.command {
                println!("the command is blacklisted.");
                return;
            }
        }

        if is_builtin::is_builtin(&self.commands.command) {
            let _com: &str = &self.commands.command;
            match _com {
                "exit" => {
                    builtin_exit::builtin_exit();
                }
                "la" => {
                    builtin_la::builtin_la().await;
                }
                "cd" => {
                    match builtin_cd::builtin_cd(&self.commands).await {
                        Ok(()) => println!(" ↓"),
                        Err(d) => println!("-! {}", d.red()),
                    };
                }
                ".." => {
                    builtin_dotdot::builtin_dotdot(&mut self.commands);
                    match builtin_cd::builtin_cd(&self.commands).await {
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
                    builtin_dream95::builtin_dream95().await;
                }
                "nywer" => builtin_nywer::builtin_nywer(self.commands.args.clone()),
                "nsd" => builtin_nsd::builtin_nsd(self.commands.args.clone()),
                "l" => builtin_nsd::builtin_nsd([].to_vec()),
                "ll" => builtin_nsd::builtin_nsd(["l".to_string()].to_vec()),
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
                        Ok(_read) => println!("❤️ {}", _read),
                        Err(err) => println!("process error => {}", err),
                    }
                }
            }
            Err(err) => println!("{}", err),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::parser::CommandParser;
    use super::*;

    #[test]
    fn command_runner_blacklisted() {
        // parse command into command object
        let mut commands = CommandParser::constructor("ls\n".to_owned());
        commands.parse_it();

        let runner = CommandRunner::constructor(commands).unwrap();

        assert_eq!(runner.black_list[1], "emacs".to_owned());
        assert_eq!(runner.black_list[0], "find".to_owned());
        
    }
}
