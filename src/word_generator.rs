#![allow(dead_code)]

use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

const BASE_PATH: &str = "./FILES_DIRECTORY/";

const ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub struct Generator {
    file_path: String,
}

impl Generator {
    pub fn new() -> Generator {
        Generator {
            file_path: format!("{}{}", BASE_PATH, "output.txt".to_string()),
        }
    }

    pub fn get_file_path(&self) -> &String {
        &self.file_path
    }

    pub fn set_file_path(&mut self, file_name: String) {
        self.file_path = format!("{}{}", BASE_PATH, file_name);
        self.create_file();
    }

    pub fn generate_from_lines(&self, qty: i32) {
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
        let temp: String = (0..10)
            .map(|_| (ASCII_LOWER[rand::thread_rng().gen_range(1, 26)]))
            .collect();
        format!("{}\n", temp)
    }
}
// pub fn save_file() {
//     // if check_file_exist() {
//     //     print!("HUE");
//     // } else {

//     // }
// }

// fn check_directory_path_exists_create_if_not() -> bool {
//     if !metadata(DIRECTORY_PATH).is_ok() {
//         match create_dir(DIRECTORY_PATH) {
//             Ok(()) => true,
//             Err(_) => panic!(
//                 "Erro ao criar a pasta '{}' na root do projeto",
//                 DIRECTORY_PATH
//             ),
//         }
//     } else {
//         true
//     }
// }
