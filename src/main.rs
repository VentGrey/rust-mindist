use std::fs::File;
use std::io::{BufRead, BufReader};

struct Iris {
    num: [f64; 4],
    id: [char; 20]
}

struct Comp {
    num: [f64; 6],
}

fn main() {
    // Abrir el archivo
    let mut f = BufReader::new(File::open("iris.data").unwrap());
}
