use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    
    #[wasm_bindgen(js_namespace = window)]
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let var_name = &format!("Hello again, whatever, {}!", name);
    log(var_name);
}
 