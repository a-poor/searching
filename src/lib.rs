mod utils;
mod tokenize;
mod transform;
mod stem;
mod filter;
mod pipeline;
mod storage;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use serde_json::Value;


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, searching!");
}

#[wasm_bindgen]
pub fn parse(text: &str) -> Vec<String> {
    let pipeline = pipeline::Pipeline::new(
        transform::NoOpTransformer,
        tokenize::WhitespaceTokenizer,
        filter::StopWordsFilter,
        transform::NoOpTransformer,
    );
    pipeline.process(text)
}

pub type DefaultPipeline = pipeline::Pipeline<transform::NoOpTransformer, tokenize::WhitespaceTokenizer, filter::StopWordsFilter, transform::NoOpTransformer>;

#[wasm_bindgen]
pub struct SearchEngine {
  storage: storage::LocalStorage,
  pipeline: DefaultPipeline,
}

impl SearchEngine {
  pub fn new() -> Result<Self, JsValue> {
    let storage = storage::LocalStorage::new()?;
    let pipeline = pipeline::Pipeline::new(
      transform::NoOpTransformer,
      tokenize::WhitespaceTokenizer,
      filter::StopWordsFilter,
      transform::NoOpTransformer,
    );
    Ok(Self { storage, pipeline })
  }

  pub fn parse(&self, text: &str) -> Vec<String> {
    self.pipeline.process(text)
  }

  fn fmt_word_key(word: &str) -> String {
    format!("search:word:{}", word)
  }

  fn fmt_doc_key(id: &str) -> String {
    format!("search:doc:{}", id)
  }

  pub fn add_document(&self, id: &str, document: &str, metadata: &str) -> Result<(), JsValue> {
    let tokens = self.parse(document);
    for token in tokens {
      let word_key = Self::fmt_word_key(&token);
      self.storage.set_item(&word_key, id)?;
    }
    let doc_key = Self::fmt_doc_key(id);
    self.storage.set_item(&doc_key, metadata)?;
    Ok(())
  }
}
