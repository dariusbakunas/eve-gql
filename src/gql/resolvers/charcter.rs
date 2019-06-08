use crate::Context;
use crate::dao::models;
use crate::esi::api;
use diesel::prelude::*;
use super::super::schema::Character;
use juniper::{FieldResult};
use crate::gql::schema::SkillQueueItem;

#[juniper::object(
    Context = Context,
)]
impl Character {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn ancestry(&self, context: &Context) -> FieldResult<models::ChrAncestry> {
        use crate::dao::schema::chrAncestries::dsl;

        let connection = executor.context().pool.clone().get().unwrap();

        let result = dsl::chrAncestries
            .find(self.ancestry_id)
            .get_result::<models::ChrAncestry>(&*connection)?;

        Ok(result)
    }

    fn bloodline(&self, context: &Context) -> FieldResult<models::ChrBloodline> {
        use crate::dao::schema::chrBloodlines::dsl;

        let connection = executor.context().pool.clone().get().unwrap();

        let result = dsl::chrBloodlines
            .find(self.bloodline_id)
            .get_result::<models::ChrBloodline>(&*connection)?;

        Ok(result)
    }

    fn race(&self, context: &Context) -> FieldResult<models::ChrRace> {
        use crate::dao::schema::chrRaces::dsl;

        let connection = executor.context().pool.clone().get().unwrap();

        let result = dsl::chrRaces
            .find(self.race_id)
            .get_result::<models::ChrRace>(&*connection)?;

        Ok(result)
    }

    fn skill_queue(&self, token: String, context: &Context) -> FieldResult<Option<Vec<SkillQueueItem>>> {
        let skill_queue: Option<Vec<SkillQueueItem>> = api::get_skill_queue(self.id, &token[..])?
            .and_then(|queue| {
                Some(queue.into_iter()
                    .map(|item| SkillQueueItem::from(item))
                    .collect())
            });

        Ok(skill_queue)
    }
}