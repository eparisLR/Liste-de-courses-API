use diesel::{Selectable, Queryable, Insertable, associations::Identifiable};
use serde::Serialize;

#[derive(Serialize, Selectable, Queryable, Insertable, Identifiable)]
#[diesel(table_name= crate::schema::categories)]
pub struct Category {
    id: i32,
    name: String
}