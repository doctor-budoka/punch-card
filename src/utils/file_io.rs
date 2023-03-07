use std::fs::{File, OpenOptions, create_dir_all,read_to_string};
use std::io::Write;
use std::path::Path;
use std::env::var;

pub fn write_file(path: &str, contents: String) {
    let path_to_write: String = expand_path(path);
    let file_result: Result<File, std::io::Error> = OpenOptions::new().create_new(true).write(true).open(path_to_write);
    if let Ok(mut file) = file_result {
        file.write_all(contents.as_bytes()).expect("Couldn't write to file!");
    }
    else {
        panic!("Couldn't create file {path}");
    }
}

pub fn read_file(path: &str) -> Result<String,std::io::Error> {
    let path_to_read = expand_path(path);
    return read_to_string(path_to_read);
}

pub fn create_dir_if_not_exists(path: &str)  {
    let dir_expanded: String = expand_path(path);
    if !Path::new(&dir_expanded).exists() {
        let expect_msg: String = format!("Unable to create directory: '{path}'");
        create_dir_all(dir_expanded).expect(&expect_msg);
    }
} 

pub fn expand_path(path: &str) -> String {
    return if path.starts_with("~/") {
        var("HOME").unwrap() + &path[1..]
    }else {path.to_string()};
}
