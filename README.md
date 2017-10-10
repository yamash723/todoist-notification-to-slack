# Todoist notification to Slack

I'm Standalone battale supprt unit - ADA.  
Would you like to explain the operation?

## Usage

This program has 2 execute mode.

* near
    * nearing(within 5 min) due date tasks name send to slack.
* past
    * passed due date tasks name send to slack.

```bash
export TODOIST_API_TOKEN=XXXXXX
export SLACK_WEBHOOK_URL=XXXXXXXXX

# mode `near`
cargo run near

# mode `past`
cargo run past
```