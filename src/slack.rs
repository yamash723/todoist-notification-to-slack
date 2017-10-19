use slack_hook::{Slack, PayloadBuilder};
use envs;

pub fn notification_to_slack(message: String) {
    let slack_webhook_uri = envs::slack_webhook_uri();
    let slack = Slack::new(&*slack_webhook_uri).unwrap();
    let p = PayloadBuilder::new()
      .text(message)
      .channel(envs::channel_name())
      .build()
      .unwrap();

    let res = slack.send(&p);
    match res {
        Ok(()) => println!("Sending message for Slack is completed!"),
        Err(x) => println!("ERR: {:?}",x)
    }
}