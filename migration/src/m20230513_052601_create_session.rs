use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let mut sql =  r#"
        CREATE TABLE "session" (
            "id" bigserial primary key not null,
            "uuid" varchar(255) unique not null,
            "user_id" bigint not null,
            "attendees" bigint[],
            "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
            "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
            CONSTRAINT "session_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE
         

          );"#;
          db.execute_unprepared(sql)
          .await?;

        sql= r#"CREATE INDEX "idx_session_user_id" ON "session" ("user_id")"#;
        db.execute_unprepared(sql)
        .await?;
        

        
        sql=r#"CREATE INDEX "idx_session_attendees" ON "session" USING gin ("attendees")"#;
        db.execute_unprepared(sql)
        .await?;

  





        Ok(())
    






       
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let mut sql = r#"ALTER TABLE "session" DROP CONSTRAINT "session_user_id_fkey" "#;
        db.execute_unprepared(sql)
        .await?;

        // sql = r#"ALTER TABLE "session" DROP CONSTRAINT "session_attendees_fkey" "#;
        // db.execute_unprepared(sql)
        // .await?;

        sql= r#"DROP INDEX "idx_session_user_id""#;
        db.execute_unprepared(sql)
        .await?;

        sql=r#"DROP INDEX "idx_session_attendees""#;
        db.execute_unprepared(sql)
        .await?;

        sql=r#"
        DROP TABLE "session";
        "#;
        db.execute_unprepared(sql)
        .await?;

        Ok(())

       
    }
}

