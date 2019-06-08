use bigdecimal::BigDecimal;

#[derive(Queryable)]
pub struct InvType {
    pub id: i32,
    pub group_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub mass: Option<f64>,
    pub volume: Option<f64>,
    pub capacity: Option<f64>,
    pub portion_size: Option<i32>,
    pub race_id: Option<i32>,
    pub base_price: Option<BigDecimal>,
    pub published: Option<bool>,
}

#[derive(Queryable)]
pub struct InvGroup {
    pub id: i32,
    pub category_id: Option<i32>,
    pub name: Option<String>,
    pub icon_id: Option<i32>,
    pub use_base_price: Option<bool>,
    pub anchored: Option<bool>,
    pub anchorable: Option<bool>,
    pub fittable_non_singleton: Option<bool>,
    pub published: Option<bool>,
}

#[derive(Queryable)]
pub struct InvMarketGroup {
    pub id: i32,
    pub parent_group_id: Option<i32>,
    pub name: Option<String>,
}

#[derive(Queryable)]
pub struct MapRegion {
    pub id: i32,
    pub name: Option<String>,
}

#[derive(Queryable)]
pub struct MapSolarSystem {
    pub region_id: Option<i32>,
    pub constellation_id: Option<i32>,
    pub id: i32,
    pub name: Option<String>,
}

#[derive(Queryable)]
pub struct ChrAncestry {
    pub id: i32,
    pub name: Option<String>,
}

#[derive(Queryable)]
pub struct ChrBloodline {
    pub id: i32,
    pub name: Option<String>,
}

#[derive(Queryable, GraphQLObject)]
pub struct ChrRace {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
}