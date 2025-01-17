use wasm_bindgen::JsValue;
use web_sys::{window, Storage};

pub struct LocalStorage {
  storage: Storage,
}

impl LocalStorage {
  pub fn new() -> Result<Self, JsValue> {
    let window = window()
      .ok_or(JsValue::from_str("Window not found"))?;
    let storage = window
      .local_storage()
      .map_err(|err| JsValue::from_str(&format!("Failed to get local storage: {:?}", err)))?
      .ok_or(JsValue::from_str("Storage not found"))?;
    Ok(Self { storage })
  }

  pub fn set_item(&self, key: &str, value: &str) -> Result<(), JsValue> {
    self.storage.set_item(key, value).map_err(|err| JsValue::from_str(&format!("Failed to set item: {:?}", err)))
  }

  pub fn get_item(&self, key: &str) -> Result<Option<String>, JsValue> {
    self.storage
      .get_item(key)
      .map_err(|err| JsValue::from_str(&format!("Failed to get item: {:?}", err)))
  }
}
