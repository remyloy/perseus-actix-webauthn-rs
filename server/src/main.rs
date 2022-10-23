use crate::actions::*;
use crate::models::*;
use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::config::CookieContentSecurity;
use actix_session::SessionMiddleware;
use actix_web::cookie::{Key, SameSite};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use async_std::sync::Mutex;
use config::Config;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use session_store::MemorySessionStore;
use std::collections::HashMap;
use std::sync::Arc;
use webauthn_rs::prelude::Url;
use webauthn_rs::WebauthnBuilder;
mod actions;
mod auth;
mod config;
mod errors;
mod models;
mod session_store;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    builder.set_private_key_file("certs/server.key", SslFiletype::PEM)?;
    builder.set_certificate_chain_file("certs/server.crt")?;

    let config = Arc::new(Config {
        endpoint: "localhost:443".to_string(),
        rp_id: "localhost".to_string(),
        rp_origin: "https://localhost:8443".to_string(),
        redirect_logout: "".to_string(),
    });

    let webauthn = Arc::new(
        {
            let rp_origin = Url::parse(&config.rp_origin).expect("Invalid URL");
            let builder =
                WebauthnBuilder::new(&config.rp_id, &rp_origin).expect("Invalid configuration");
            builder.build()
        }
        .expect("Invalid configuration"),
    );

    let session_key = {
        let secret: &Vec<u8> = &(0..64).collect();
        Key::from(secret)
    };

    let config2 = config.clone();
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&config.rp_origin)
            .allow_any_header()
            .allow_any_method()
            .supports_credentials();
        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(MemorySessionStore::default(), session_key.clone())
                    .cookie_content_security(CookieContentSecurity::Private)
                    .cookie_http_only(true)
                    .cookie_secure(true)
                    .cookie_same_site(SameSite::None)
                    .build(),
            )
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(AppState {
                config: config.clone(),
                webauthn: webauthn.clone(),
                users: Mutex::new(Users {
                    name_to_id: HashMap::new(),
                    keys: HashMap::new(),
                }),
            }))
            .service(index)
            .service(get_identity)
            .service(logout)
            .service(register_start)
            .service(register_finish)
            .service(login_start)
            .service(login_finish)
    })
    .bind_openssl(&config2.endpoint, builder)?
    .run()
    .await
}
