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

#[derive(GraphQLObject)]
// Market group
pub struct InvMarketGroup {
    pub id: i32,
    pub name: Option<String>,
}

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::<Context>::new())
}