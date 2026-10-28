use std::error::Error;
use sea_orm::ActiveValue::Set;

use sea_orm::{ActiveModelBehavior, IntoActiveModel};
use sea_orm::{ActiveModelTrait, ConnectionTrait, DatabaseBackend, DatabaseConnection, EntityTrait, Schema, Statement, StatementBuilder, entity, sea_query::Table};

mod models;


pub struct DBManager{
    db: DatabaseConnection
}

impl DBManager {
    pub async fn new(url: &str) -> Self{
        DBManager { db: sea_orm::Database::connect(url).await.unwrap() }
    }

    fn create_table_if_not_exists_statement(schema: &Schema, entity: impl EntityTrait)->impl StatementBuilder{
        let mut statement = schema.create_table_from_entity(entity);
        statement.if_not_exists();
        statement
    }

    pub async fn create_tables(& mut self)->Result<(), sea_orm::error::DbErr>{
        let builder = self.db.get_database_backend();
        let schema = Schema::new(builder);

        self.db.execute(&DBManager::create_table_if_not_exists_statement(&schema, models::User)).await?;
        self.db.execute(&DBManager::create_table_if_not_exists_statement(&schema, models::Folder)).await?;
        self.db.execute(&DBManager::create_table_if_not_exists_statement(&schema, models::Recipe)).await?;

        Ok(())
    }

    pub async fn insert_data<A>(&self, data: A)->Result<(), sea_orm::error::DbErr>
    where
        A: models::ActiveModelData + ActiveModelTrait + ActiveModelBehavior + Send + Sync,
        <<A as sea_orm::ActiveModelTrait>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<A>
    {
        data.insert(&self.db).await?;
        Ok(())
    }

    pub async fn drop_tables(& mut self)->Result<(), sea_orm::error::DbErr>{
        let builder = self.db.get_database_backend();

        self.db.execute(Table::drop().table(models::User).if_exists().cascade()).await?;
        self.db.execute(Table::drop().table(models::Folder).if_exists().cascade()).await?;
        self.db.execute(Table::drop().table(models::Recipe).if_exists().cascade()).await?;

        Ok(())
    }
    
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_insert_data(){
        let user = models::UserActiveModel::new(23, 33, "Antony");
        // insert_data(user);
        
    }
}