use chrono::prelude::*;
use chrono::Duration;
use chrono::ParseResult;

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncStrust {
    pub sync_token: String, 
    pub full_sync: bool,
    pub projects: Option<Vec<self::ProjectStruct>>,
    pub items: Option<Vec<self::ItemStruct>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectStruct {
    pub id: u32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemStruct {
    pub content: String,
    pub project_id: u32,
    pub due_date_utc: Option<String>,
    pub priority: Option<u8>,
}

impl ItemStruct {
    pub fn parsed_due_date_utc(&self) -> ParseResult<DateTime<FixedOffset>> {
       let due_date_utc = self.due_date_utc.clone().unwrap_or(String::new());
       DateTime::parse_from_str(&due_date_utc, "%a %e %b %Y %T %z")
    }

    pub fn display_due_date_utc(&self) -> String {
        match self.parsed_due_date_utc() {
            Ok(s) => s.with_timezone(&FixedOffset::east(9*3600))
                      .format("%m-%d %H:%M").to_string(),
            Err(_) => String::new()
        }
    }

    pub fn is_nearing_due_date(&self) -> bool {
        let due_date_utc = match self.parsed_due_date_utc() {
            Ok(date) => date.with_timezone(&FixedOffset::east(9*3600)),
            Err(_) => return false
        };
        
        let now_date = Utc::now().with_timezone(&FixedOffset::east(9*3600));
        let due_limit_date = now_date + Duration::minutes(5);

        now_date <= due_date_utc && due_date_utc <= due_limit_date
    }

    pub fn is_past_due_date(&self) -> bool {
        let due_date_utc = match self.parsed_due_date_utc() {
            Ok(date) => date.with_timezone(&FixedOffset::east(9*3600)),
            Err(_) => return false
        };

        let now_date = Utc::now().with_timezone(&FixedOffset::east(9*3600));
        due_date_utc < now_date
    }

    pub fn is_today_due_date(&self) -> bool {
        let due_date_utc = match self.parsed_due_date_utc() {
            Ok(date) => date.with_timezone(&FixedOffset::east(9*3600)),
            Err(_) => return false
        };

        let now_date = Utc::now().with_timezone(&FixedOffset::east(9*3600));
        due_date_utc.day() == now_date.day()
    }
}