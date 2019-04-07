#![allow(dead_code)]

use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{BufRead, BufReader};

const BASE_PATH: &str = "./FILES_DIRECTORY/";

const ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub struct Generator {
    file_path: String,
    total_itens: u32,
    header_is_set: bool,
    pagination_size: i32,
}

impl Generator {
    pub fn new() -> Generator {
        Generator {
            file_path: format!("{}{}", BASE_PATH, "output.txt".to_string()),
            header_is_set: false,
            pagination_size: 300i32,
            total_itens: 0u32,
        }
    }

    pub fn get_file_path(&self) -> &String {
        &self.file_path
    }

    pub fn set_file_path(&mut self, file_name: String) {
        self.file_path = format!("{}{}", BASE_PATH, file_name);
        self.create_file();
    }

    pub fn set_pagination_size(&mut self, size: i32) {
        self.pagination_size = size
    }

    pub fn generate_from_lines(&mut self, mut qty: i32) {
        self.set_header_if_not_set();

        loop {
            if qty - self.pagination_size > 0 {
                qty -= self.pagination_size;

                self.generate_line_by_num(self.pagination_size);
            } else {
                self.generate_line_by_num(qty);
                break;
            }
        }
    }

    pub fn generate_from_mb_size(&mut self, size: i32) {
        self.set_header_if_not_set();

        while std::fs::metadata(self.get_file_path())
            .expect("Couldn't open the file")
            .len() as f32
            / 1e+6f32
            < size as f32
        {
            self.generate_line_by_num(self.pagination_size)
        }
    }

    pub fn print_file(&self) {
        let file =
            BufReader::new(File::open(self.get_file_path()).expect("Couldn't open the file"));

        for (_num, line) in file.lines().enumerate().take(self.pagination_size as usize) {
            if let Ok(line) = line {
                println!("{}", line.to_uppercase())
            }
        }
    }

    fn generate_line_by_num(&mut self, qty: i32) {
        let mut temp_string: String = "".to_owned();

        for _i in 0..qty {
            self.increment_total_lines();
            temp_string.push_str(&self.generate_line(self.get_total_lines()));
        }

        self.save_file(temp_string)
    }

    fn get_total_lines(&self) -> u32 {
        self.total_itens
    }

    fn increment_total_lines(&mut self) {
        self.total_itens += 1u32
    }

    fn save_file(&self, data: String) {
        let mut file = OpenOptions::new()
            .append(true)
            .open(self.get_file_path())
            .unwrap();

        match write!(file, "{}", data) {
            Ok(_) => (),
            Err(why) => panic!("couldn't save {}: {}", self.file_path, why.description()),
        }
    }

    fn check_file_exist(&self) -> bool {
        std::fs::metadata(self.get_file_path()).is_ok()
    }

    fn create_file(&self) {
        match File::create(self.get_file_path()) {
            Ok(_) => (),
            Err(why) => panic!("couldn't create {}: {}", self.file_path, why.description()),
        }
    }

    fn generate_line(&self, index: u32) -> String {
        format!(
            "{} | {} | {} | {} | {} | {} | {}\n",
            self.get_formated_index(index),
            self.generate_string(),
            self.generate_string(),
            self.generate_number(),
            self.generate_number(),
            self.generate_date(),
            self.generate_number_of_people()
        )
    }

    fn get_formated_index(&self, index: u32) -> String {
        let string_index = index.to_string();
        let index_size = string_index.chars().count();

        let white_spaces: String = (0..7 - index_size).map(|_| " ".to_string()).collect();

        format!("{}{}", index, white_spaces)
    }

    fn generate_string(&self) -> String {
        let string: String = (0..rand::thread_rng().gen_range(1, 21))
            .map(|_| (ASCII_LOWER[rand::thread_rng().gen_range(1, 26)]))
            .collect();

        let string_size = string.chars().count();

        let white_spaces: String = (0..20 - string_size).map(|_| " ".to_string()).collect();

        format!("{}{}", string, white_spaces)
    }

    fn generate_number(&self) -> String {
        let string = rand::thread_rng().gen_range(0, 16).to_string();

        if string.chars().count() < 2 {
            format!("0{}      ", string)
        } else {
            format!("{}      ", string)
        }
    }

    fn generate_date(&self) -> String {
        let mut day = rand::thread_rng().gen_range(1, 32).to_string();
        let mut month = rand::thread_rng().gen_range(1, 13).to_string();
        let year = rand::thread_rng().gen_range(2012, 2020);

        if day.chars().count() < 2 {
            day = format!("0{}", day);
        }

        if month.chars().count() < 2 {
            month = format!("0{}", month);
        }

        format!("{}/{}/{}", day, month, year)
    }

    fn generate_number_of_people(&self) -> i32 {
        rand::thread_rng().gen_range(0, 50_001)
    }

    fn set_header_if_not_set(&mut self) {
        if !self.header_is_set {
            self.save_file(format!(
                "{} | {} | {} | {} | {} | {} | {}\n",
                "  INDEX",
                "       TEAM 1       ",
                "       TEAM 2       ",
                " Score 1",
                " Score 2",
                " GAME DATE",
                " PUBLIC NUMBER"
            ));
            self.header_is_set = true;
        }
    }
}
