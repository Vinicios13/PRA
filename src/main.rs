mod word_generator;

fn main() {
    let mut gerador_1 = word_generator::Generator::new();
    gerador_1.set_file_path("a.txt".to_string());
}
