use axum::{extract::Path, http::StatusCode, Json};
use diesel::{insert_into, QueryDsl, SelectableHelper, BelongingToDsl};
use diesel_async::RunQueryDsl;

use crate::{connection::{internal_error, DatabaseConnection}, model::{Category, Ingredient, NewRecipe, Recipe, RecipeCategories, RecipeIngredients}, schema::{self, categories, ingredients, recipes}};

pub async fn get_recipes(DatabaseConnection(mut conn): DatabaseConnection) -> Result<Json<Vec<Recipe>>, (StatusCode, String)> {
    let res = recipes::table
        .select(Recipe::as_select())
        .load(&mut conn)
        .await
        .map_err(internal_error)?;
    
    Ok(Json(res))
}

pub async fn insert_one_recipe(DatabaseConnection(mut conn): DatabaseConnection, Json(payload): Json<NewRecipe>) -> Result<(), (StatusCode, String)> {
    insert_into(schema::recipes::table)
        .values(&payload)
        .execute(&mut conn)
        .await
        .map_err(internal_error)?;

    Ok(())
}

pub async fn get_recipe_ingredients(DatabaseConnection(mut conn): DatabaseConnection, Path(recipe_id_param): Path<i32>) -> Result<Json<Vec<Ingredient>>, (StatusCode, String)> {
    let recipe = recipes::table
        .find(recipe_id_param)
        .select(Recipe::as_select())
        .get_result(&mut conn)
        .await
        .map_err(internal_error)?;

    let res = RecipeIngredients::belonging_to(&recipe)
        .inner_join(ingredients::table)
        .select(Ingredient::as_select())
        .load(&mut conn)
        .await
        .map_err(internal_error)?;

    Ok(Json(res))
}

pub async fn get_recipe_categories(DatabaseConnection(mut conn): DatabaseConnection, Path(recipe_id_param): Path<i32>) -> Result<Json<Vec<Category>>, (StatusCode, String)> {
    let recipe = recipes::table
        .find(recipe_id_param)
        .select(Recipe::as_select())
        .get_result(&mut conn)
        .await
        .map_err(internal_error)?;

    let res = RecipeCategories::belonging_to(&recipe)
        .inner_join(categories::table)
        .select(Category::as_select())
        .load(&mut conn)
        .await
        .map_err(internal_error)?;

    Ok(Json(res))
}