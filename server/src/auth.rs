use crate::models::*;
use actix_session::Session;
use anyhow::Result;
use async_std::sync::MutexGuard;
use webauthn_rs::prelude::*;

/// Remove any previous registrations that may have occured from the session.
pub fn clear_reg_state(session: &Session) {
    session.remove("reg_state");
}

/// Note that due to the session store in use being a server side memory store, this is
/// safe to store the reg_state into the session since it is not client controlled and
/// not open to replay attacks. If this was a cookie store, this would be UNSAFE.
pub fn insert_reg_state(
    session: &Session,
    user: &User,
    reg_state: &PasskeyRegistration,
) -> anyhow::Result<()> {
    session
        .insert("reg_state", (user, reg_state))
        .map_err(|e| anyhow::Error::msg(format!("Failed to insert {}", e)))
}

pub fn get_reg_state(session: &Session) -> anyhow::Result<(User, PasskeyRegistration)> {
    session
        .get("reg_state")
        .map_err(|_| anyhow::Error::msg("Session missing"))
        .and_then(|s| s.ok_or_else(|| anyhow::Error::msg("Corrupt Session")))
}

pub fn clear_auth_state(session: &Session) {
    session.remove("auth_state");
}

pub fn insert_auth_state(
    session: &Session,
    user_unique_id: Uuid,
    auth_state: &PasskeyAuthentication,
) -> anyhow::Result<()> {
    session
        .insert("auth_state", (user_unique_id, auth_state))
        .map_err(|e| anyhow::Error::msg(format!("session update failed {}", e)))
}

pub fn get_auth_state(session: &Session) -> Result<(Uuid, PasskeyAuthentication)> {
    session
        .get("auth_state")
        .map_err(|_| anyhow::Error::msg("Session missing"))
        .and_then(|s| s.ok_or_else(|| anyhow::Error::msg("Corrupt missing")))
}

/// Look up their unique id from the username
pub fn name_to_id<'a>(users_guard: &'a MutexGuard<Users>, username: &str) -> Option<Uuid> {
    users_guard.name_to_id.get(username).copied()
}

/// If the user has any other credentials, we exclude these here so they can't be duplicate registered.
/// It also hints to the browser that only new credentials should be "blinked" for interaction.
pub fn get_existing_credentials(
    users_guard: &MutexGuard<Users>,
    user_unique_id: Uuid,
) -> Option<Vec<Base64UrlSafeData>> {
    get_allowed_credentials(users_guard, user_unique_id)
        .map(|keys| keys.iter().map(|sk| sk.cred_id().clone()).collect())
        .ok()
}

pub fn get_allowed_credentials<'a>(
    users_guard: &'a MutexGuard<Users>,
    user_unique_id: Uuid,
) -> anyhow::Result<&'a Vec<Passkey>> {
    users_guard
        .keys
        .get(&user_unique_id)
        .ok_or_else(|| anyhow::Error::msg("User has no credentials"))
}

pub async fn insert_user(state: &AppState, user: &User, sk: Passkey) {
    let mut users_guard = state.users.lock().await;

    users_guard
        .keys
        .entry(user.unique_id)
        .and_modify(|keys| keys.push(sk.clone()))
        .or_insert_with(|| vec![sk.clone()]);

    users_guard
        .name_to_id
        .insert(user.name.to_string(), user.unique_id);
}

pub fn get_user(users_guard: &MutexGuard<Users>, user_unique_id: Uuid) -> anyhow::Result<User> {
    let user_name = users_guard
        .name_to_id
        .iter()
        .find(|x| x.1 == &user_unique_id)
        .ok_or_else(|| anyhow::Error::msg("Failed to parse user unique identity"))?
        .0;
    Ok(User {
        unique_id: user_unique_id,
        name: user_name.to_string(),
        display_name: user_name.to_string(),
    })
}

pub fn start_passkey_registration(
    state: &AppState,
    user: &User,
    exclude_credentials: Option<Vec<Base64UrlSafeData>>,
) -> Result<(CreationChallengeResponse, PasskeyRegistration)> {
    state
        .webauthn
        .start_passkey_registration(
            user.unique_id,
            &user.name,
            &user.display_name,
            exclude_credentials,
        )
        .map_err(|e| anyhow::Error::msg(format!("start_passkey_registration failed {}", e)))
}

pub fn finish_passkey_registration(
    state: &AppState,
    reg: &RegisterPublicKeyCredential,
    reg_state: &PasskeyRegistration,
) -> anyhow::Result<Passkey> {
    state
        .webauthn
        .finish_passkey_registration(reg, reg_state)
        .map_err(|e| anyhow::Error::msg(format!("finish_passkey_registration failed {}", &e)))
}

pub async fn update_credential(
    state: &AppState,
    user_unique_id: Uuid,
    auth_result: &AuthenticationResult,
) -> Result<()> {
    let mut users_guard = state.users.lock().await;
    users_guard
        .keys
        .get_mut(&user_unique_id)
        .map(|keys| {
            keys.iter_mut().for_each(|sk| {
                // This will update the credential if it's the matching
                // one. Otherwise it's ignored. That is why it is safe to
                // iterate this over the full list.
                sk.update_credential(auth_result);
            })
        })
        .ok_or_else(|| anyhow::Error::msg("User has no credentials"))
}

pub fn start_passkey_authentication(
    state: &AppState,
    allow_credentials: &[Passkey],
) -> anyhow::Result<(RequestChallengeResponse, PasskeyAuthentication)> {
    state
        .webauthn
        .start_passkey_authentication(allow_credentials)
        .map_err(|e| anyhow::Error::msg(format!("passkey authentication failed {}", e)))
}

pub fn finish_passkey_authentication(
    state: &AppState,
    auth: &PublicKeyCredential,
    auth_state: &PasskeyAuthentication,
) -> Result<AuthenticationResult> {
    state
        .webauthn
        .finish_passkey_authentication(auth, auth_state)
        .map_err(|e| anyhow::Error::msg(format!("finish passkey authentication failed {}", e)))
}
