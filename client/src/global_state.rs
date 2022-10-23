use perseus::{state::GlobalStateCreator, RenderFnResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn get_global_state_creator() -> GlobalStateCreator {
    GlobalStateCreator::new().build_state_fn(get_build_state)
}

#[perseus::make_rx(AppStateRx)]
pub struct AppState {
    pub reg_state: AuthState,
    pub login_state: AuthState,
    pub error: String,
    pub user: Option<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthState {
    Yes,
    No,
    Server,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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

#[perseus::global_build_state]
pub async fn get_build_state() -> RenderFnResult<AppState> {
    Ok(AppState {
        reg_state: AuthState::Server,
        login_state: AuthState::Server,
        error: "".to_string(),
        user: None,
    })
}

#[cfg(target_arch = "wasm32")]
impl<'a> AppStateRx<'a> {
    pub fn load_identity_state(app_state: &AppStateRx<'a>, cx: sycamore::reactive::Scope<'a>) {
        use perseus::spawn_local_scoped;
        let app_state = app_state.clone();
        spawn_local_scoped(cx, async move {
            let user = Self::get_identity_state().await;
            Self::update_identity_state(app_state.user, app_state.error, &user);
        });
    }

    pub fn update_identity_state(
        user_signal: &'a sycamore::reactive::Signal<Option<User>>,
        _error_signal: &'a sycamore::reactive::Signal<String>,
        user: &anyhow::Result<User>,
    ) {
        use crate::log;

        let old_user = &*user_signal.get_untracked();
        log!("set user {:?} => {:?}", old_user, user);
        match user {
            Ok(user) => user_signal.set(Some(user.clone())),
            Err(e) => {
                user_signal.set(None);
                log!("error: {}", e);
                // error_signal.set(e.to_string());
            }
        }
    }

    pub async fn get_identity_state() -> anyhow::Result<User> {
        use crate::config::CONFIG;

        let jsval = crate::service::actions::get_json(CONFIG.identity_url).await?;
        let user = serde_wasm_bindgen::from_value(jsval)
            .map_err(|e| anyhow::Error::msg(format!("Failed to cast into User {}", e)))?;
        Ok(user)
    }
}
