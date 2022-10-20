// web_sys_mod.rs
//! helper functions for web_sys, window, document, dom, console,
//! local_storage, session_storage,...

use serde::de::DeserializeOwned;
use serde::Serialize;
// region: use
use unwrap::unwrap;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::console;
use web_sys::{Request, RequestInit, Response};
// endregion: use

/// Simple macro to set listener of onclick events to an element_id to an async function.
/// no args: on_click!(element_1_id, function_ident)
/// no args: on_click!("example_email",example_email)
#[macro_export]
macro_rules! on_click {
    ($element_1_id: expr, $function_ident: ident) => {{
        let closure = Closure::wrap(Box::new(move || {
            // event listeners use sync functions,
            // but most of other functions are async because of javascript
            wasm_bindgen_futures::spawn_local(async move {
                $function_ident().await;
            });
        }) as Box<dyn FnMut()>);

        let html_element = get_html_element_by_id($element_1_id);
        html_element.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }};
}

/// return window object
pub fn window() -> web_sys::Window {
    unwrap!(web_sys::window())
}

/// debug write into session_storage
pub fn debug_write(text: &str) {
    // writing to the console is futile for mobile phones
    // I must write it on the UI.
    // so I must access this string from the UI renderer
    // add_to_begin_of_debug_text(text);
    console::log_1(&JsValue::from_str(text));
}

/// get element by id
pub fn get_element_by_id(element_id: &str) -> web_sys::Element {
    let document = unwrap!(window().document());
    match document.get_element_by_id(element_id) {
        Some(el) => el,
        None => {
            debug_write(&format!(
                "Error: not found get_element_by_id {}",
                element_id
            ));
            panic!("Error: not found get_element_by_id")
        }
    }
}

/// get html element by id
pub fn get_html_element_by_id(element_id: &str) -> web_sys::HtmlElement {
    let element = get_element_by_id(element_id);
    let html_element: web_sys::HtmlElement = unwrap!(element.dyn_into::<web_sys::HtmlElement>());
    //return
    html_element
}

/// get input element value string by id
pub fn get_input_element_value_string_by_id(element_id: &str) -> String {
    // debug_write("before get_element_by_id");
    let input_element = get_element_by_id(element_id);
    // debug_write("before dyn_into");
    let input_html_element = unwrap!(input_element.dyn_into::<web_sys::HtmlInputElement>());
    // debug_write("before value()");
    input_html_element.value()
}

/// fetch in Rust with async await for executor spawn_local()
/// return the response as json. Any error will panic.
pub async fn fetch_json_response(url: String, json_jsvalue_body: String) -> String {
    let headers = web_sys::Headers::new().unwrap();
    headers.set("Accept", "application/json").unwrap();
    headers.set("Content-Type", "application/json").unwrap();
    // Request init
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.headers(&headers);
    // headers: new Headers({'content-type': 'application/json'}),
    let json_jsvalue_body = JsValue::from_str(&json_jsvalue_body);
    opts.body(Some(&json_jsvalue_body));
    let request = unwrap!(Request::new_with_str_and_init(&url, &opts));
    // log1("before fetch");
    let resp_jsvalue = unwrap!(JsFuture::from(window().fetch_with_request(&request)).await);
    // log1("after fetch");
    let resp: Response = unwrap!(resp_jsvalue.dyn_into());
    // log1("before text()");
    let text_jsvalue = unwrap!(JsFuture::from(unwrap!(resp.text())).await);
    let txt_response: String = unwrap!(text_jsvalue.as_string());
    txt_response
}

/// fetch in Rust with async await for executor spawn_local()
/// parameter is a Serializable object
/// return the response as deserialized object from json. Any error will panic.
pub async fn fetch_object_response<T>(url: String, obj: impl Serialize) -> T
where
    T: DeserializeOwned,
{
    let json_jsvalue_body = serde_json::to_string(&obj).unwrap();
    let json_jsvalue = fetch_json_response(url, json_jsvalue_body).await;
    let resp_data: T = serde_json::from_str(&json_jsvalue).unwrap();
    resp_data
}

/// set element inner text string by id
pub fn set_element_inner_html_by_id(
    element_id: &str,
    html: &crate::web_sys_html_encode_mod::HtmlEncoded,
) {
    //debug_write("before get_element_by_id");
    let element = get_element_by_id(element_id);
    //debug_write("before value()");
    let html = html.get_html();
    element.set_inner_html(&html);
}
