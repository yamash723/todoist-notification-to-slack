# Todoist notification to Slack

I'm Standalone battale supprt unit - ADA.  
Would you like to explain the operation?

## Usage

This program has 3 execute mode.

* near
    * nearing(within 5 min) due date tasks send to slack.
* past
    * passed due date tasks send to slack.
* today
    * today's tasks send to slack

```bash
export TODOIST_API_TOKEN=XXXXXX
export SLACK_WEBHOOK_URL=XXXXXXXXX
export SLACK_CHANNEL_NAME=XXXXXXXXX

# mode `near`
cargo run near

# mode `past`
cargo run past

# mode `today`
cargo run today
```