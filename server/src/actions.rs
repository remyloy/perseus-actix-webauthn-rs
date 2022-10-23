use crate::auth::*;
use crate::errors::MyError;
use crate::models::*;
use actix_identity::Identity;
use actix_session::Session;
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse};
use log::info;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use webauthn_rs::prelude::{PublicKeyCredential, RegisterPublicKeyCredential};

#[get("/")]
async fn index(identity: Option<Identity>) -> Result<HttpResponse, MyError> {
    let body = match identity
        .ok_or_else(|| "Hello Anonymous!".to_string())
        .and_then(|x| x.id().map_err(|_| "Hello Anonymous!".to_string()))
        .map(|id| format!("Hello {}", id))
    {
        Ok(x) => x,
        Err(x) => x,
    };
    Ok(HttpResponse::Ok().body(body))
}

#[get("/identity")]
async fn get_identity(
    identity: Identity,
    state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    let id = identity
        .id()
        .map_err(|e| anyhow::Error::msg(format!("Failed to get identity {}", e)))?;
    let user_unique_id = Uuid::parse_str(&id)
        .map_err(|e| anyhow::Error::msg(format!("Failed to parse user unique identity {}", e)))?;
    let users_guard = state.users.lock().await;
    let user = get_user(&users_guard, user_unique_id)?;

    Ok(HttpResponse::Ok().json(user))
}

#[get("/logout")]
async fn logout(identity: Identity) -> Result<HttpResponse, MyError> {
    identity.logout();
    Ok(HttpResponse::SeeOther()
        .insert_header(("LOCATION", "https://localhost:8443/"))
        .finish())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UserRegistration {
    pub name: String,
    pub display_name: String,
}

fn is_same_user(user_unique_id: Uuid, identity: &Identity) -> bool {
    user_unique_id == Uuid::parse_str(&identity.id().unwrap()).unwrap()
}

fn is_username_available(
    user_unique_id: Option<Uuid>,
    identity: Option<Identity>,
) -> anyhow::Result<Uuid> {
    match (user_unique_id, identity) {
        // case: user_name is taken by and currently user is anonymous
        (Some(_), None) => Err(anyhow::Error::msg("Username is already taken")),
        // case: user_name is taken by another user than the currently logged in user
        (Some(user_unique_id), Some(identity)) if !is_same_user(user_unique_id, &identity) => {
            Err(anyhow::Error::msg("Username is already taken"))
        }
        // case: user_name is taken by currently logged in user
        (Some(user_unique_id), Some(_)) => Ok(user_unique_id),
        // case: user_name is unused
        (None, _) => Ok(Uuid::new_v4()),
    }
}

#[post("/register_start")]
async fn register_start(
    user_registration: web::Json<UserRegistration>,
    state: web::Data<AppState>,
    session: Session,
    identity: Option<Identity>,
) -> Result<HttpResponse, MyError> {
    info!("Start register {:?}", user_registration);

    clear_reg_state(&session);

    let users_guard = state.users.lock().await;
    let user_unique_id = name_to_id(&users_guard, &user_registration.name);
    let user_unique_id = is_username_available(user_unique_id, identity)?;
    let user = User {
        unique_id: user_unique_id,
        name: user_registration.name.to_string(),
        display_name: user_registration.display_name.to_string(),
    };
    let exclude_credentials = get_existing_credentials(&users_guard, user.unique_id);
    let (ccr, reg_state) = start_passkey_registration(&state, &user, exclude_credentials)?;
    insert_reg_state(&session, &user, &reg_state)?;

    Ok(HttpResponse::Ok().json(&ccr))
}

#[post("/register_finish")]
async fn register_finish(
    request: HttpRequest,
    reg: web::Json<RegisterPublicKeyCredential>,
    state: web::Data<AppState>,
    session: Session,
) -> Result<HttpResponse, MyError> {
    info!("Finish register");

    let (user, reg_state) = get_reg_state(&session)?;

    session
        .remove("reg_state")
        .ok_or_else(|| anyhow::Error::msg("Cannot remove reg_state"))?;

    let sk = finish_passkey_registration(&state, &reg, &reg_state)?;
    insert_user(&state, &user, sk).await;

    Identity::login(&request.extensions(), user.unique_id.to_string())
        .map_err(|e| anyhow::Error::msg(format!("Login failed {}", e)))?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/login_start")]
async fn login_start(
    username: String,
    state: web::Data<AppState>,
    session: Session,
) -> Result<HttpResponse, MyError> {
    info!("Start Authentication {}", username);
    clear_auth_state(&session);

    let users_guard = state.users.lock().await;
    let user_unique_id =
        name_to_id(&users_guard, &username).ok_or_else(|| anyhow::Error::msg("User not found"))?;

    let allow_credentials = get_allowed_credentials(&users_guard, user_unique_id)?;

    let (rcr, auth_state) = start_passkey_authentication(&state, allow_credentials)?;

    drop(users_guard);
    insert_auth_state(&session, user_unique_id, &auth_state)?;

    Ok(HttpResponse::Ok().json(rcr))
}

#[post("/login_finish")]
async fn login_finish(
    request: HttpRequest,
    auth: web::Json<PublicKeyCredential>,
    state: web::Data<AppState>,
    session: Session,
) -> Result<HttpResponse, MyError> {
    info!("Finish Authentication");

    let (user_unique_id, auth_state) = get_auth_state(&session)?;
    clear_auth_state(&session);
    let auth_result = finish_passkey_authentication(&state, &auth, &auth_state)?;
    update_credential(&state, user_unique_id, &auth_result).await?;

    let users_guard = state.users.lock().await;
    let user = get_user(&users_guard, user_unique_id)?;

    Identity::login(&request.extensions(), user_unique_id.to_string())
        .map_err(|e| anyhow::Error::msg(format!("Login failed {}", e)))?;
    Ok(HttpResponse::Ok().json(user))
}
