#[cfg(target_arch = "wasm32")]
#[path = ""]
mod browser {
    pub mod register_browser;
    pub use register_browser::*;
}

#[cfg(target_arch = "wasm32")]
pub use browser::*;

#[cfg(not(target_arch = "wasm32"))]
#[path = ""]
mod server {
    pub mod register_server;
    pub use register_server::*;
}

#[cfg(not(target_arch = "wasm32"))]
pub use server::*;

pub mod authorized;
pub use authorized::*;
pub mod navbar;
pub use navbar::*;