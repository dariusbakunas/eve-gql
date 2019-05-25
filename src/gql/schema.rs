use juniper::FieldResult;
use juniper::RootNode;
use juniper::EmptyMutation;
use crate::Context;
use diesel::prelude::*;

use crate::dao::models;

pub struct Query;

#[derive(GraphQLObject)]
#[graphql(description = "An item")]
struct InvType {
    pub id: i32,
    pub name: Option<String>,
}

impl From<models::InvType> for InvType {
    fn from(model: models::InvType) -> Self {
        InvType {
            id: model.id,
            name: model.name,
        }
    }
}

#[juniper::object(
    Context = Context,
)]
impl Query {
    fn invTypes(context: &Context) -> FieldResult<Vec<InvType>> {
        use crate::dao::schema::invTypes::dsl;

        let connection = executor.context().pool.clone().get().unwrap();

        let results = dsl::invTypes.order(dsl::typeName)
            .load::<models::InvType>(&*connection)?
            .into_iter()
            .map(|item| InvType::from(item))
            .collect();

        Ok(results)
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::<Context>::new())
}