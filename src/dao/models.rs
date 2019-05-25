#[derive(Queryable)]
pub struct InvType {
    pub id: i32,
    pub group: Option<i32>,
    pub name: Option<String>,
}