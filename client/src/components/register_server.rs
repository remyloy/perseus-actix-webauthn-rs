use crate::global_state::*;
use sycamore::prelude::*;

#[derive(Prop)]
pub struct RegisterProps<'a> {
    pub reg_state: &'a Signal<AuthState>,
    pub login_state: &'a Signal<AuthState>,
    pub error: &'a Signal<String>,
    pub user: &'a Signal<Option<User>>,
}

#[component]
pub fn Register<'a, G: Html>(cx: Scope<'a>, _props: RegisterProps<'a>) -> View<G> {
    println!("register_server");
    view! {cx, "server"}
}
