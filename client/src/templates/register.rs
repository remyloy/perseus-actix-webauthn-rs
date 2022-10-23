use crate::{
    components::{Authorized, Navbar},
    global_state::*,
};
use perseus::prelude::*;
use sycamore::prelude::*;

#[make_rx(RegisterPageStateRx)]
pub struct RegisterPageState {
    pub user_name: String,
    pub display_name: String,
}

#[perseus::template_rx]
pub fn register_page<'a, G: Html>(
    cx: Scope<'a>,
    page_state: RegisterPageStateRx,
    app_state: AppStateRx<'a>,
) -> View<G> {
    #[cfg(target_arch = "wasm32")]
    AppStateRx::load_identity_state(&app_state, cx);

    let user_name_entered = create_signal_from_rc(cx, page_state.user_name.get());
    let display_name_entered = create_signal_from_rc(cx, page_state.display_name.get());

    let on_register = move |_| {
        #[cfg(target_arch = "wasm32")]
        perseus::spawn_local_scoped(cx, async move {
            let res = crate::service::actions::register(
                &crate::config::CONFIG,
                user_name_entered.get().to_string(),
                display_name_entered.get().to_string(),
            )
            .await;
            match res {
                Ok(_) => {
                    app_state.reg_state.set(AuthState::Yes);
                    app_state.error.set("".to_string());
                }
                Err(err) => {
                    app_state.reg_state.set(AuthState::No);
                    app_state.error.set(err.to_string());
                }
            }
            let user = AppStateRx::get_identity_state().await;
            AppStateRx::update_identity_state(&app_state.user, &app_state.error, &user);
            if user.is_ok() {
                web_sys::window().unwrap().location().set_href("/").unwrap();
            }
        });
    };

    let unauthorized = view! { cx,
        div (class="hero bg-base-200") {
            div (class="hero-content flex-col") {
                div (class="card flex-shrink-0 w-full max-w-sm shadow-2xl bg-base-100") {
                    div (class="card-body") {
                        div (class="form-control") {
                            label (class="label"){
                            span (class="label-text") {"Email"}
                            }
                            input (type="text", placeholder="email", class="input input-bordered", bind:value=user_name_entered)
                        }
                        div (class="form-control") {
                            label (class="label"){
                            span (class="label-text") {"Display name"}
                            }
                            input (type="text", placeholder="display name", class="input input-bordered", bind:value=display_name_entered)
                        }
                        div (class="form-control mt-6") {
                            button (class="btn btn-primary", on:click=on_register) {
                                "Register"
                            }
                            (if *app_state.error.get() != "" {
                                view!{cx,
                                    div (class="alert alert-error shadow-lg mt-6") {
                                        div {
                                            svg(xmlns="http://www.w3.org/2000/svg", class="stroke-current flex-shrink-0 h-6 w-6", fill="none", viewBox="0 0 24 24") {
                                                path(stroke-linecap="round", stroke-linejoin="round", stroke-width="2", d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z")
                                            }
                                            span { (*app_state.error.get()) }
                                        }
                                    }
                                }
                            } else {
                                view!{ cx,}
                            })
                        }
                        a (class="link", href="/") { "Go back" }
                    }
                }
            }
        }
    };
    view! { cx,
        Navbar(user = app_state.user, error = app_state.error)
        Authorized(user = app_state.user, unauthorized = Some(unauthorized)) {
            div {
                "
                Looks like you are already logged in.
                So no there's no need to register again for you.
                If you really want to register again with another username logout first.
                If you wanted to register another authenticator check out to the
                "
                a(href="settings") { "Settings" }
                "."
            }
        }
    }
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "WebAuthn - Register" }
    }
}

#[perseus::build_state]
pub fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<RegisterPageState> {
    Ok(RegisterPageState {
        user_name: "".to_string(),
        display_name: "".to_string(),
    })
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("register")
        .template(register_page)
        .head(head)
        .build_state_fn(get_build_state)
}
