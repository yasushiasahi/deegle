use std::thread;

mod arg;
mod language;
mod translater;

use translater::{Deepl, Google, Translater};

pub fn run() {
    let args = arg::parse();

    let translaters: Vec<Box<dyn Translater + Send + Sync>> = match (args.deepl, args.google) {
        (true, false) => vec![Box::new(Deepl::new(&args.text))],
        (false, true) => vec![Box::new(Google::new(&args.text))],
        _ => vec![
            Box::new(Deepl::new(&args.text)),
            Box::new(Google::new(&args.text)),
        ],
    };

    let mut handles = Vec::new();
    for t in translaters {
        handles.push(thread::spawn(move || t.translate()))
    }

    for h in handles {
        let r = h.join().unwrap();
        println!("{}\n", r);
    }
}
