use std::io;
use chumsky::prelude::*;

fn main() {
    println!("Novarc 0.1.0 ready.\n");

    let mut line = String::new();
    loop {
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        println!("{}", line);
    }
}
