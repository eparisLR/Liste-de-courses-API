use diesel::{associations::Associations, Identifiable, Queryable, Selectable};
use serde::Serialize;

use crate::model::Recipe;

#[derive(Serialize, Selectable, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Recipe, foreign_key = recipe_id))]
#[diesel(table_name= crate::schema::steps)]
pub struct Step {
    id: i32,
    instructions: String,
    recipe_id: i32
}