use diesel::{associations::Associations, Identifiable, Queryable, Selectable, Insertable};
use serde::Serialize;

use crate::model::Recipe;

#[derive(Serialize, Selectable, Queryable, Insertable, Identifiable, Associations)]
#[diesel(belongs_to(Recipe, foreign_key = recipe_id))]
#[diesel(table_name= crate::schema::steps)]
pub struct Step {
    id: i32,
    instructions: String,
    recipe_id: i32
}