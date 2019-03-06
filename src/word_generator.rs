#![allow(dead_code)]

use std::error::Error;
use std::fs::File;

const BASE_PATH: &str = "./FILES_DIRECTORY/";

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

    pub fn check_file_exist(&self) -> bool {
        std::fs::metadata(self.get_file_path()).is_ok()
    }

    pub fn create_file(&self) {
        match File::create(self.get_file_path()) {
            Ok(_) => (),
            Err(why) => panic!("couldn't create {}: {}", self.file_path, why.description()),
        }
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
