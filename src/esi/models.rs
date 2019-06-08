use serde::{Deserialize};
use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug)]
pub struct CharacterResponse {
    pub alliance_id: i32,
    pub ancestry_id: i32,
    pub bloodline_id: i32,
    pub corporation_id: u32,
    pub gender: String,
    pub race_id: i32,
    pub name: String,
    pub security_status: f32,
}

#[derive(Deserialize, Debug)]
pub struct SkillQueueResponse {
    pub queue_position: i32,
    pub skill_id: i32,
    pub level_end_sp: i32,
    pub level_start_sp: i32,
    pub finished_level: i32,
    pub training_start_sp: i32,
    pub start_date: DateTime<Utc>,
    pub finish_date: DateTime<Utc>,
}