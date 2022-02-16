use anyhow::Result;
use colored::*;
use std::fmt::Display;

mod deepl;
mod google;

pub use deepl::Deepl;
pub use google::Google;

pub trait Translater {
    fn new(input: &str) -> Self
    where
        Self: Sized + Sync + Send;
    fn translate(&self) -> Translation;
}

#[derive(Debug)]
pub struct Translation {
    service_name: String,
    result: Result<Vec<String>, Vec<String>>,
}

impl Display for Translation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let service = format!("{}:", self.service_name.green()).green();

        let result = match &self.result {
            Ok(r) => r.join("\n").cyan(),
            Err(e) => e.join("\n").red(),
        };

        write!(f, "{}\n{}", service, result)
    }
}
