use diesel::debug_query;
use diesel::prelude::*;
use juniper::{FieldResult};

use crate::Context;
use crate::dao::models;
use crate::esi::api;

use super::super::schema;
use r2d2::PooledConnection;
use r2d2_diesel::ConnectionManager;
use cached::UnboundCache;

cached_key!{
    GET_SKILL: UnboundCache<i32, Option<schema::Skill>> = UnboundCache::new();
    Key = { id };
    fn get_skill(connection: PooledConnection<ConnectionManager<MysqlConnection>>, id: i32) -> Option<schema::Skill> = {
        use crate::dao::schema::invTypes::dsl;

        let query = dsl::invTypes
            .find(id);

        let sql = debug_query::<diesel::mysql::Mysql, _>(&query);
        info!("Get skill groups: {:?}", sql);

        match query.first::<models::InvType>(&*connection) {
            Ok(skill) => Some(schema::Skill::from(skill)),
            Err(_) => None,
        }
    }
}

cached_key_result!{
    GET_GROUP_SKILLS: UnboundCache<i32, Vec<schema::Skill>> = UnboundCache::new();
    Key = { id };
    fn get_group_skills(connection: PooledConnection<ConnectionManager<MysqlConnection>>, id: i32) -> FieldResult<Vec<schema::Skill>> = {
        use crate::dao::schema::invTypes::dsl;

        let query = dsl::invTypes.order(dsl::typeName)
            .filter(dsl::groupID.eq(id))
            .filter(dsl::published.eq(true));

        let sql = debug_query::<diesel::mysql::Mysql, _>(&query);
        info!("Get skills for group: {:?}", sql);

        let result = query.load::<models::InvType>(&*connection);

        match result {
            Ok(s) => {
                let skills: Vec<schema::Skill> = s
                    .into_iter()
                    .map(|skill: models::InvType| schema::Skill::from(skill))
                    .collect();
                Ok(skills)
            },
            Err(e) => Err(e),
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

    fn skill(context: &Context, id: i32) -> FieldResult<Option<schema::Skill>> {
        let connection = context.pool.get().unwrap();

        let skill = get_skill(connection, id);

        Ok(skill)
    }

    fn inv_types(context: &Context) -> FieldResult<Vec<schema::InvType>> {
        use crate::dao::schema::invTypes::dsl;

        let connection = context.pool.get().unwrap();

        let results = dsl::invTypes.order(dsl::typeName)
            .load::<models::InvType>(&*connection)?
            .into_iter()
            .map(|item| schema::InvType::from(item))
            .collect();

        Ok(results)
    }

    fn inv_groups(context: &Context) -> FieldResult<Vec<schema::InvGroup>> {
        use crate::dao::schema::invGroups::dsl;

        let connection = context.pool.get().unwrap();

        let results = dsl::invGroups.order(dsl::groupName)
            .load::<models::InvGroup>(&*connection)?
            .into_iter()
            .map(|item| schema::InvGroup::from(item))
            .collect();

        Ok(results)
    }

    fn map_regions(context: &Context) -> FieldResult<Vec<schema::MapRegion>> {
        use crate::dao::schema::mapRegions::dsl;

        let connection = context.pool.get().unwrap();

        let results = dsl::mapRegions.order(dsl::regionName)
            .load::<models::MapRegion>(&*connection)?
            .into_iter()
            .map(|item| schema::MapRegion::from(item))
            .collect();

        Ok(results)
    }

    fn map_solar_systems(context: &Context) -> FieldResult<Vec<schema::MapSolarSystem>> {
        use crate::dao::schema::mapSolarSystems::dsl;

        let connection = context.pool.get().unwrap();

        let results = dsl::mapSolarSystems.order(dsl::solarSystemName)
            .load::<models::MapSolarSystem>(&*connection)?
            .into_iter()
            .map(|item| schema::MapSolarSystem::from(item))
            .collect();

        Ok(results)
    }

    fn inv_market_groups(context: &Context) -> FieldResult<Vec<schema::InvMarketGroup>> {
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

    fn skill_groups(context: &Context) -> FieldResult<Vec<schema::SkillGroup>> {
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
impl schema::SkillGroup {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &Option<String> {
        &self.name
    }

    fn skills(&self, context: &Context) -> FieldResult<Vec<schema::Skill>> {
        use crate::dao::schema::invTypes::dsl;

        let connection = context.pool.get().unwrap();
        let skills = get_group_skills(connection, self.id.clone());

        skills
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