use juniper::RootNode;
use juniper::EmptyMutation;
use crate::Context;

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
    pub name: Option<String>,
    pub index: u32,
}

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::<Context>::new())
}