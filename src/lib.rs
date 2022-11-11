use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn fibonacci(n: i32) -> i32 {
	if n < 2 { return n; }
	return fibonacci(n-1) + fibonacci(n-2);
}
