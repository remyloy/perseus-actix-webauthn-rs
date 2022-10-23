use crate::{
    components::{Authorized, Navbar},
    global_state::*,
};
use perseus::prelude::*;
use sycamore::prelude::*;

#[perseus::template_rx]
pub fn settings_page<'a, G: Html>(cx: Scope<'a>, _: (), app_state: AppStateRx<'a>) -> View<G> {
    #[cfg(target_arch = "wasm32")]
    AppStateRx::load_identity_state(&app_state, cx);
    let unauthorized = view! { cx,
        a(class="link", href="/") { "Go back "}
    };
    view! { cx,
        Navbar(user = app_state.user, error = app_state.error)
        Authorized(user = app_state.user, unauthorized = Some(unauthorized)) {
            div (class="hero min-h-[60vh] bg-base-200") {
                div (class="hero-content flex-col") {
                    "todo"
                }
            }
        }
    }
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "WebAuthn - Settings" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("settings").template(settings_page).head(head)
}
