use anyhow::Ok;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    console, window, CredentialCreationOptions, Request, RequestInit, RequestMode, Response,
};
use webauthn_rs_proto::{
    CreationChallengeResponse, PublicKeyCredential, RegisterPublicKeyCredential,
    RequestChallengeResponse,
};

use crate::{config::Config, utils::group::Group};

pub async fn get(url: &str) -> anyhow::Result<Response> {
    console::log_1(&JsValue::from_str(&format!("get {}", url)));
    let window = window().ok_or_else(|| anyhow::anyhow!("Failed to obtain window"))?;

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    opts.credentials(web_sys::RequestCredentials::Include);
    let request = Request::new_with_str_and_init(url, &opts)
        .map_err(|e| anyhow::anyhow!("Failed to create fetch request {:?}", e))?;
    request
        .headers()
        .set("content-type", "application/json")
        .map_err(|e| anyhow::anyhow!("Failed to set header {:?}", e))?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| anyhow::anyhow!("Failed to fetch {:?}", e))?;
    let resp: Response = resp_value
        .clone()
        .dyn_into()
        .map_err(|e| anyhow::anyhow!("Failed to cast JSON into fetch response {:?}", e))?;
    if resp.ok() {
        console::log_2(&JsValue::from_str("get"), &JsValue::from(resp.status()));
        Ok(resp)
    } else {
        let prom = resp
            .text()
            .map_err(|e| anyhow::anyhow!("Failed to get text from fetch response {:?}", e))?;
        let text = JsFuture::from(prom)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get text from fetch response {:?}", e))?
            .as_string()
            .map(|e| anyhow::anyhow!("GET to {} failed with {:?}", url, e))
            .unwrap_or_else(|| anyhow::anyhow!("Failed to get text from fetch response"));
        Err(text)
    }
}

pub async fn get_json(url: &str) -> anyhow::Result<JsValue> {
    let resp = get(url).await?;
    let prom = resp
        .json()
        .map_err(|e| anyhow::anyhow!("Failed to get JSON from fetch response {:?}", e))?;
    let jsval = JsFuture::from(prom)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to get JSON from fetch response {:?}", e))?;
    Ok(jsval)
}

async fn post(url: &str, body: Option<&JsValue>) -> anyhow::Result<Response> {
    console::log_1(&JsValue::from_str(&format!("post {} {:?}", url, body)));
    let window = window().ok_or_else(|| anyhow::anyhow!("Failed to obtain window"))?;

    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.body(body);
    opts.credentials(web_sys::RequestCredentials::Include);
    let request = Request::new_with_str_and_init(url, &opts)
        .map_err(|e| anyhow::anyhow!("Failed to create fetch request {:?}", e))?;
    request
        .headers()
        .set("content-type", "application/json")
        .map_err(|e| anyhow::anyhow!("Failed to set header {:?}", e))?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| anyhow::anyhow!("Failed to fetch {:?}", e))?;
    let resp: Response = resp_value
        .clone()
        .dyn_into()
        .map_err(|e| anyhow::anyhow!("Failed to cast JSON into fetch response {:?}", e))?;
    if resp.ok() {
        console::log_2(&JsValue::from_str("post"), &JsValue::from(resp.status()));
        Ok(resp)
    } else {
        let prom = resp
            .text()
            .map_err(|e| anyhow::anyhow!("Failed to get text from fetch response {:?}", e))?;
        let text = JsFuture::from(prom)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get text from fetch response {:?}", e))?
            .as_string()
            .map(|e| anyhow::anyhow!("POST to {} failed with {:?}", url, e))
            .unwrap_or_else(|| anyhow::anyhow!("Failed to get text from fetch response"));
        Err(text)
    }
}

async fn post_json(url: &str, body: Option<&JsValue>) -> anyhow::Result<JsValue> {
    let resp = post(url, body).await?;
    console::log_2(&JsValue::from_str("post"), &JsValue::from(resp.status()));
    let prom = resp
        .json()
        .map_err(|e| anyhow::anyhow!("Failed to get JSON from fetch response {:?}", e))?;
    let jsval = JsFuture::from(prom)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to get JSON from fetch response {:?}", e))?;
    console::log_2(&JsValue::from_str("post"), &jsval);
    Ok(jsval)
}

pub async fn register<'a>(
    config: &Config<'a>,
    username: String,
    display_name: String,
) -> anyhow::Result<()> {
    let grp = Group::new(&format!("register {}", username));
    let ccr = register_start(config, username, display_name).await?;
    let rpkc = update_register_challenge(ccr).await?;
    register_complete(config, rpkc).await?;
    drop(grp);
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UserRegistration {
    pub name: String,
    pub display_name: String,
}

async fn register_start<'a>(
    config: &Config<'a>,
    username: String,
    display_name: String,
) -> anyhow::Result<CreationChallengeResponse> {
    let body = serde_json::to_string(&UserRegistration {
        name: username,
        display_name: display_name,
    })
    .map(|s| JsValue::from(s))
    .map_err(|e| anyhow::Error::msg(format!("Failed to serialize request body {}", e)))?;
    let jsval = post_json(config.register_start, Some(&body)).await?;
    let ccr: CreationChallengeResponse = serde_wasm_bindgen::from_value(jsval).map_err(|e| {
        anyhow::anyhow!(
            "Failed to deserialize JSON into CreationChallengeResponse {:?}",
            e
        )
    })?;
    Ok(ccr)
}

async fn update_register_challenge(
    ccr: CreationChallengeResponse,
) -> anyhow::Result<RegisterPublicKeyCredential> {
    console::log_1(&JsValue::from_str("update_register_challenge"));
    let window = window().ok_or_else(|| anyhow::anyhow!("Failed to obtain window"))?;
    let c_options: CredentialCreationOptions = ccr.clone().into();
    console::log_1(&c_options);
    let promise = window
        .navigator()
        .credentials()
        .create_with_options(&c_options)
        .map_err(|e| {
            anyhow::anyhow!(
                "Failed to create credentials with options {:?} {:?}",
                &c_options,
                e
            )
        })?;
    let jsval = JsFuture::from(promise).await.map_err(|e| {
        anyhow::Error::msg(format!(
            "Unable to complete credential creation with options {:?}",
            e
        ))
    })?;
    let w_rpkc = web_sys::PublicKeyCredential::from(jsval);
    let rpkc = RegisterPublicKeyCredential::from(w_rpkc);
    Ok(rpkc)
}

async fn register_complete<'a>(
    config: &Config<'a>,
    rpkc: RegisterPublicKeyCredential,
) -> anyhow::Result<()> {
    console::log_2(
        &JsValue::from_str("register_complete"),
        &JsValue::from_str(&format!("{:?}", rpkc)),
    );
    let req_jsvalue = serde_json::to_string(&rpkc)
        .map(|s| JsValue::from(s))
        .map_err(|e| anyhow::anyhow!("Failed to get text from fetch response {:?}", e))?;
    let public_key_credential = post(config.register_finish, Some(&req_jsvalue)).await?;
    console::log_2(
        &JsValue::from_str("register_complete"),
        &public_key_credential,
    );
    Ok(())
}

pub async fn authenticate<'a>(config: &Config<'a>, username: String) -> anyhow::Result<()> {
    let grp = Group::new(&format!("authenticate {}", username));
    let rcr = authenticate_begin(config, username).await?;
    let pkc = update_authenticate_challenge(rcr).await?;
    authenticate_complete(config, pkc).await?;
    drop(grp);
    Ok(())
}

async fn authenticate_begin<'a>(
    config: &Config<'a>,
    username: String,
) -> anyhow::Result<RequestChallengeResponse> {
    console::log_2(
        &JsValue::from_str("authenticate_begin"),
        &JsValue::from_str(&format!("username {:?}", username)),
    );
    let jsval = post_json(config.login_start, Some(&JsValue::from_str(&username))).await?;
    let rcr: RequestChallengeResponse = serde_wasm_bindgen::from_value(jsval).map_err(|e| {
        anyhow::anyhow!(
            "Failed to deserialize JSON into RequestChallengeResponse {:?}",
            e
        )
    })?;
    Ok(rcr)
}

async fn update_authenticate_challenge(
    rcr: RequestChallengeResponse,
) -> anyhow::Result<PublicKeyCredential> {
    console::log_2(
        &JsValue::from_str("update_authenticate_challenge"),
        &JsValue::from_str(&format!("rcr {:?}", rcr)),
    );
    let window = window().ok_or_else(|| anyhow::anyhow!("Failed to obtain window"))?;

    let c_options: web_sys::CredentialRequestOptions = rcr.into();
    let promise = window
        .navigator()
        .credentials()
        .get_with_options(&c_options)
        .map_err(|e| {
            anyhow::anyhow!(
                "Failed to get credentials from container {:?} {:?}",
                c_options,
                e
            )
        })?;
    let jsval = JsFuture::from(promise).await.map_err(|e| {
        anyhow::anyhow!(
            "Failed to get credentials from container {:?} {:?}",
            c_options,
            e
        )
    })?;
    // Wait on the promise, when complete it will issue a callback.
    let w_rpkc = web_sys::PublicKeyCredential::from(jsval);
    // Serialise the web_sys::pkc into the webauthn proto version, ready to
    // handle/transmit.
    let pkc = PublicKeyCredential::from(w_rpkc);
    Ok(pkc)
}

async fn authenticate_complete<'a>(
    config: &Config<'a>,
    pkc: PublicKeyCredential,
) -> anyhow::Result<()> {
    console::log_2(
        &JsValue::from_str("authenticate_complete"),
        &JsValue::from_str(&format!("pkc {:?}", pkc)),
    );

    let req_jsvalue = serde_json::to_string(&pkc)
        .map(|s| JsValue::from(&s))
        .map_err(|e| anyhow::anyhow!("Failed to serialize {:?} {:?}", pkc, e))?;

    let _resp = post(config.login_finish, Some(&req_jsvalue)).await?;
    console::log_2(
        &JsValue::from_str("authenticate_complete"),
        &JsValue::from_str("AuthenticateSuccess"),
    );
    Ok(())
}
