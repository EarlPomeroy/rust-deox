#![allow(unused_must_use)]

use std::{env, fs::File, io::Read};

mod deox;
use crate::deox::header::Header;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Missing class file or jar\n");
        println!("help: ./rust-deox file.[class|jar]");
        return;
    }

    read_file(&args[1]);
}

fn read_file(file_name: &String) -> bool {
    let file = File::open(file_name);

    if let Ok(mut f) = file {
        println!("File Found!");
        let mut buffer = [0; 8];
        f.read(&mut buffer);

        let h = Header::new(buffer);

        match h {
            Ok(header) => {
                println!("{}", header);
                true
            }
            Err(e) => {
                println!("Could not parse file: {}", e);
                false
            }
        }
    } else {
        println!("Error: File Not Found!");
        false
    }
}
