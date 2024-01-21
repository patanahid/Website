//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use async_graphql::SimpleObject;
use database::Visibility;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "collection")]
#[graphql(name = "Collection")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_on: DateTimeUtc,
    pub last_updated_on: DateTimeUtc,
    pub name: String,
    pub description: Option<String>,
    #[graphql(skip)]
    pub user_id: i32,
    pub visibility: Visibility,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::collection_to_entity::Entity")]
    CollectionToEntity,
    #[sea_orm(has_many = "super::review::Entity")]
    Review,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::collection_to_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CollectionToEntity.def()
    }
}

impl Related<super::review::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Review.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}