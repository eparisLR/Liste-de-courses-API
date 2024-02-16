use diesel::{Selectable, Queryable, Insertable};
use serde::Serialize;

#[derive(Serialize, Selectable, Queryable, Insertable)]
#[diesel(table_name= crate::schema::ingredients)]
pub struct Ingredient {
    id: i32,
    name: String
}