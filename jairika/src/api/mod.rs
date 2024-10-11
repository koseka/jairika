pub mod architecture;
pub mod engine;

use crate::Store;
use crate::{Error, Result};

use std::path::PathBuf;
use surrealdb::engine::local::Db;
use surrealdb::engine::local::SurrealKV;
use surrealdb::Surreal;

/// Creates a new model at the specified address.
///
/// ### Example
///
/// ```no_run
/// use jairika as jrk;
///
/// #[tokio::main]
/// async fn main() {
///     jrk::init("path/to/model/").await.unwrap();
/// }
/// ```
pub async fn init(address: &str) -> Result<()> {
    let path = PathBuf::from(address);

    if !path.exists() {
        return Err(Error::ModelAddressNotFound(address.to_string()));
    }

    if !path.is_dir() {
        return Err(Error::InvalidModelAddress(address.to_string()));
    }

    Store::new(address).map_err(|e| Error::ModelCreationFailed(e.to_string()))?;

    let conn = Surreal::new::<SurrealKV>(address)
        .await
        .map_err(|e| Error::ModelCreationFailed(e.to_string()))?;

    conn.query("DEFINE NAMESPACE main")
        .await
        .map_err(|e| Error::ModelCreationFailed(e.to_string()))?;

    conn.query("USE NS main")
        .await
        .map_err(|e| Error::ModelCreationFailed(e.to_string()))?;

    conn.query("DEFINE DATABASE prime")
        .await
        .map_err(|e| Error::ModelCreationFailed(e.to_string()))?;

    Ok(())
}

/// A machine learning model.
pub struct Model {
    pub(crate) entry: Surreal<Db>,
    pub(crate) address: Option<String>,
    pub(crate) activations: Option<Vec<Vec<f64>>>,
    pub(crate) layers: Option<Vec<Vec<f64>>>,
}

impl Model {
    /// Instantiates a model.
    ///
    /// ### Example
    ///
    /// ```no_run
    /// use jairika as jrk;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     jrk::init("path/to/model/").await.unwrap();
    ///     let model = jrk::Model::new("path/to/model/").await.unwrap();
    /// }
    /// ```
    pub async fn new(address: &str) -> Result<Self> {
        match Surreal::new::<SurrealKV>(address).await {
            Ok(entry) => {
                //TODO: Find a way to determine what the main branch is and what the last iteration is.
                entry
                    .query("USE NS main")
                    .await
                    .map_err(|e| Error::ModelInstantiationFailed(e.to_string()))?;
                entry
                    .query("USE DB prime")
                    .await
                    .map_err(|e| Error::ModelInstantiationFailed(e.to_string()))?;

                Ok(Model {
                    entry: entry,
                    address: Some(address.to_string()),
                    activations: None,
                    layers: None,
                })
            }
            Err(e) => Err(Error::ModelInstantiationFailed(e.to_string())),
        }
    }

    /// Switches the active variant of the model.
    ///
    /// ### Example
    ///
    /// ```no_run
    /// use jairika as jrk;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     jrk::init("path/to/model/").await.unwrap();
    ///     let mut model = jrk::Model::new("path/to/model/").await.unwrap();
    ///     model.switch("variant").await.unwrap();
    /// }
    /// ```
    pub async fn switch(&mut self, variant: &str) -> Result<()> {
        self.entry
            .query(&format!("USE NS {}", variant))
            .await
            .map_err(|e| Error::ModelInstantiationFailed(e.to_string()))?;

        //TODO: Find a way to determine what the last iteration is.
        self.entry
            .query("USE DB prime")
            .await
            .map_err(|e| Error::ModelInstantiationFailed(e.to_string()))?;

        Ok(())
    }

    /// Returns the address of the model.
    pub fn address(&self) -> Option<&str> {
        self.address.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init() {
        init("test_model").await.unwrap();
        assert!(std::path::Path::new("test_model").exists());
    }

    #[tokio::test]
    async fn test_model_new() {
        let model = Model::new("test_model").await.unwrap();
        assert_eq!(model.address(), Some("test_model"));
    }
}
