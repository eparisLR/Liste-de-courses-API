use axum::{http::StatusCode, Json};
use diesel::{QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

use crate::{connection::{internal_error, DatabaseConnection}, model::Recipe, schema::recipes};

pub async fn get_recipes(DatabaseConnection(mut conn): DatabaseConnection,) -> Result<Json<Vec<Recipe>>, (StatusCode, String)> {
    let res = recipes::table
        .select(Recipe::as_select())
        .load(&mut conn)
        .await
        .map_err(internal_error)?;
    
    Ok(Json(res))
}