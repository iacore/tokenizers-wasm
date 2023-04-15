use js_sys;
use std::{convert::TryInto, str::FromStr};
use tokenizers::tokenizer::{Encoding, Tokenizer};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct TokenizerWasm {
    tokenizer: Tokenizer,
}

#[wasm_bindgen]
impl TokenizerWasm {
    #[wasm_bindgen(constructor)]
    pub fn from_buffer(json: String) -> TokenizerWasm {
        TokenizerWasm {
            tokenizer: Tokenizer::from_str(json.as_str()).unwrap_throw().into(),
        }
    }

    pub fn encode(&self, text: &str, add_special_tokens: bool) -> EncodingWasm {
        EncodingWasm {
            encoding: self
                .tokenizer
                .encode(text, add_special_tokens)
                .unwrap_throw(),
        }
    }

    pub fn decode(&self, tokens: js_sys::Object, skip_special_tokens: bool) -> String {
        let it = tokens.dyn_ref::<js_sys::Iterator>().unwrap_throw();
        let ids = it
            .into_iter()
            .map(|x| {
                let x_f64 = x.unwrap_throw()
                    .as_f64()
                    .unwrap_throw();
                x_f64 as u32
            })
            .collect();
        self.tokenizer
            .decode(ids, skip_special_tokens)
            .unwrap_throw()
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
