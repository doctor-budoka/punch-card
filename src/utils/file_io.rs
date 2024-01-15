use std::fs::{File, OpenOptions, create_dir_all,read_to_string};
use std::io::Write;
use std::path::Path;
use std::env::var;

pub const BASE_DIR: &str = "~/.punch-card/";

pub fn write_file(path: &str, contents: String) {
    let path_to_write: String = expand_path(path);
    let file_result: Result<File, std::io::Error> = OpenOptions::new().create(true).write(true).open(path_to_write);
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

pub fn create_base_dir_if_not_exists() {
    create_dir_if_not_exists(&BASE_DIR)
}

pub fn expand_path(path: &str) -> String {
    return if path.starts_with("~/") {
        var("HOME").unwrap() + &path[1..]
    }else {path.to_string()};
}


pub fn edit_file_in_vim(path: &String) {
    std::process::Command::new("vim")
    .arg(path)
    .spawn()
    .expect("Error: Failed to run editor")
    .wait()
    .expect("Error: Editor returned a non-zero status");
}


pub trait FromString<T, E> {
    fn try_from_string(yaml_str: &String) -> Result<T, E>;

    fn from_string(yaml_str: &String) -> T;
}

pub trait ToFile  {
    fn get_path(&self) -> String;

    fn write(&self);
}

pub trait SafeFileEdit<T:FromString<T,E> + ToFile, E>: ToFile + FromString<T, E> {
    fn safe_edit_from_file(&self) {
        let std_path: String = self.get_path();
        let temp_path: String = (&std_path).to_string() + "-temp";
        std::process::Command::new("cp").args([&std_path, &temp_path]).output().expect("Failed to create temporary data!");
    
        println!("Opening config in vim...");
        edit_file_in_vim(&temp_path);
        println!("Vim closed.");
        let yaml_str: String = read_file(&temp_path).unwrap();
        let new_result: Result<T, E> = T::try_from_string(&yaml_str);
        match new_result {
            Ok(new_value) => {
                std::process::Command::new("rm").arg(&std_path).output().expect("Failed to clean up the temporary data!");
                new_value.write();
            },
            Err(_) => println!("Invalid Config created. Please try again"),
        };
        std::process::Command::new("rm").arg(&temp_path).output().expect("Failed to clean up the temporary data!");
    }
}
