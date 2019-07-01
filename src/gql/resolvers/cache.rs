use diesel::prelude::*;
use cached::UnboundCache;
use super::super::schema;
use crate::dao::models;
use crate::esi::api;
use crate::esi;
use crate::errors::*;
use r2d2::PooledConnection;
use r2d2_diesel::ConnectionManager;
use diesel::mysql::MysqlConnection;
use juniper::{FieldResult};
use diesel::debug_query;
use chrono::{DateTime, Utc};

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
    GET_CORPORATION: UnboundCache<i32, Option<schema::Corporation>> = UnboundCache::new();
    Key = { id };
    fn get_corporation(id: i32) -> FieldResult<Option<schema::Corporation>> = {
        let result: Result<Option<esi::models::CorporationResponse>> = api::get_corporation(id);

        match result {
            Ok(response) => {
                match response {
                    Some(corporation) => {
                        return Ok(Some(schema::Corporation {
                            id: id,
                            name: corporation.name,
                            description: corporation.description,
                            tax_rate: corporation.tax_rate,
                            member_count: corporation.member_count,
                            date_founded: corporation.date_founded,
                            ticker: corporation.ticker,
                            url: corporation.url,
                        }));
                    },
                    None => {
                        return Ok(None);
                    }
                }
            },
            Err(e) => {
                error!("Unable to fetch corporation: {:?}", e);
                return Err("Unable to fetch corporation");
            }
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