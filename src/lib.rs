use std::env;
use std::thread;

mod language;
mod translater;

use translater::{Deepl, Google, Translater};

pub fn run() {
    let input = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("no text");
        std::process::exit(1);
    });

    let translaters: Vec<Box<dyn Translater + Send + Sync>> =
        vec![Box::new(Deepl::new(&input)), Box::new(Google::new(&input))];

    let mut handles = Vec::new();
    for t in translaters {
        handles.push(thread::spawn(move || t.translate()))
    }

    for h in handles {
        let r = h.join().unwrap();
        println!("{}", r.service_name);
        match r.result {
            Ok(rs) => {
                for s in rs {
                    println!("{}", s);
                }
            }
            Err(es) => {
                for s in es {
                    println!("{}", s);
                }
            }
        }
    }
}
