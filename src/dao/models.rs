#[derive(Queryable)]
pub struct InvType {
    pub id: i32,
    pub group_id: Option<i32>,
    pub name: Option<String>,
}

#[derive(Queryable)]
pub struct InvGroup {
    pub id: i32,
    pub category_id: Option<i32>,
    pub name: Option<String>,
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