use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, patch, post},
    Extension, Json, Router,
};

use crate::{
    application::usecases::quest_ops::QuestUseCase,
    domain::{
        repositories::{quest_ops::QuestRepository, quest_viewing::QuestViewingRepository},
        value_objects::quest_model::{AddQuestModel, EditQuestModel},
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad,
        repositories::{quest_ops::QuestPostgres, quest_viewing::QuestViewingPostgres},
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let quest_repository = QuestPostgres::new(Arc::clone(&db_pool));
    let quest_viewing_repository = QuestViewingPostgres::new(Arc::clone(&db_pool));

    let quest_usecase = QuestUseCase::new(
        Arc::new(quest_repository),
        Arc::new(quest_viewing_repository),
    );

    Router::new()
        .route("/", post(add))
        .route("/:quest_id", patch(edit))
        .route("/:quest_id", delete(remove))
        .with_state(Arc::new(quest_usecase))
}

pub async fn add<T1, T2>(
    State(quest_usecase): State<Arc<QuestUseCase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Json(add_quest_model): Json<AddQuestModel>,
) -> impl IntoResponse
where
    T1: QuestRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    unimplemented!()
}

pub async fn edit<T1, T2>(
    State(quest_usecase): State<Arc<QuestUseCase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>,
    Json(edit_quest_model): Json<EditQuestModel>,
) -> impl IntoResponse
where
    T1: QuestRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    unimplemented!()
}

pub async fn remove<T1, T2>(
    State(quest_usecase): State<Arc<QuestUseCase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: QuestRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    unimplemented!()
}
