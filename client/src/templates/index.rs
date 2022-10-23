use crate::components::{Navbar, Register};
use crate::global_state::*;
use perseus::Template;
use sycamore::prelude::{view, Html, Scope, SsrNode, View};
use sycamore::reactive::{create_selector, Signal};
use sycamore::{component, Prop};

#[derive(Prop)]
pub struct HeroProps<'a> {
    pub reg_state: &'a Signal<AuthState>,
    pub login_state: &'a Signal<AuthState>,
    pub error: &'a Signal<String>,
    pub user: &'a Signal<Option<User>>,
}

#[component]
pub fn Hero<'a, G: Html>(cx: Scope<'a>, props: HeroProps<'a>) -> View<G> {
    view! {cx,
        div(class="hero min-h-[60vh] bg-base-200"){
            div(class="hero-content flex-col lg:flex-row-reverse"){
                (if *create_selector(cx, || props.user.get().is_none()).get() {
                    view! { cx,
                        div(class="text-center lg:text-left"){
                            h1(class="text-5xl font-bold"){ "WebAuthn + perseus + actix"}
                            p(class="py-6") { "Try it out by first registering with a username that you choose. Then complete the registration and try to login." }
                        }
                        Register(reg_state = props.reg_state, login_state = props.login_state, error = props.error, user = props.user)
                    }
                } else {
                    view! { cx,
                        img(src="/.perseus/static/landscape.jpg", class="max-w-sm rounded-lg shadow-2xl")
                        div {
                            h1(class="text-5xl font-bold"){ "Well done!" }
                            p(class="py-6") {
                                "Your next steps could be to either logout and login again.
                                Or try to create another account using another username.
                                Finally you could try to register another authenticator to your account, maybe another device or browser."
                             }
                        }
                    }
                })
            }
        }
    }
}

#[perseus::template_rx]
pub fn index_page<'a, G: Html>(cx: Scope<'a>, _: (), app_state: AppStateRx<'a>) -> View<G> {
    #[cfg(target_arch = "wasm32")]
    if G::IS_BROWSER {
        use crate::log;

        AppStateRx::load_identity_state(&app_state, cx);
        let group = crate::utils::group::Group::new("AppState");
        log!("reg_state: {:?}", app_state.reg_state.get());
        log!("login_state: {:?}", app_state.login_state.get());
        log!("error: {:?}", app_state.error.get());
        log!("user: {:?}", app_state.user.get());
        drop(group);
    }

    view! { cx,
        Navbar(user = app_state.user, error = app_state.error)
        Hero(reg_state = app_state.reg_state, login_state = app_state.login_state, user = app_state.user, error = app_state.error)
    }
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "WebAuthn - Welcome" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index").template(index_page).head(head)
}
