use crate::schema::branches;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = branches)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Branch {
    pub state: String,
    pub distance: i32,
    pub done: bool,
}

#[derive(Insertable)]
#[diesel(table_name = branches)]
pub struct NewBranch<'a> {
    pub state: &'a str,
    pub distance: i32,
}
