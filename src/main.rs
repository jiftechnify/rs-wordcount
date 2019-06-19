use std::env;
use std::fs::File;
use std::io::BufReader;

// lib.rsの内容は自クレート名で参照できる
use wordcount::count;
use wordcount::CountOption;

fn main() {
    let filename = env::args().nth(1).expect("1 argument FILENAME required");
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let freqs = count(reader, CountOption::default());
    println!("{:?}", freqs);
}

