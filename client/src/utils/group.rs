use wasm_bindgen::JsValue;
use web_sys::console;

pub struct Group;

impl Group {
    pub fn new(name: &str) -> Self {
        console::group_collapsed_1(&JsValue::from_str(name));
        Self {}
    }
}

impl Drop for Group {
    fn drop(&mut self) {
        console::group_end();
    }
}
