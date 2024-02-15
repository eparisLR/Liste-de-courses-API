use diesel::{Selectable, Queryable, Insertable};
use serde::Serialize;

#[derive(Serialize, Selectable, Queryable, Insertable)]
#[diesel(table_name= Recipes)]
pub struct Recipe {
    id: i32,
    name: String,
    ingredients: Vec<Ingredient>,
    categories: Vec<Category>,
    steps: Vec<Step>
}