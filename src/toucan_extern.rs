use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use worker::js_sys;

#[wasm_bindgen(module = "/src/js/toucan.js")]
extern "C" {
    pub(crate) type Toucan;

    #[wasm_bindgen(constructor, js_class = "Toucan")]
    pub(crate) fn new(args: js_sys::Object) -> Toucan;

    #[wasm_bindgen(method, js_class = "Toucan", js_name = "setTag")]
    pub(crate) fn set_tag(this: &Toucan, key: &str, value: &str);

    #[wasm_bindgen(method, js_class = "Toucan", js_name = "setContext")]
    pub(crate) fn set_context(this: &Toucan, name: &str, obj: js_sys::Object);

    #[wasm_bindgen(method, js_class = "Toucan", js_name = "captureMessage")]
    pub(crate) fn capture_message(this: &Toucan, msg: &str) -> JsValue;

    #[wasm_bindgen(method, js_class = "Toucan", js_name = "captureException")]
    pub(crate) fn capture_exception(this: &Toucan, err: js_sys::Error) -> JsValue;

    #[wasm_bindgen(method, js_class = "Toucan", js_name = "setPropagationContext")]
    pub(crate) fn set_propagation_context(this: &Toucan, pctx: js_sys::Object) -> JsValue;

    #[wasm_bindgen(method, js_class = "Toucan", js_name = "setRequestBody")]
    pub(crate) fn set_request_body(this: &Toucan, body: &str);

    #[wasm_bindgen(method, js_class = "Toucan", js_name = "setEnabled")]
    pub(crate) fn set_enabled(this: &Toucan, enabled: bool);

    #[wasm_bindgen(method, js_class = "Toucan", js_name = "addBreadcrumb")]
    pub(crate) fn add_breadcrumb(this: &Toucan, breadcrumb: js_sys::Object, maxBreadcrumbs: u32);
}

// Implement Debug for Toucan
impl std::fmt::Debug for Toucan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Toucan Instance")
    }
}
