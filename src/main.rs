mod word_generator;
use ::time::PreciseTime;

fn main() {
    let mut gerador_1 = word_generator::Generator::new();

    gerador_1.set_file_path("bla123.txt".to_string());

    let start = PreciseTime::now();
    gerador_1.set_pagination_size(10000i32);
    gerador_1.generate_from_mb_size(1024i32);

    //gerador_1.generate_from_lines(10000i32);

    //gerador_1.get_file();

    println!("{}", start.to(PreciseTime::now()))
}
