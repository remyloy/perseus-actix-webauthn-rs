use std::{collections::HashMap, sync::Arc};

use async_std::sync::Mutex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use webauthn_rs::prelude::*;

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// Since a user's username could change at anytime, we need to bind to a unique id.
    /// We use uuid's for this purpose, and you should generate these randomly. If the
    /// username does exist and is found, we can match back to our unique id. This is
    /// important in authentication, where presented credentials may *only* provide
    /// the unique id, and not the username!
    pub unique_id: Uuid,
    pub name: String,
    pub display_name: String,
}

#[derive(Debug)]
pub struct Users {
    pub name_to_id: HashMap<String, Uuid>,
    pub keys: HashMap<Uuid, Vec<Passkey>>,
}

pub struct AppState {
    pub config: Arc<Config>,
    pub webauthn: Arc<Webauthn>,
    pub users: Mutex<Users>,
}
