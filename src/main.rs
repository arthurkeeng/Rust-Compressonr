extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    if args().len() != 3 {
        eprintln!("this is how to use : `source of file` , `target of compressed file`");
        return;
    }

    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());

    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();

    let output = encoder.finish().unwrap();

    println!("source len : {:?} ", input.get_ref().metadata().unwrap());
    println!("target length : {:?}", output.metadata().unwrap().len());

    println!("time elapsed : {:?}", start.elapsed())
}
