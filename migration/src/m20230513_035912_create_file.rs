use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Use `execute_unprepared` if the SQL statement doesn't have value bindings
        let mut sql =  r#"
        CREATE TABLE "file" (
            "id" bigserial primary key not null,
            "filename" varchar(255) not null,
            "content" text,
            "user_id" bigint not null,
            "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
            "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
            CONSTRAINT "file_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE
          );"#;
          db.execute_unprepared(sql)
          .await?;

        sql= r#"CREATE INDEX "idx_file_user_id" ON "file" ("user_id")"#;
        db.execute_unprepared(sql)
        .await?;

        sql= r#"CREATE INDEX idx_file_filename_trigram
        ON "file" USING gin (filename gin_trgm_ops)"#;
        db.execute_unprepared(sql)
        .await?;

        sql=r#"CREATE INDEX idx_file_content ON "file" USING gin(to_tsvector('english', content))"#;
        db.execute_unprepared(sql)
        .await?;

    //    sql=r#"
    //    CONSTRAINT "file_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "user" ("id")  ON DELETE CASCADE
    //    "#;
    //    db.execute_unprepared(sql)
    //    .await?;





        Ok(())
    

        
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();
        let mut sql = r#"ALTER TABLE "file" DROP CONSTRAINT "file_user_id_fkey" "#;
        db.execute_unprepared(sql)
        .await?;

        sql=r#"DROP INDEX idx_file_user_id"#;
        db.execute_unprepared(sql)
        .await?;

        sql= r#"DROP INDEX idx_file_content"#;
        db.execute_unprepared(sql)
        .await?;

        sql=r#"DROP INDEX idx_file_filename_trigram"#;
        db.execute_unprepared(sql)
        .await?;

        sql=r#"
        DROP TABLE "file";
        "#;
        db.execute_unprepared(sql)
        .await?;

        Ok(())

    }
}

