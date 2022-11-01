use std::{env, fs::File, io::Read};

mod deox;

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

    match file {
        Ok(mut f) => {
            println!("File Found!");
            let mut buffer = [0; 8];
            f.read(&mut buffer);

            println!("{:?}", buffer);

            let h = deox::Header::new(buffer);
            match h {
                Ok(header) => {
                    println!("{:?}", header);
                    true
                }
                Err(e) => {
                    println!("Could not parse file: {}", e);
                    false
                }
            }
        }
        Err(_) => {
            println!("Error: File Not Found!");
            false
        }
    }
}
