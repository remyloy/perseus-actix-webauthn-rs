mod components;
mod config;
mod error_pages;
mod global_state;
mod service;
mod templates;
mod utils;

use perseus::{Html, PerseusApp, PerseusRoot};

#[cfg(not(target_arch = "wasm32"))]
pub async fn dflt_server<
    M: perseus::stores::MutableStore + 'static,
    T: perseus::i18n::TranslationsManager + 'static,
>(
    props: perseus::server::ServerProps<M, T>,
    (host, port): (String, u16),
) {
    use actix_web::{App, HttpServer};
    use futures::executor::block_on;
    use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
    use perseus_actix_web::configurer;

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("../server/certs/server.key", SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file("../server/certs/server.crt")
        .unwrap();

    HttpServer::new(move || App::new().configure(block_on(configurer(props.clone()))))
        .bind_openssl((host, port), builder)
        .expect("Couldn't bind to given address. Maybe something is already running on the selected port?")
        .run()
        .await
        .expect("Server failed.")
}

#[perseus::main(dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .global_state_creator(crate::global_state::get_global_state_creator())
        .error_pages(crate::error_pages::get_error_pages)
        .template(crate::templates::index::get_template)
        .template(crate::templates::register::get_template)
        .template(crate::templates::settings::get_template)
        .index_view(|cx| {
            sycamore::view! { cx,
                // We don't need a `<!DOCTYPE html>`, that's added automatically by Perseus (though that can be overridden if you really want by using `.index_view_str()`)
                // We need a `<head>` and a `<body>` at the absolute minimum for Perseus to work properly (otherwise certain script injections will fail)
                html(data-theme="light") {
                head {
                    title { "WebAuthn + Perseus + Actix Example" }
                    link(rel="stylesheet", href=".perseus/static/app.css")
                }
                body {
                    // This creates an element into which our app will be interpolated
                    // This uses a few tricks internally beyond the classic `<div id="root">`, so we use this wrapper for convenience
                    PerseusRoot()
                    // Because this is in the index view, this will be below every single one of our pages
                    // Note that elements in here can't be selectively removed from one page, it's all-or-nothing in the index view (it wraps your whole app)
                    // Note also that this won't be reloaded, even when the user switches pages
                    footer(class="footer items-center p-4 bg-neutral text-neutral-content") {
                        div(class="items-center grid-flow-col") {
                            svg(width="36", height="36", viewBox="0 0 24 24", xmlns="http://www.w3.org/2000/svg", fill-rule="evenodd", clip-rule="evenodd", class="fill-current"){
                                path(d="M22.672 15.226l-2.432.811.841 2.515c.33 1.019-.209 2.127-1.23 2.456-1.15.325-2.148-.321-2.463-1.226l-.84-2.518-5.013 1.677.84 2.517c.391 1.203-.434 2.542-1.831 2.542-.88 0-1.601-.564-1.86-1.314l-.842-2.516-2.431.809c-1.135.328-2.145-.317-2.463-1.229-.329-1.018.211-2.127 1.231-2.456l2.432-.809-1.621-4.823-2.432.808c-1.355.384-2.558-.59-2.558-1.839 0-.817.509-1.582 1.327-1.846l2.433-.809-.842-2.515c-.33-1.02.211-2.129 1.232-2.458 1.02-.329 2.13.209 2.461 1.229l.842 2.515 5.011-1.677-.839-2.517c-.403-1.238.484-2.553 1.843-2.553.819 0 1.585.509 1.85 1.326l.841 2.517 2.431-.81c1.02-.33 2.131.211 2.461 1.229.332 1.018-.21 2.126-1.23 2.456l-2.433.809 1.622 4.823 2.433-.809c1.242-.401 2.557.484 2.557 1.838 0 .819-.51 1.583-1.328 1.847m-8.992-6.428l-5.01 1.675 1.619 4.828 5.011-1.674-1.62-4.829z")
                            }

                            p {"Copyright \u{00a9} 2022 - All right reserved"}
                        }
                        div(class="grid-flow-col gap-4 md:place-self-center md:justify-self-end") {

                        }
                     }
                }}
            }
        })
}
