use axum::{async_trait, extract::{FromRef, FromRequestParts}, http::{request::Parts, StatusCode}, Json};
use diesel::{QueryDsl, SelectableHelper};
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl};

use crate::{model::Recipe, schema::recipes};

pub type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub struct DatabaseConnection(
    bb8::PooledConnection<'static, AsyncDieselConnectionManager<AsyncPgConnection>>,
);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    S: Send + Sync,
    Pool: FromRef<S>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = Pool::from_ref(state);

        let conn = pool.get_owned().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

pub async fn get_recipes(DatabaseConnection(mut conn): DatabaseConnection,) -> Result<Json<Vec<Recipe>>, (StatusCode, String)> {
    let res = recipes::table
        .select(Recipe::as_select())
        .load(&mut conn)
        .await
        .map_err(internal_error)?;
    
    Ok(Json(res))
}