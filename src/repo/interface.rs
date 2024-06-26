use axum::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::errors::Result;

/* Trait for database interface operations */
#[async_trait]
pub trait DBInterface {
    /* Method to insert a record into the database */
    async fn insert_record<T: Serialize + Sync, U: DeserializeOwned + Sync + Clone>(
        &self,
        tb_name: String,
        data: &T,
    ) -> Result<Option<U>>;

    /* Method to select records from the database */
    async fn select<T: DeserializeOwned + Sync>(&self, tb_name: String) -> Result<Vec<T>>;

    /* Method to delete a record from the database */
    async fn delete(&self, id: String) -> Result<bool>;

    /* Method to update a record into the database */
    async fn update_record<T: Serialize + for<'de> Deserialize<'de> + Sync>(
        &self,
        id: String,
        tb_name: String,
        data: &T,
    ) -> Result<bool>;

    /* Method to select records from the database */
    async fn select_where<T: DeserializeOwned + Sync>(
        &self,
        tb_name: String,
        filter: String,
        columns: String, // separate columns by ',' in string format
    ) -> Result<Vec<T>>;
}
