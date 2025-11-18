use crate::domain::{
    repositories::{quest_ops::QuestRepository, quest_viewing::QuestViewingRepository},
    value_objects::quest_model::{AddQuestModel, EditQuestModel},
};
use anyhow::Result;
use std::sync::Arc;

pub struct QuestUseCase<T1, T2>
where
    T1: QuestRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    quest_repository: Arc<T1>,
    quest_viewing_repository: Arc<T2>
}

impl<T1, T2> QuestUseCase<T1, T2>
where
    T1: QuestRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync
{
    pub fn new(quest_repository: Arc<T1>, quest_viewing_repository: Arc<T2>) -> Self {
        Self { quest_repository, quest_viewing_repository }
    }

    pub async fn add(&self, guild_commander_id: i32, add_quest_model: AddQuestModel) -> Result<i32> {
        unimplemented!()
    }

    pub async fn edit(&self, guild_commander_id: i32, edit_quest_model: EditQuestModel) -> Result<i32> {
        unimplemented!()
    }

    pub async fn remove(&self, guild_commander_id: i32) -> Result<()> {
        unimplemented!()
    }
}
