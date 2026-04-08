use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("todos")
                    .if_not_exists()
                    .col(pk_uuid("id").default(Expr::cust("uuidv7()")))
                    .col(timestamp("created_at").not_null().default(Expr::current_timestamp()))
                    .col(timestamp("updated_at").not_null().default(Expr::current_timestamp()))
                    .col(timestamp("deleted_at").null())
                    .col(string("title").not_null())
                    .col(timestamp("due_date").not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table("todos").to_owned()).await
    }
}
