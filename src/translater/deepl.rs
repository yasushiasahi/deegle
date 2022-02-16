use anyhow::{anyhow, Result};
use reqwest::header::HeaderMap;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::StatusCode;
use serde::Deserialize;

use super::{Translater, Translation};
use crate::language::Lang;

pub struct Deepl {
    input: String,
}

impl Translater for Deepl {
    fn new(input: &str) -> Self
    where
        Self: Sized + Sync + Send,
    {
        Deepl {
            input: String::from(input),
        }
    }

    fn translate(&self) -> Translation {
        match self.request() {
            Ok(res) => Translation {
                service_name: String::from("Deepl"),
                result: Ok(res.translations.iter().map(|t| t.text.to_owned()).collect()),
            },
            Err(err) => Translation {
                service_name: String::from("Deepl"),
                result: Err(vec![err.to_string()]),
            },
        }
    }
}

impl Deepl {
    fn make_parameters(&self) -> String {
        match Lang::from_source(&self.input) {
            Lang::En => format!("text={}&source_lang=EN&target_lang=JA", self.input),
            Lang::Jp => format!("text={}&source_lang=JA&target_lang=EN-US", self.input),
        }
    }

    fn request(&self) -> Result<ResponseBody> {
        let api_key = std::env::var("DEEGLE_DEEPL_API_KEY")?;

        let client = reqwest::blocking::Client::new();
        let res = client
            .post("https://api-free.deepl.com/v2/translate")
            .headers(make_headers(&api_key))
            .body(self.make_parameters())
            .send()?;

        if res.status().is_success() {
            let res = res.json::<ResponseBody>()?;
            Ok(res)
        } else {
            let status = res.status();

            if let Ok(err) = &res.json::<ErrorResponse>() {
                return Err(anyhow!("{}", err.message));
            }

            match status {
                StatusCode::FORBIDDEN => Err(anyhow!(StatusCode::FORBIDDEN.to_string())),
                _ => Err(anyhow!("unknown error")),
            }
        }
    }
}

fn make_headers(api_key: &str) -> HeaderMap {
    let mut h = HeaderMap::new();
    h.insert(
        AUTHORIZATION,
        format!("DeepL-Auth-Key {}", api_key).parse().unwrap(),
    );
    h.insert(
        CONTENT_TYPE,
        "application/x-www-form-urlencoded".parse().unwrap(),
    );

    h
}

#[derive(Deserialize, Debug)]
struct ResponseBody {
    translations: Vec<DeeplTranslation>,
}

#[derive(Deserialize, Debug)]
struct DeeplTranslation {
    // detected_source_language: String,
    text: String,
}

#[derive(Deserialize, Debug)]
struct ErrorResponse {
    message: String,
}
