use std::env;

pub fn todoist_api_token() -> String {
    env::var("TODOIST_API_TOKEN").expect("Environment variables TODR_AUTHTOKEN is not set.")
}

pub fn slack_webhook_uri() -> String {
    env::var("SLACK_WEBHOOK_URL").expect("Environment variables SLACK_WEBHOOK_URL is not set.")
}

pub fn channel_name() -> String {
    env::var("SLACK_CHANNEL_NAME").expect("Environment variables SLACK_CHANNEL_NAME is not set.")
}