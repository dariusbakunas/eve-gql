use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct CharacterResponse {
    pub alliance_id: i32,
    pub ancestry_id: i32,
    pub bloodline_id: i32,
    pub corporation_id: u32,
    pub gender: String,
    pub race_id: u32,
    pub name: String,
    pub security_status: f32,
}