//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub username: String,
    pub password: String,
    pub status: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_vote::Entity")]
    UserVote,
    #[sea_orm(has_many = "super::voting::Entity")]
    Voting,
}

impl Related<super::user_vote::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserVote.def()
    }
}

impl Related<super::voting::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Voting.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
