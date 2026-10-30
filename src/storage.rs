use std::error::Error;
use sea_orm::ActiveValue::Set;

use sea_orm::sea_query::IntoCondition;
use sea_orm::{ActiveModelBehavior, IntoActiveModel, ModelTrait, QueryFilter};
use sea_orm::{ActiveModelTrait, ConnectionTrait, DatabaseBackend, DatabaseConnection, EntityTrait, Schema, Statement, StatementBuilder, entity, sea_query::Table};

use crate::storage::models::ModelData;

mod models;


/// Specialized struct for managing db.
/// Stores only User, Folder or Recipe data.
pub struct DBManager{
    db: DatabaseConnection
}

impl DBManager {
    /// Create storage instance.
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

    /// Insert data that implements ModelData trait
    pub async fn insert_data<A>(&self, data: A)->Result<(), sea_orm::error::DbErr>
    where
        A: ActiveModelTrait + ActiveModelBehavior + Send + Sync,
        <<A as sea_orm::ActiveModelTrait>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<A> + ModelData
    {
        data.insert(&self.db).await?;
        Ok(())
    }

    pub async fn remove_data<A>(&self, data: A)->Result<(), sea_orm::error::DbErr>
    where
        A: ActiveModelTrait + Send + Sync + ActiveModelBehavior,
        <<A as sea_orm::ActiveModelTrait>::Entity as sea_orm::EntityTrait>::Model: ModelData
    {
        data.delete(&self.db).await?;
        Ok(())
    }

    pub async fn update_data<A>(&self, data: A)->Result<(), sea_orm::error::DbErr>
    where
        A: ActiveModelTrait + Send + Sync + ActiveModelBehavior,
        <<A as sea_orm::ActiveModelTrait>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<A> + ModelData
    {
        data.update(&self.db).await?;
        Ok(())
    }

    pub async fn find_by_id<E, T>(&self, id: T)->Result<Option<E::Model>, sea_orm::error::DbErr>
    where
        E: EntityTrait + Send + Sync,
        <E as EntityTrait>::Model: ModelData,
        T: Into<<<E as sea_orm::EntityTrait>::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType>
    {
        E::find_by_id(id).one(&self.db).await
    }

    pub async fn find<E, F>(&self, filter: F)->Result<Vec<E::Model>, sea_orm::error::DbErr>
    where
        E: EntityTrait + Send + Sync,
        <E as EntityTrait>::Model: ModelData,
        F: IntoCondition
    {
        E::find().filter(filter).all(&self.db).await
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

//Operations:
//User: add
//Folder: add, delete, update(rename, move)
//Recipe: add, delete, update(rename, move, change(notes, ingridients))

//TODO 4: Recipe recalc logic. Think + Make + Test + Release
//TODO 3: Database logic. Test + Release

//Tests
//TODO: Tests for all Module functions from models.

//Priority!
//TODO 2: UI logic (Web App). Make + Test + Release
//TODO 1: Backend logic. Think + Make + Test + Release
