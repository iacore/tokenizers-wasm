use js_sys;
use std::str::FromStr;
use tokenizers::tokenizer::{Encoding, Tokenizer};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct TokenizerWasm {
    tokenizer: Tokenizer,
}

#[wasm_bindgen]
impl TokenizerWasm {
    #[wasm_bindgen(constructor)]
    pub fn from_buffer(json: String) -> Result<TokenizerWasm, JsValue> {
        Ok(TokenizerWasm {
            tokenizer: Tokenizer::from_str(json.as_str()).map_err(|e| format!("{e}"))?,
        })
    }

    pub fn encode(&self, text: &str, add_special_tokens: bool) -> Result<EncodingWasm, JsValue> {
        Ok(EncodingWasm {
            encoding: self
                .tokenizer
                .encode(text, add_special_tokens)
                .map_err(|e| format!("{e}"))?,
        })
    }

    pub fn decode(&self, tokens: &JsValue, skip_special_tokens: bool) -> Result<String, JsValue> {
        let tokens = js_sys::try_iter(tokens)?.ok_or_else(|| "`tokens` is not a iterable")?;

        let mut ids = vec![];

        for tok in tokens.into_iter().map(|x| -> Result<_, JsValue> {
            let x_f64 = x?.as_f64().ok_or_else(|| "iter: as f64")?;
            Ok(x_f64 as u32)
        }) {
            ids.push(tok?);
        }
        Ok(self
            .tokenizer
            .decode(ids, skip_special_tokens)
            .map_err(|e| format!("{e}"))?)
    }
}

#[wasm_bindgen]
pub struct EncodingWasm {
    encoding: Encoding,
}

#[wasm_bindgen]
impl EncodingWasm {
    #[wasm_bindgen(method, getter = input_ids)]
    pub fn get_ids(&self) -> js_sys::Uint32Array {
        self.encoding.get_ids().into()
    }

    #[wasm_bindgen(method, getter = tokens)]
    pub fn get_tokens(&self) -> js_sys::Array {
        self.encoding
            .get_tokens()
            .iter()
            .map(|x| js_sys::JsString::from(x.as_str()))
            .collect()
    }
}
