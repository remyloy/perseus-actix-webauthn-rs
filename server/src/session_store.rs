use actix_session::storage::{LoadError, SaveError, SessionKey, SessionStore, UpdateError};
use actix_web::cookie::time::Duration;
use log::debug;
use std::{cell::RefCell, collections::HashMap};
use uuid::Uuid;

pub(crate) type SessionState = HashMap<String, String>;

#[derive(Default)]
pub struct MemorySessionStore {
    store: RefCell<HashMap<String, SessionState>>,
}

#[async_trait::async_trait(?Send)]
impl SessionStore for MemorySessionStore {
    async fn load(&self, session_key: &SessionKey) -> Result<Option<SessionState>, LoadError> {
        debug!("load {:?}", session_key);
        let key: String = session_key.as_ref().to_string();
        let session_state = self.store.borrow().get(&key).map(|x| x.to_owned());
        Ok(session_state)
    }

    async fn save(
        &self,
        session_state: SessionState,
        ttl: &Duration,
    ) -> Result<SessionKey, SaveError> {
        debug!("save {:?} ttl {}", session_state, ttl);
        let key = Uuid::new_v4().to_string();
        let session_key: SessionKey = key
            .clone()
            .try_into()
            .map_err(|e| SaveError::Other(anyhow::Error::from(e)))?;
        self.store
            .try_borrow_mut()
            .map_err(|e| SaveError::Other(anyhow::Error::from(e)))?
            .insert(key, session_state);
        Ok(session_key)
    }

    async fn update(
        &self,
        session_key: SessionKey,
        session_state: SessionState,
        ttl: &Duration,
    ) -> Result<SessionKey, UpdateError> {
        debug!("update {:?} {:?} ttl {}", session_key, session_state, ttl);
        let key: String = session_key.as_ref().to_string();
        self.store
            .try_borrow_mut()
            .map_err(|e| UpdateError::Other(anyhow::Error::from(e)))?
            .insert(key, session_state);
        Ok(session_key)
    }

    async fn update_ttl(
        &self,
        session_key: &SessionKey,
        ttl: &Duration,
    ) -> Result<(), anyhow::Error> {
        debug!("update_ttl {:?} ttl {}", session_key, ttl);
        Ok(())
    }

    async fn delete(&self, session_key: &SessionKey) -> Result<(), anyhow::Error> {
        debug!("delete {:?}", session_key);
        let key: String = session_key.as_ref().to_string();
        self.store
            .try_borrow_mut()
            .map_err(|e| UpdateError::Other(anyhow::Error::from(e)))?
            .remove(&key)
            .ok_or_else(|| {
                UpdateError::Other(anyhow::Error::msg("Could not remove session_key"))
            })?;
        Ok(())
    }
}
