use std::env;

extern crate slack_hook;
use slack_hook::{PayloadBuilder, Slack};

pub async fn slack_sender() {
    let slack = Slack::new(is_slack_api_url().as_str()).unwrap();
    let p = PayloadBuilder::new()
        .text("someone entered your secret folder")
        .build()
        .unwrap();

    let res = slack.send(&p);
    match res {
        Ok(()) => println!("ok"),
        Err(x) => println!("ERR: {:?}", x),
    }
}

pub fn is_slack_api_url() -> String {
    return match env::var("DREAM95_SLACK_URL") {
        Ok(v) => v,
        Err(_) => "".to_string(),
    };
}
