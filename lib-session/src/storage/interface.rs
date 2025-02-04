use std::{collections::HashMap, future::Future};

use actix_web::cookie::time::Duration;
use derive_more::derive::Display;

use super::SessionKey;

pub type SessionState = HashMap<String, String>;

/// The interface to retrieve and save the current session data from/to the chosen storage backend.
///
/// You can provide your own custom session store backend by implementing this trait.
pub trait SessionStore {
    /// Loads the session state associated to a session key.
    fn load(
        &self,
        session_key: &SessionKey,
    ) -> impl Future<Output = Result<Option<SessionState>, LoadError>>;

    /// Persist the session state for a newly created session.
    ///
    /// Returns the corresponding session key.
    fn save(
        &self,
        session_state: SessionState,
        ttl: &Duration,
    ) -> impl Future<Output = Result<SessionKey, SaveError>>;

    /// Updates the session state associated to a pre-existing session key.
    fn update(
        &self,
        session_key: SessionKey,
        session_state: SessionState,
        ttl: &Duration,
    ) -> impl Future<Output = Result<SessionKey, UpdateError>>;

    /// Updates the TTL of the session state associated to a pre-existing session key.
    fn update_ttl(
        &self,
        session_key: &SessionKey,
        ttl: &Duration,
    ) -> impl Future<Output = Result<(), anyhow::Error>>;

    /// Deletes a session from the store.
    fn delete(&self, session_key: &SessionKey) -> impl Future<Output = Result<(), anyhow::Error>>;
}

#[derive(Debug, Display)]
pub enum LoadError {
    #[display("Failed to deserialize session state")]
    Deserialization(anyhow::Error),

    #[display("Something went wrong when retrieving the session state")]
    Other(anyhow::Error),
}

impl std::error::Error for LoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Deserialization(err) => Some(err.as_ref()),
            Self::Other(err) => Some(err.as_ref()),
        }
    }
}

#[derive(Debug, Display)]
pub enum SaveError {
    #[display("Failed to serialize session state")]
    Serialization(anyhow::Error),

    #[display("Something went wrong when persisting the session state")]
    Other(anyhow::Error),
}

impl std::error::Error for SaveError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Serialization(err) => Some(err.as_ref()),
            Self::Other(err) => Some(err.as_ref()),
        }
    }
}

#[derive(Debug, Display)]
pub enum UpdateError {
    #[display("Failed to serialize session state")]
    Serialization(anyhow::Error),

    #[display("Something went wrong when updating the session state.")]
    Other(anyhow::Error),
}

impl std::error::Error for UpdateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Serialization(err) => Some(err.as_ref()),
            Self::Other(err) => Some(err.as_ref()),
        }
    }
}
