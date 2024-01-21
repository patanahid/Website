//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use async_graphql::SimpleObject;
use async_trait::async_trait;
use chrono::{NaiveDate, Utc};
use database::SeenState;
use sea_orm::{entity::prelude::*, ActiveValue};
use sea_query::Expr;
use serde::{Deserialize, Serialize};

use crate::{
    entities::{prelude::UserToEntity, user_to_entity},
    models::media::{
        SeenOrReviewOrCalendarEventExtraInformation, SeenPodcastExtraInformation,
        SeenShowExtraInformation,
    },
    utils::associate_user_with_metadata,
};

// When updating a media item's progress, here are the things that should happen:
// - remove from watchlist if it was in there
// - add to in progress
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, SimpleObject)]
#[graphql(name = "Seen")]
#[sea_orm(table_name = "seen")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub progress: i32,
    pub started_on: Option<NaiveDate>,
    pub finished_on: Option<NaiveDate>,
    pub user_id: i32,
    pub metadata_id: i32,
    pub state: SeenState,
    #[graphql(skip)]
    #[serde(skip)]
    pub updated_at: Vec<DateTimeUtc>,
    #[graphql(skip)]
    #[serde(skip)]
    pub extra_information: Option<SeenOrReviewOrCalendarEventExtraInformation>,
    #[sea_orm(ignore)]
    pub show_information: Option<SeenShowExtraInformation>,
    #[sea_orm(ignore)]
    pub podcast_information: Option<SeenPodcastExtraInformation>,
    // Generated columns
    pub last_updated_on: DateTimeUtc,
    pub num_times_updated: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::metadata::Entity",
        from = "Column::MetadataId",
        to = "super::metadata::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Metadata,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::metadata::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Metadata.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let state = self.state.clone().unwrap();
        let progress = self.progress.clone().unwrap();
        if progress == 100 && state == SeenState::InProgress {
            self.state = ActiveValue::Set(SeenState::Completed);
        }
        Ok(self)
    }

    async fn after_save<C>(model: Model, db: &C, insert: bool) -> Result<Model, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            associate_user_with_metadata(&model.user_id, &model.metadata_id, db)
                .await
                .ok();
        }
        UserToEntity::update_many()
            .filter(user_to_entity::Column::UserId.eq(model.user_id))
            .filter(user_to_entity::Column::MetadataId.eq(model.metadata_id))
            .col_expr(
                user_to_entity::Column::LastUpdatedOn,
                Expr::value(Utc::now()),
            )
            .exec(db)
            .await?;
        Ok(model)
    }
}