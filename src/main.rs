mod word_generator;
use ::time::PreciseTime;
use std::{thread, time};

fn main() {
    let mut gerador_1 = word_generator::Generator::new();
    gerador_1.set_file_path("bla123.txt".to_string());

    let start = PreciseTime::now();
    //gerador_1.generate_from_lines(10_000_000i32);
    thread::sleep(time::Duration::from_millis(10));

    let end = PreciseTime::now();
    println!("{}", start.to(end))
}
