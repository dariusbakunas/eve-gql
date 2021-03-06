use diesel::prelude::*;
use juniper::{FieldResult, FieldError};
use chrono::{DateTime, Utc};

use crate::Context;
use crate::dao::models;
use crate::esi::api;

use super::super::schema::{Character, Corporation, CharacterSkill, SkillQueueItem};
use crate::gql::resolvers::cache::{get_corporation};
use crate::errors::ErrorKind;

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

    fn corporation(&self, context: &Context) -> FieldResult<Option<Corporation>> {
        let corporation = get_corporation(self.corporation_id);
        return corporation;
    }

    fn race(&self, context: &Context) -> FieldResult<models::ChrRace> {
        use crate::dao::schema::chrRaces::dsl;

        let connection = context.pool.get().unwrap();

        let result = dsl::chrRaces
            .find(self.race_id)
            .get_result::<models::ChrRace>(&*connection)?;

        Ok(result)
    }

    fn skill_points(&self, context: &Context) -> FieldResult<Option<i32>> {
        let token = &context.esi_token;

        match token {
            None => {
                Err(FieldError::new(
                    "skillPoints require ESI token",
                    graphql_value!({ "user_error": "ESI token missing" })
                ))
            },
            Some(esi) => {
                let sp: Option<i32> = api::get_skills(self.id, &esi[..])?
                    .and_then(|skills| {
                        Some(skills.total_sp)
                    });

                Ok(sp)
            },
        }
    }

    fn skills(&self, context: &Context) -> FieldResult<Option<Vec<CharacterSkill>>> {
        let token = &context.esi_token;

        match token {
            None => {
                Err(FieldError::new(
                    "skills require ESI token",
                    graphql_value!({ "user_error": "ESI token missing" })
                ))
            },
            Some(esi) => {
                let sp: Option<Vec<CharacterSkill>> = api::get_skills(self.id, &esi[..])?
                    .and_then(|response| {
                        let resp: Vec<CharacterSkill> = response.skills.iter()
                            .map(|skill| {
                                CharacterSkill {
                                    id: skill.skill_id,
                                    trained_skill_level: skill.trained_skill_level,
                                    active_skill_level: skill.active_skill_level,
                                    skillpoints_in_skill: skill.skillpoints_in_skill,
                                }
                            })
                            .collect();
                        Some(resp)
                    });

                Ok(sp)
            },
        }
    }

    fn skill_queue(&self, context: &Context) -> FieldResult<Option<Vec<SkillQueueItem>>> {
        let token = &context.esi_token;

        match token {
            None => {
                Err(FieldError::new(
                    "skillQueue requires ESI token",
                    graphql_value!({ "user_error": "ESI token missing" })
                ))
            },
            Some(esi) => {
                let skill_queue: Option<Vec<SkillQueueItem>> = api::get_skill_queue(self.id, &esi[..])?
                    .and_then(|queue| {
                        Some(queue.into_iter()
                            .map(|item| SkillQueueItem::from(item))
                            .collect())
                    });

                Ok(skill_queue)
            },
        }
    }
}