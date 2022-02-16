use anyhow::Result;

mod deepl;
mod google;

pub use deepl::Deepl;
pub use google::Google;

#[derive(Debug)]
pub struct Translation {
    pub service_name: String,
    pub result: Result<Vec<String>, Vec<String>>,
}

pub trait Translater {
    fn new(input: &str) -> Self
    where
        Self: Sized + Sync + Send;
    fn translate(&self) -> Translation;
}
