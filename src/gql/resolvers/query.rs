use diesel::debug_query;
use diesel::prelude::*;
use juniper::{FieldResult};

use crate::Context;
use crate::dao::models;
use crate::esi;
use crate::esi::api;
use crate::esi::models::SkillQueueResponse;

use super::super::schema;

impl From<models::InvType> for schema::InvType {
    fn from(model: models::InvType) -> Self {
        schema::InvType {
            id: model.id,
            name: model.name,
        }
    }
}

impl From<models::InvGroup> for schema::InvGroup {
    fn from(model: models::InvGroup) -> Self {
        schema::InvGroup {
            id: model.id,
            name: model.name,
        }
    }
}

impl From<models::InvMarketGroup> for schema::InvMarketGroup {
    fn from(model: models::InvMarketGroup) -> Self {
        schema::InvMarketGroup {
            id: model.id,
            name: model.name,
        }
    }
}

impl From<models::MapRegion> for schema::MapRegion {
    fn from(model: models::MapRegion) -> Self {
        schema::MapRegion {
            id: model.id,
            name: model.name,
        }
    }
}

impl From<models::MapSolarSystem> for schema::MapSolarSystem {
    fn from(model: models::MapSolarSystem) -> Self {
        schema::MapSolarSystem {
            id: model.id,
            name: model.name,
        }
    }
}

impl From<models::InvGroup> for schema::SkillGroup {
    fn from(model: models::InvGroup) -> Self {
        schema::SkillGroup {
            id: model.id,
            name: model.name,
        }
    }
}

impl From<esi::models::SkillQueueResponse> for schema::SkillQueueItem {
    fn from(model: SkillQueueResponse) -> Self {
        schema::SkillQueueItem {
            id: model.skill_id,
            name: None, // will resolve later
            index: model.queue_position,
            finished_level: model.finished_level,
            start_date: model.start_date,
            finish_date: model.finish_date,
            level_start_sp: model.level_start_sp,
            level_end_sp: model.level_end_sp,
            training_start_sp: model.training_start_sp,
        }
    }
}

#[juniper::object(
Context = Context,
)]
impl schema::Query {
    fn character(context: &Context, id: i32) -> FieldResult<Option<schema::Character>> {
        let character = api::get_character(id)?.and_then(|character| {
            Some(schema::Character {
                id,
                name: character.name,
                ancestry_id: character.ancestry_id,
                bloodline_id: character.bloodline_id,
                race_id: character.race_id,
            })
        });

        Ok(character)
    }

    fn invTypes(context: &Context) -> FieldResult<Vec<schema::InvType>> {
        use crate::dao::schema::invTypes::dsl;

        let connection = context.pool.get().unwrap();

        let results = dsl::invTypes.order(dsl::typeName)
            .load::<models::InvType>(&*connection)?
            .into_iter()
            .map(|item| schema::InvType::from(item))
            .collect();

        Ok(results)
    }

    fn invGroups(context: &Context) -> FieldResult<Vec<schema::InvGroup>> {
        use crate::dao::schema::invGroups::dsl;

        let connection = context.pool.get().unwrap();

        let results = dsl::invGroups.order(dsl::groupName)
            .load::<models::InvGroup>(&*connection)?
            .into_iter()
            .map(|item| schema::InvGroup::from(item))
            .collect();

        Ok(results)
    }

    fn mapRegions(context: &Context) -> FieldResult<Vec<schema::MapRegion>> {
        use crate::dao::schema::mapRegions::dsl;

        let connection = context.pool.get().unwrap();

        let results = dsl::mapRegions.order(dsl::regionName)
            .load::<models::MapRegion>(&*connection)?
            .into_iter()
            .map(|item| schema::MapRegion::from(item))
            .collect();

        Ok(results)
    }

    fn mapSolarSystems(context: &Context) -> FieldResult<Vec<schema::MapSolarSystem>> {
        use crate::dao::schema::mapSolarSystems::dsl;

        let connection = context.pool.get().unwrap();

        let results = dsl::mapSolarSystems.order(dsl::solarSystemName)
            .load::<models::MapSolarSystem>(&*connection)?
            .into_iter()
            .map(|item| schema::MapSolarSystem::from(item))
            .collect();

        Ok(results)
    }

    fn invMarketGroups(context: &Context) -> FieldResult<Vec<schema::InvMarketGroup>> {
        use crate::dao::schema::invMarketGroups::dsl;

        let connection = context.pool.get().unwrap();

        let results = dsl::invMarketGroups.order(dsl::marketGroupName)
            .filter(dsl::parentGroupID.is_null())
            .load::<models::InvMarketGroup>(&*connection)?
            .into_iter()
            .map(|item| schema::InvMarketGroup::from(item))
            .collect();

        Ok(results)
    }

    fn skillGroups(context: &Context) -> FieldResult<Vec<schema::SkillGroup>> {
        use crate::dao::schema::invGroups::dsl;

        let connection = context.pool.get().unwrap();

        let query = dsl::invGroups.order(dsl::groupName)
            .filter(dsl::published.eq(true))
            .filter(dsl::categoryID.eq(16));

        let sql = debug_query::<diesel::mysql::Mysql, _>(&query);

        info!("Get skill groups: {:?}", sql);

        let results = query
            .load::<models::InvGroup>(&*connection)?
            .into_iter()
            .map(|group| schema::SkillGroup::from(group))
            .collect();

        Ok(results)
    }
}

#[juniper::object(
Context = Context,
)]
impl models::ChrAncestry {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &Option<String> {
        &self.name
    }
}

#[juniper::object(
Context = Context,
)]
impl models::ChrBloodline {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &Option<String> {
        &self.name
    }
}

#[juniper::object(
Context = Context,
)]
impl schema::InvMarketGroup {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &Option<String> {
        &self.name
    }

    fn invMarketGroups(&self, context: &Context) -> FieldResult<Vec<schema::InvMarketGroup>> {
        use crate::dao::schema::invMarketGroups::dsl;

        let connection = context.pool.get().unwrap();

        let results = dsl::invMarketGroups.order(dsl::marketGroupName)
            .filter(dsl::parentGroupID.eq(&self.id))
            .load::<models::InvMarketGroup>(&*connection)?
            .into_iter()
            .map(|item| schema::InvMarketGroup::from(item))
            .collect();

        Ok(results)
    }
}

#[juniper::object(
Context = Context,
)]
impl schema::MapRegion {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &Option<String> {
        &self.name
    }

    fn mapSolarSystems(&self, context: &Context) -> FieldResult<Vec<schema::MapSolarSystem>> {
        use crate::dao::schema::mapSolarSystems::dsl;

        let connection = context.pool.get().unwrap();

        let results = dsl::mapSolarSystems.order(dsl::solarSystemName)
            .filter(dsl::regionID.eq(&self.id))
            .load::<models::MapSolarSystem>(&*connection)?
            .into_iter()
            .map(|item| schema::MapSolarSystem::from(item))
            .collect();

        Ok(results)
    }
}