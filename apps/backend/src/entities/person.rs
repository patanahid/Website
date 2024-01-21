//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use async_graphql::SimpleObject;
use chrono::NaiveDate;
use database::MetadataSource;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::media::MetadataImage;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, SimpleObject)]
#[graphql(name = "Person")]
#[sea_orm(table_name = "person")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub identifier: String,
    pub source: MetadataSource,
    pub created_on: DateTimeUtc,
    pub last_updated_on: DateTimeUtc,
    pub name: String,
    #[sea_orm(column_type = "Json")]
    #[graphql(skip)]
    pub images: Option<Vec<MetadataImage>>,
    #[sea_orm(ignore)]
    pub display_images: Vec<String>,
    pub description: Option<String>,
    pub gender: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub death_date: Option<NaiveDate>,
    pub place: Option<String>,
    pub website: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::collection_to_entity::Entity")]
    CollectionToEntity,
    #[sea_orm(has_many = "super::metadata_to_person::Entity")]
    MetadataToPerson,
    #[sea_orm(has_many = "super::review::Entity")]
    Review,
}

impl Related<super::collection_to_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CollectionToEntity.def()
    }
}

impl Related<super::metadata_to_person::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MetadataToPerson.def()
    }
}

impl Related<super::review::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Review.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}