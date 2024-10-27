use std::env;

pub fn get_file_path(path: &str) -> String {
    let mut full_path = env::current_dir().unwrap();
    full_path.push(path);

    full_path.to_str().unwrap().to_string()
}