#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate url;
extern crate reqwest;
extern crate chrono;
extern crate slack_hook;
extern crate itertools;

use std::env;

mod envs;
mod tasks;
mod types;
mod todoist;
mod slack;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Execute path is {}.", args[0]);

    if args.iter().count() <= 1 {
        panic!("Not set Execute mode params! please set `near` or `past` or `today`");
    }

    let mode = &args[1];
    match mode.as_ref() {
        "near" => tasks::check_naring_due_date(),
        "past" => tasks::check_past_due_date(),
        "today" => tasks::check_today_due_date(),
        _ => panic!("Execute mode is wrong! please set `near` or `past` or `today`")
    }
}