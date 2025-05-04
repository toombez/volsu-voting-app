use crate::m20220101_000001_create_table::User;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .create_table(
                Table::create()
                    .table(Voting::Table)
                    .if_not_exists()
                    .col(pk_uuid(Voting::Id))
                    .col(string(Voting::Title))
                    .col(string(Voting::Text))
                    .col(uuid(Voting::AuthorId))
                    .col(date_time(Voting::CreatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_voting_author")
                            .from(Voting::Table, Voting::AuthorId)
                            .to(User::Table, User::Id)
                    )
                    .to_owned()
            )
            .await?;

        let _ = manager
            .create_table(
                Table::create()
                    .table(UserVote::Table)
                    .if_not_exists()
                    .col(pk_auto(UserVote::Id))
                    .col(uuid(UserVote::UserId))
                    .col(date_time(UserVote::CreatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_vote_user")
                            .from(UserVote::Table, UserVote::UserId)
                            .to(User::Table, User::Id)
                    )
                    .col(uuid(UserVote::VotingId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_vote_voting")
                            .from(UserVote::Table, UserVote::VotingId)
                            .to(Voting::Table, Voting::Id)
                    )
                    .to_owned()
            ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager.drop_table(
            Table::drop()
                .table(Voting::Table)
                .to_owned()
        ).await?;

        let _ = manager.drop_table(
            Table::drop()
                .table(UserVote::Table)
                .to_owned()
        ).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Voting {
    Table,
    Id,
    Title,
    Text,
    AuthorId,
    CreatedAt,
}

#[derive(DeriveIden)]
pub enum UserVote {
    Table,
    Id,
    UserId,
    VotingId,
    CreatedAt,
}
