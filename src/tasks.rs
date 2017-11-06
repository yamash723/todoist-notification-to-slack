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
        println!("all tasks is not past due date.");
    }
}

pub fn check_today_due_date() {
    println!("Execute task 'check_today_due_date'");

    let items = get_today_due_date_items();
    let item_count: usize = items.iter().count();

    if item_count >= 1 {
        let greeting = "I'm Standalone battale supprt unit - ADA.";
        let info = format!("{}\n\n{} tasks is today due date.", greeting, item_count);
        println!("{}", info);
        send_slack(info, items);
    } else {
        println!("all tasks is not today due date.");
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

fn get_today_due_date_items() -> Vec<types::ItemStruct> {
    let items: Vec<types::ItemStruct> = todoist::render_items();
    items.into_iter()
         .filter(move |task| task.is_today_due_date())
         .collect::<Vec<types::ItemStruct>>()
}

fn send_slack(info: String, items: Vec<types::ItemStruct>) {
    let projects = todoist::render_projects();

    let get_project_name = |id: u32| -> String {
        let projects = &projects;
        projects.into_iter()
                .find(|p| p.id == id)
                .expect("undefine project!!").name.clone()
    };

    // mm-dd HH:MM - {task_name} [#{project_name}]
    // mm-dd HH:MM - {task_name} [#{project_name}]
    let mut message = String::new();
    let mut items = items;
    items.sort_by_key(|item| item.project_id);

    for item in items {
        let project_name = get_project_name(item.project_id);
        message = format!(
            "{}\n{} - {} [{}]",
            message,
            &item.display_due_date_utc(),
            &item.content,
            project_name
        );
    }

    message = format!("{}\n\n{}", message, info);
    slack::notification_to_slack(message);
}