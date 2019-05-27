use crate::Context;
use juniper::FieldResult;
use diesel::prelude::*;
use crate::dao::models;
use super::schema::Query;
use super::schema::InvType;
use super::schema::InvGroup;
use super::schema::InvMarketGroup;
use super::schema::MapRegion;
use super::schema::MapSolarSystem;

impl From<models::InvType> for InvType {
    fn from(model: models::InvType) -> Self {
        InvType {
            id: model.id,
            name: model.name,
        }
    }
}

impl From<models::InvGroup> for InvGroup {
    fn from(model: models::InvGroup) -> Self {
        InvGroup {
            id: model.id,
            name: model.name,
        }
    }
}

impl From<models::InvMarketGroup> for InvMarketGroup {
    fn from(model: models::InvMarketGroup) -> Self {
        InvMarketGroup {
            id: model.id,
            name: model.name,
        }
    }
}

impl From<models::MapRegion> for MapRegion {
    fn from(model: models::MapRegion) -> Self {
        MapRegion {
            id: model.id,
            name: model.name,
        }
    }
}

impl From<models::MapSolarSystem> for MapSolarSystem {
    fn from(model: models::MapSolarSystem) -> Self {
        MapSolarSystem {
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

    fn invGroups(context: &Context) -> FieldResult<Vec<InvGroup>> {
        use crate::dao::schema::invGroups::dsl;

        let connection = executor.context().pool.clone().get().unwrap();

        let results = dsl::invGroups.order(dsl::groupName)
            .load::<models::InvGroup>(&*connection)?
            .into_iter()
            .map(|item| InvGroup::from(item))
            .collect();

        Ok(results)
    }

    fn mapRegions(context: &Context) -> FieldResult<Vec<MapRegion>> {
        use crate::dao::schema::mapRegions::dsl;

        let connection = executor.context().pool.clone().get().unwrap();

        let results = dsl::mapRegions.order(dsl::regionName)
            .load::<models::MapRegion>(&*connection)?
            .into_iter()
            .map(|item| MapRegion::from(item))
            .collect();

        Ok(results)
    }

    fn mapSolarSystems(context: &Context) -> FieldResult<Vec<MapSolarSystem>> {
        use crate::dao::schema::mapSolarSystems::dsl;

        let connection = executor.context().pool.clone().get().unwrap();

        let results = dsl::mapSolarSystems.order(dsl::solarSystemName)
            .load::<models::MapSolarSystem>(&*connection)?
            .into_iter()
            .map(|item| MapSolarSystem::from(item))
            .collect();

        Ok(results)
    }

    fn invMarketGroups(context: &Context) -> FieldResult<Vec<InvMarketGroup>> {
        use crate::dao::schema::invMarketGroups::dsl;

        let connection = executor.context().pool.clone().get().unwrap();

        let results = dsl::invMarketGroups.order(dsl::marketGroupName)
            .filter(dsl::parentGroupID.is_null())
            .load::<models::InvMarketGroup>(&*connection)?
            .into_iter()
            .map(|item| InvMarketGroup::from(item))
            .collect();

        Ok(results)
    }
}

#[juniper::object(
Context = Context,
)]
impl InvMarketGroup {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &Option<String> {
        &self.name
    }

    fn invMarketGroups(&self, context: &Context) -> FieldResult<Vec<InvMarketGroup>> {
        use crate::dao::schema::invMarketGroups::dsl;

        let connection = executor.context().pool.clone().get().unwrap();

        let results = dsl::invMarketGroups.order(dsl::marketGroupName)
            .filter(dsl::parentGroupID.eq(&self.id))
            .load::<models::InvMarketGroup>(&*connection)?
            .into_iter()
            .map(|item| InvMarketGroup::from(item))
            .collect();

        Ok(results)
    }
}

#[juniper::object(
Context = Context,
)]
impl MapRegion {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &Option<String> {
        &self.name
    }

    fn mapSolarSystems(&self, context: &Context) -> FieldResult<Vec<MapSolarSystem>> {
        use crate::dao::schema::mapSolarSystems::dsl;

        let connection = executor.context().pool.clone().get().unwrap();

        let results = dsl::mapSolarSystems.order(dsl::solarSystemName)
            .filter(dsl::regionID.eq(&self.id))
            .load::<models::MapSolarSystem>(&*connection)?
            .into_iter()
            .map(|item| MapSolarSystem::from(item))
            .collect();

        Ok(results)
    }
}
