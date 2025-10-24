use serde::Serialize;

#[derive(Serialize)]
pub struct MingbuError {
    pub code: &'static str,
    pub message: String,
}

pub fn to_json<T: Serialize>(result: &T) -> Result<String, MingbuError> {
    serde_json::to_string(result)
        .map_err(|e| MingbuError {
            code: "SERDE_ERROR",
            message: e.to_string(),
        })
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn to_json_js<T: Serialize>(result: &T) -> Result<String, JsValue> {
    to_json(result).map_err(|e| JsValue::from_str(&e.to_string()))
}