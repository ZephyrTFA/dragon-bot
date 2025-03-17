use chrono::NaiveDateTime;
use mysql::prelude::FromRow;

#[derive(FromRow, Clone)]
pub struct ByondDiscordLink {
    pub id: u64,
    pub ckey: String,
    pub discord_id: u64,
    pub timestamp: NaiveDateTime,
    pub one_time_token: String,
    pub valid: bool,
}
