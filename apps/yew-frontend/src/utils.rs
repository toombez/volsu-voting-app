use wasm_bindgen::JsCast;

pub fn store_token(token: &str) {
    let token_cookie = wasm_cookies::cookies::set(
        "token",
        &token,
        &wasm_cookies::CookieOptions::default()
    );

    let document = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .dyn_into::<web_sys::HtmlDocument>()
        .unwrap();

    let _ = document.set_cookie(&token_cookie);
}

pub fn clear_token() {
    let cookie = wasm_cookies::cookies::delete("token");

    let document = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .dyn_into::<web_sys::HtmlDocument>()
        .unwrap();

    let _ = document.set_cookie(&cookie);
}

pub fn get_token() -> Option<String> {
    let document = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .dyn_into::<web_sys::HtmlDocument>()
        .unwrap();

    let cookie = document.cookie().unwrap();

    wasm_cookies
        ::cookies
        ::get(&cookie, "token")
        .map(|token| token.unwrap().to_string())
}
