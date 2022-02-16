use clap::Parser;

/// Japanese <-> English translation with deepl and google
#[derive(Parser, Debug)]
pub struct Args {
    /// text you want to translate
    pub text: String,

    /// deepl only
    #[clap(short, long)]
    pub deepl: bool,

    /// google only
    #[clap(short, long)]
    pub google: bool,
}

pub fn parse() -> Args {
    Args::parse()
}
