use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};

use crate::{
    application::usecases::guild_commanders::GuildCommandersUseCase,
    domain::{
        repositories::guild_commanders::GuildCommandersRepository,
        value_objects::adventurer_model::RegisterAdventurerModel,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad, repositories::guild_commanders::GuildCommanderPostgres,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let guild_commander_repository = GuildCommanderPostgres::new(db_pool);
    let guild_commander_usecase = GuildCommandersUseCase::new(Arc::new(guild_commander_repository));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(guild_commander_usecase))
}

pub async fn register<T>(
    State(guild_commander_usecase): State<Arc<GuildCommandersUseCase<T>>>,
    Json(register_adventurer_model): Json<RegisterAdventurerModel>,
) -> impl IntoResponse
where
    T: GuildCommandersRepository + Sync + Send,
{
    unimplemented!()
}
