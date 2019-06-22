use juniper::EmptyMutation;
use juniper::RootNode;

use crate::Context;
use chrono::{DateTime, Utc};

pub struct Query;

#[derive(GraphQLObject)]
// An item
pub struct InvType {
    pub id: i32,
    pub name: Option<String>,
}

#[derive(GraphQLObject)]
// Item group
pub struct InvGroup {
    pub id: i32,
    pub name: Option<String>,
}

pub struct InvMarketGroup {
    pub id: i32,
    pub name: Option<String>,
}

pub struct MapRegion {
    pub id: i32,
    pub name: Option<String>,
}

#[derive(GraphQLObject)]
pub struct MapSolarSystem {
    pub id: i32,
    pub name: Option<String>,
}

pub struct Character {
    pub id: i32,
    pub name: String,
    pub ancestry_id: i32,
    pub bloodline_id: i32,
    pub race_id: i32,
}

pub struct SkillQueueItem {
    pub id: i32,
    pub index: i32,
    pub finished_level: i32,
    pub start_date: DateTime<Utc>,
    pub finish_date: DateTime<Utc>,
    pub level_start_sp: i32,
    pub level_end_sp: i32,
    pub training_start_sp: i32,
}

#[derive(GraphQLObject, Clone)]
pub struct Skill {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
}

pub struct SkillGroup {
    pub id: i32,
    pub name: Option<String>,
}

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::<Context>::new())
}