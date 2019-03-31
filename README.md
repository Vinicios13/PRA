### Usage

```rust no_run
mod word_generator;

fn main() {
    let mut gerador_1 = word_generator::Generator::new();
    //creates a new file under FILES_DIRECTORY
    gerador_1.set_file_path('test');

    //define the number of lines that will be stored in memory before saving it into the file
    gerador_1.set_pagination_size(10000i32);

    //there are two ways to generate txt, by mb size or number of lines
    gerador_1.generate_from_lines(10000i32); //will generate 10000 lines
    gerador_1.generate_from_mb_size(1024i32); //will generate 1 gb of text

    //will print the file till pagination limit
    gerador_1.print_file();
}
```
