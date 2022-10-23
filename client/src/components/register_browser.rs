use perseus::spawn_local_scoped;
use sycamore::prelude::{component, create_signal, view, Html, Prop, Scope, Signal, View};

use crate::{config::CONFIG, global_state::*};

#[derive(Prop)]
pub struct RegisterProps<'a> {
    pub reg_state: &'a Signal<AuthState>,
    pub login_state: &'a Signal<AuthState>,
    pub error: &'a Signal<String>,
    pub user: &'a Signal<Option<User>>,
}

#[component]
pub fn Register<'a, G: Html>(cx: Scope<'a>, props: RegisterProps<'a>) -> View<G> {
    let username_entered = create_signal(cx, "".to_string());

    let on_login = move |_| {
        spawn_local_scoped(cx, async move {
            let res =
                crate::service::actions::authenticate(&CONFIG, username_entered.get().to_string())
                    .await;
            match res {
                Ok(_) => {
                    props.login_state.set(AuthState::Yes);
                    props.error.set("".to_string());
                }
                Err(err) => {
                    props.login_state.set(AuthState::No);
                    props.error.set(err.to_string());
                }
            }
            let user = AppStateRx::get_identity_state().await;
            AppStateRx::update_identity_state(&props.user, &props.error, &user);
        });
    };

    view! {cx,
        div (class="card flex-shrink-0 w-full max-w-sm shadow-2xl bg-base-100") {
            div (class="card-body") {
                div (class="form-control") {
                    label (class="label"){
                    span (class="label-text") {"Email"}
                    }
                    input (type="text", placeholder="email", class="input input-bordered", bind:value=username_entered)
                }
                div (class="form-control mt-6") {
                    button (class="btn btn-primary", on:click=on_login) {
                        "Login"
                    }
                }

                a (class="link", href="register") { "Register" }
            }
        }
    }
}
