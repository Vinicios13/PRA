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
    pagination_size: i32,
}

impl Generator {
    pub fn new() -> Generator {
        Generator {
            file_path: format!("{}{}", BASE_PATH, "output.txt".to_string()),
            pagination_size: 300i32,
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

    pub fn generate_from_lines(&self, mut qty: i32) {
        loop {
            if qty - self.pagination_size > 0 {
                qty = qty - self.pagination_size;

                self.generate_line_by_num(self.pagination_size);
            } else {
                self.generate_line_by_num(qty);
                break;
            }
        }
    }

    pub fn generate_from_mb_size(&self, size: i32) -> std::io::Result<()> {
        while std::fs::metadata(self.get_file_path())?.len() as f32 / 1e+6f32 < size as f32 {
            self.generate_line_by_num(self.pagination_size)
        }
        Ok(())
    }

    pub fn get_file(&self) -> std::io::Result<()> {
        let file = File::open(self.get_file_path())?;
        let file = BufReader::new(file);

        for (num, line) in file.lines().enumerate().take(self.pagination_size as usize) {
            println!("{} : {}", num, line?.to_uppercase());
        }
        Ok(())
    }

    fn generate_line_by_num(&self, qty: i32) {
        let mut temp_string: String = "".to_owned();

        for _i in 0..qty {
            temp_string.push_str(&self.generate_line());
        }

        self.save_file(temp_string)
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

    fn generate_line(&self) -> String {
        format!(
            "{} x {} - {} x {} - {} - {}\n",
            self.generate_string(),
            self.generate_string(),
            self.generate_number(),
            self.generate_number(),
            self.generate_date(),
            self.generate_number_of_people()
        )
    }

    fn generate_string(&self) -> String {
        (0..rand::thread_rng().gen_range(1, 21))
            .map(|_| (ASCII_LOWER[rand::thread_rng().gen_range(1, 26)]))
            .collect()
    }

    fn generate_number(&self) -> i32 {
        rand::thread_rng().gen_range(0, 16)
    }

    fn generate_date(&self) -> String {
        let day = rand::thread_rng().gen_range(1, 32);
        let month = rand::thread_rng().gen_range(1, 13);
        let year = rand::thread_rng().gen_range(2012, 2020);

        format!("{}/{}/{}", day, month, year)
    }

    fn generate_number_of_people(&self) -> i32 {
        rand::thread_rng().gen_range(0, 50_001)
    }
}
