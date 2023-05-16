use std::env;

pub fn get_working_dir() -> std::string::String {
    if let Ok(current_dir) = env::current_dir() {
        return current_dir.to_string_lossy().into_owned();
    } else {
        panic!("Failed to determine the current directory.")
    }
}

pub fn is_dir(possible_dir: &str) -> bool {
    if let Some(last_char) = possible_dir.chars().last() {
        if last_char == '/' {
            return true;
        } else {
            return false;
        }
    }

    return false;
}
