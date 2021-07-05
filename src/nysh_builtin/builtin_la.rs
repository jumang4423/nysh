// command line tools
use super::super::tools::parser::CommandParser;
use super::super::tools::runner::CommandRunner;

pub async fn builtin_la() {
    let mut commands = CommandParser::constructor("ls -al\n".to_string());
    commands.parse_it();
    let mut runner = CommandRunner::constructor(commands).unwrap();
    runner.async_runner().await;
}
