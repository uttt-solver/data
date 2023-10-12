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
