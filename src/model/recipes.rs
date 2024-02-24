use diesel::{associations::{Associations, Identifiable}, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use crate::model::{Ingredient, Category};

#[derive(Serialize, Selectable, Queryable, Identifiable)]
#[diesel(table_name= crate::schema::recipes)]
pub struct Recipe {
    id: i32,
    name: String
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name= crate::schema::recipes)]
pub struct NewRecipe {
    name: String
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Recipe, foreign_key = recipe_id))]
#[diesel(belongs_to(Ingredient, foreign_key = ingredient_id))]
#[diesel(table_name= crate::schema::recipesingredients)]
pub struct RecipeIngredients {
    id: i32,
    recipe_id: i32,
    ingredient_id: i32
}

#[derive(Serialize, Selectable, Queryable, Insertable, Identifiable, Associations)]
#[diesel(belongs_to(Recipe, foreign_key = recipe_id))]
#[diesel(belongs_to(Category, foreign_key = category_id))]
#[diesel(table_name= crate::schema::recipescategories)]
pub struct RecipeCategories {
    id: i32,
    recipe_id: i32,
    category_id: i32
}