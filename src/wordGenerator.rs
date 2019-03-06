#![allow(dead_code)]

use std::fs::create_dir;
use std::fs::metadata;
use std::fs::File;

pub struct Generator {
    file_path: String,
}

impl Generator {
    pub fn new(file_name: String) -> Generator {
        let base_path: String = "./FILES_DIRECTORY/".to_string();

        Generator {
            file_path: format!("{}{}", base_path, file_name),
        }
    }

    pub fn get_file_path(&self) -> &String {
        &self.file_path
    }
}

// pub fn save_file() {
//     // if check_file_exist() {
//     //     print!("HUE");
//     // } else {

//     // }
// }

// fn check_file_exist() -> bool {
//     false
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
