pub mod cookie;

use crate::request_context::driver::cookie::CookieMap;
use serde::de::DeserializeOwned;
use tower_sessions::Session;

pub const PRIVATE_COOKIE_NAME: &str = "__loco_app_session";

#[derive(Debug, Clone)]
pub enum Driver {
    TowerSession(Session),
    CookieMap(CookieMap),
}

impl Driver {
    /// Inserts a `impl Serialize` value into the session.
    /// # Arguments
    /// * `key` - The key to store the value
    /// * `value` - The value to store
    /// # Errors
    /// * `CookieMapError` - When the value is unable to be serialized
    /// * `TowerSessionError` - When the value is unable to be serialized or if the session has not been hydrated and loading from the store fails, we fail with `Error::Store`
    pub async fn insert<T>(&mut self, key: &str, value: T) -> Result<(), DriverError>
    where
        T: serde::Serialize + Send + Sync,
    {
        match self {
            Self::CookieMap(cookie_map) => {
                cookie_map.insert(key, value)?;
            }
            Self::TowerSession(session) => {
                session.insert(key, value).await?;
            }
        }
        Ok(())
    }

    /// Gets a `impl DeserializeOwned` value from the session.
    /// # Arguments
    /// * `key` - The key to get the value from
    /// # Returns
    /// * `Option<T>` - The value if it exists
    /// # Errors
    /// * `CookieMapError` - When the value is unable to be deserialized
    /// * `TowerSessionError` - When the value is unable to be deserialized or if the session has not been hydrated and loading from the store fails, we fail with `Error::Store`
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, DriverError> {
        match self {
            Self::CookieMap(cookie_map) => Ok(cookie_map.get(key)?),
            Self::TowerSession(session) => Ok(session.get(key).await?),
        }
    }

    /// Removes a `serde_json::Value` from the session.
    ///
    /// # Arguments
    /// * `key` - The key to remove from the session
    ///
    /// # Return
    /// * `Option<T>` - The value if it exists
    ///
    /// # Errors
    /// * `CookieMapError` - When the value is unable to be deserialized
    /// * `TowerSessionError` - When the value is unable to be deserialized or if the session has not been hydrated and loading from the store fails, we fail with `Error::Store`
    pub async fn remove<T: DeserializeOwned>(
        &mut self,
        key: &str,
    ) -> Result<Option<T>, DriverError> {
        match self {
            Self::CookieMap(cookie_map) => Ok(cookie_map.remove(key)?),
            Self::TowerSession(session) => Ok(session.remove(key).await?),
        }
    }

    /// Clears the session.
    pub async fn clear(&mut self) {
        match self {
            Self::CookieMap(cookie_map) => {
                cookie_map.clear();
            }
            Self::TowerSession(session) => {
                session.clear().await;
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DriverError {
    #[error("CookieMapError: {0}")]
    CookieMapError(#[from] cookie::CookieMapError),
    #[error("TowerSessionError: {0}")]
    TowerSessionError(#[from] tower_sessions::session::Error),
}
