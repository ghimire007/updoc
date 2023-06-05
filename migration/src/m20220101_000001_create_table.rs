use sea_orm_migration::prelude::*;
use crate::sea_orm::Statement;

#[derive(Debug,DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Use `execute_unprepared` if the SQL statement doesn't have value bindings
        let mut sql =  r#"CREATE TABLE "users" (
            "id" bigserial primary key  not null,
            "username" varchar(255) unique not null,
            "password" varchar(255) not null
          )"#;
          db.execute_unprepared(sql)
          .await?;
        sql= r#"CREATE INDEX idx_users_username_trigram
        ON "users" USING gin (username gin_trgm_ops)"#;
        db.execute_unprepared(sql)
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let mut sql=r#"DROP INDEX idx_users_username_trigram "#;
        db.execute_unprepared(sql)
        .await?; 

       let mut sql = r#"
        DROP TABLE "users"
          "#;
        db.execute_unprepared(sql)
          .await?;

    
        
        Ok(())
 
    }
}


