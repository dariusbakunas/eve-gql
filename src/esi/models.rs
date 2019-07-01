use serde::{Deserialize};
use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug)]
pub struct CharacterResponse {
    pub alliance_id: i32,
    pub birthday: DateTime<Utc>,
    pub ancestry_id: i32,
    pub bloodline_id: i32,
    pub corporation_id: i32,
    pub gender: String,
    pub race_id: i32,
    pub name: String,
    pub security_status: f64,
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

#[derive(Deserialize, Debug)]
pub struct CorporationResponse {
    pub alliance_id: Option<i32>,
    pub ceo_id: i32,
    pub creator_id: i32,
    pub date_founded: DateTime<Utc>,
    pub description: String,
    pub faction_id: Option<i32>,
    pub home_station_id: i32,
    pub member_count: i32,
    pub name: String,
    pub shares: i32,
    pub tax_rate: f64,
    pub ticker: String,
    pub url: String,
    pub war_eligible: Option<bool>,
}