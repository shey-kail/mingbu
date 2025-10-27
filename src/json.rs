use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct MingbuError {
    pub code: &'static str,
    pub message: String,
}

impl fmt::Display for MingbuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MingbuError {}: {}", self.code, self.message)
    }
}

impl std::error::Error for MingbuError {}

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