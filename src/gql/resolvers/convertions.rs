use crate::dao::models;
use crate::gql::schema;
use crate::esi;

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
    fn from(model: esi::models::SkillQueueResponse) -> Self {
        schema::SkillQueueItem {
            id: model.skill_id,
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

impl From<models::InvType> for schema::Skill {
    fn from(model: models::InvType) -> Self {
        schema::Skill {
            id: model.id,
            name: model.name,
            description: model.description,
        }
    }
}