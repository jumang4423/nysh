use dirs;
use std::env;
// command line tools
use colored::*;

use super::super::tools;

use super::super::tools::parser::CommandParser;

pub async fn builtin_cd(commands: &CommandParser) -> Result<(), String> {
    match commands.args.len() {
        0 => {
            env::set_current_dir(dirs::home_dir().unwrap()).unwrap();
        }
        1 => {
            if !env::set_current_dir(commands.args.join("")).is_ok() {
                return Err("no directory found".to_owned());
            }
        }
        _ => {
            return Err("too many arguments".to_owned());
        }
    }

    if std::path::Path::new(&format!(
        "{}/.flag",
        env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap()
    ))
    .exists()
    {
        if is_slack_api_url() {
            println!(
                "{}",
                "! you cannot enter this directory. This incident will be reported to slack".red()
            );

            tools::slack::slack_sender().await;
        } else {
            println!("{}", "! you cannot enter this directory".red());
            println!(
                "{}",
                "! dengerous since no connection with slack in your shell scope".red()
            );
            println!(
                "{} {}",
                "! so run".red(),
                "export DREAM95_SLACK_URL={your slack webhook url}".green()
            );
        }
    }

    Ok(())
}

pub fn is_slack_api_url() -> bool {
    return match env::var("DREAM95_SLACK_URL") {
        Ok(_) => true,
        Err(_) => false,
    };
}
