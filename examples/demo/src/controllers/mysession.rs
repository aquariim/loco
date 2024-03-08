#![allow(clippy::unused_async)]
use axum_session::{Session, SessionNullPool};
use loco_rs::prelude::*;

/// Get a session
///
/// # Errors
///
/// This function will return an error if result fails
pub async fn get_session(_session: Session<SessionNullPool>) -> Result<()> {
    format::empty()
}

pub fn routes() -> Routes {
    Routes::new().prefix("mysession").add("/", get(get_session))
}
