use anyhow::{anyhow, Result};
use reqwest::Url;
use serde::Deserialize;

use super::{Translater, Translation};
use crate::language::Lang;

pub struct Google {
    input: String,
}

impl Translater for Google {
    fn new(input: &str) -> Self {
        Google {
            input: input.to_string(),
        }
    }

    fn translate(&self) -> Translation {
        match self.request() {
            Ok(res) => Translation {
                service_name: "Google".to_string(),
                result: Ok(res
                    .data
                    .translations
                    .iter()
                    .map(|t| {
                        html_escape::decode_html_entities(&t.translated_text.clone()).into_owned()
                    })
                    .collect()),
            },
            Err(e) => Translation {
                service_name: "Google".to_string(),
                result: Err(vec![e.to_string()]),
            },
        }
    }
}

impl Google {
    fn get_langs(&self) -> (String, String) {
        match Lang::from_source(&self.input) {
            Lang::En => (String::from("en"), String::from("ja")),
            Lang::Jp => (String::from("ja"), String::from("en")),
        }
    }

    fn request(&self) -> Result<ResponseBody> {
        let api_key = std::env::var("DEEGLE_GOOGLE_API_KEY")?;
        let (source, target) = self.get_langs();

        let url = Url::parse_with_params(
            "https://translation.googleapis.com/language/translate/v2",
            &[
                ("q", self.input.clone()),
                ("key", api_key),
                ("target", target),
                ("source", source),
            ],
        )?;

        let res = reqwest::blocking::get(url)?;

        if res.status().is_success() {
            let res = res.json::<ResponseBody>()?;
            Ok(res)
        } else {
            let err = res.json::<ErrorResponse>()?;
            Err(anyhow!(
                "{}:{}, {}",
                err.error.code,
                err.error.status,
                err.error.message
            ))
        }
    }
}

#[derive(Deserialize, Debug)]
struct ResponseBody {
    data: TranslateTextResponseList,
}

#[derive(Deserialize, Debug)]
struct TranslateTextResponseList {
    translations: Vec<TranslateTextResponseTranslation>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TranslateTextResponseTranslation {
    // detected_source_language: String,
    translated_text: String,
}

#[derive(Deserialize, Debug)]
struct ErrorResponse {
    error: GoogleError,
}

#[derive(Deserialize, Debug)]
struct GoogleError {
    message: String,
    code: u32,
    status: String,
}
