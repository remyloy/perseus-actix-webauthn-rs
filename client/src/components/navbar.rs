use crate::{config::CONFIG, global_state::*};
use sycamore::prelude::*;

use super::Authorized;

#[derive(Prop)]
pub struct NavbarProps<'a> {
    pub error: &'a Signal<String>,
    pub user: &'a Signal<Option<User>>,
}

#[component]
pub fn Navbar<'a, G: Html>(cx: Scope<'a>, props: NavbarProps<'a>) -> View<G> {
    view! {cx,
        div(class="navbar bg-base-100") {
            div(class="flex-1") {
                a(class="btn btn-ghost normal-case text-xl", href="/"){ "WebAuthn" }
            }
            div(class="flex-none") {
                Authorized(user = props.user) {
                    div(class="dropdown dropdown-end") {
                        label(tabindex="0", class="btn btn-ghost btn-circle avatar") {
                            div(class="w-10 rounded-full") {
                                img(src="/.perseus/static/person.png", alt="people")
                            }
                        }
                        ul(tabindex="0", class="menu menu-compact dropdown-content mt-3 p-2 shadow bg-base-100 rounded-box w-52") {
                            li { a(href="settings") { "Settings" } }
                            li { a(href=CONFIG.logout_url) { "Logout" } }
                        }
                    }
                }
            }
        }
    }
}
