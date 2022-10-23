use crate::global_state::User;
use sycamore::prelude::*;

#[derive(Prop)]
pub struct AuthorizedProps<'a, G: Html> {
    children: Children<'a, G>,
    #[builder(default)]
    unauthorized: Option<View<G>>,
    pub user: &'a ReadSignal<Option<User>>,
}

#[component]
pub fn Authorized<'a, G: Html>(cx: Scope<'a>, props: AuthorizedProps<'a, G>) -> View<G> {
    let is_authorized = create_selector(cx, || props.user.get().is_some());
    let children = props.children.call(cx);
    let unauthorized = props.unauthorized;
    View::new_dyn(cx, move || {
        if *is_authorized.get() {
            children.clone()
        } else {
            unauthorized.clone().unwrap_or_else(|| View::empty())
        }
    })
}
