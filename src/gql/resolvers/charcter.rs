use diesel::prelude::*;
use juniper::FieldResult;
use chrono::{DateTime, Utc};

use crate::Context;
use crate::dao::models;
use crate::esi::api;
use crate::gql::schema::SkillQueueItem;

use super::super::schema::Character;

#[juniper::object(
    Context = Context,
)]
impl Character {
    fn id(&self) -> i32 { self.id }
    fn name(&self) -> &String { &self.name }
    fn birthday(&self) -> DateTime<Utc> { self.birthday }
    fn gender(&self) -> &String { &self.gender }
    fn security_status(&self) -> f64 { self.security_status }

    fn ancestry(&self, context: &Context) -> FieldResult<models::ChrAncestry> {
        use crate::dao::schema::chrAncestries::dsl;

        let connection = context.pool.get().unwrap();

        let result = dsl::chrAncestries
            .find(self.ancestry_id)
            .get_result::<models::ChrAncestry>(&*connection)?;

        Ok(result)
    }

    fn bloodline(&self, context: &Context) -> FieldResult<models::ChrBloodline> {
        use crate::dao::schema::chrBloodlines::dsl;

        let connection = context.pool.get().unwrap();

        let result = dsl::chrBloodlines
            .find(self.bloodline_id)
            .get_result::<models::ChrBloodline>(&*connection)?;

        Ok(result)
    }

    fn race(&self, context: &Context) -> FieldResult<models::ChrRace> {
        use crate::dao::schema::chrRaces::dsl;

        let connection = context.pool.get().unwrap();

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