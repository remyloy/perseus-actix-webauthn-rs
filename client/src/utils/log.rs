#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}
