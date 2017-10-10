use types;
use todoist;
use slack;

pub fn check_naring_due_date() {
    println!("Execute task 'check_naring_due_date'");

    let items = get_nearing_due_date_items();
    let item_count: usize = items.iter().count();

    if item_count >= 1 {
        let info = format!("{} tasks is nearing due date.", item_count);
        println!("{}", info);
        send_slack(info, items);
    } else {
        println!("all tasks is not nearing due date.");
    }
}

pub fn check_past_due_date() {
    println!("Execute task 'check_past_due_date'");

    let items = get_past_due_date_items();
    let item_count: usize = items.iter().count();

    if item_count >= 1 {
        let info = format!("{} tasks is past due date.", item_count);
        println!("{}", info);
        send_slack(info, items);
    } else {
        println!("all tasks is not nearing due date.");
    }
}

fn get_nearing_due_date_items() -> Vec<types::ItemStruct> {
    let items: Vec<types::ItemStruct> = todoist::render_items();
    items.into_iter()
         .filter(move |task| task.is_nearing_due_date())
         .collect::<Vec<types::ItemStruct>>()
}

fn get_past_due_date_items() -> Vec<types::ItemStruct> {
    let items: Vec<types::ItemStruct> = todoist::render_items();
    items.into_iter()
         .filter(move |task| task.is_past_due_date())
         .collect::<Vec<types::ItemStruct>>()
}

fn send_slack(info: String, items: Vec<types::ItemStruct>) {
    let mut message = format!("{}\n\n", info);
    for item in items.into_iter() {
        message = format!("{}{}\n", message, &item.content);
    }
    slack::notification_to_slack(message);
}