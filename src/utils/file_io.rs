use crate::utils::config::get_config;
use std::env;
use std::env::{home_dir, var};
use std::fs::{copy, create_dir_all, read_to_string, remove_file, File, OpenOptions};
use std::io::Write;
use std::path::Path;

pub const DEFAULT_BASE_DIR: &str = "~/.punch-card/";

pub fn write_file(path: &str, contents: String) {
    let path_str_to_write: String = expand_path(path);
    let path_to_write: &Path = Path::new(&path_str_to_write);
    if path_to_write.exists() {
        remove_file(path_str_to_write.clone()).expect("Should be able to delete");
    }
    let file_result: Result<File, std::io::Error> = OpenOptions::new()
        .create(true)
        .write(true)
        .open(path_str_to_write);
    if let Ok(mut file) = file_result {
        file.write_all(contents.as_bytes())
            .expect("Couldn't write to file!");
    } else {
        panic!("Couldn't create file {path}");
    }
}

pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    let path_to_read = expand_path(path);
    return read_to_string(path_to_read);
}

pub fn create_dir_if_not_exists(path: &str) {
    let dir_expanded: String = expand_path(path);
    if !Path::new(&dir_expanded).exists() {
        let expect_msg: String = format!("Unable to create directory: '{path}'");
        create_dir_all(dir_expanded).expect(&expect_msg);
    }
}

pub fn get_base_dir() -> String {
    if let Ok(punch_home) = var("PUNCH_CARD_HOME") {
        return punch_home;
    } else {
        return DEFAULT_BASE_DIR.to_owned();
    }
}

pub fn create_base_dir_if_not_exists() {
    create_dir_if_not_exists(&get_base_dir())
}

pub fn expand_path(path: &str) -> String {
    if path.starts_with("~/") {
        let home_path: String = match (var("HOME"), home_dir()) {
            (_, Some(home_dir_path)) => home_dir_path
                .to_str()
                .expect("Provided path is not valid (possibly has non-UTF-8 characters")
                .to_owned(),
            (Ok(home_dir), _) => home_dir,
            (_, _) => panic!("You are using an unsupported OS!"),
        };
        return home_path;
    } else {
        return path.to_string();
    };
}

pub fn edit_file(path: &str) {
    let config = get_config();

    let editor = if let Some(vim_path) = config.editor_path() {
        vim_path.clone()
    }
    // Debian/Ubuntu export an EDITOR variable
    else if let Ok(editor) = env::var("EDITOR") {
        editor.to_string()
    } else {
        "vim".to_string()
    };

    // Users may want to pass extra arguments to their editor eg: 'gedit -w' (which will open gedit and return to punch when the editor is closed)
    let mut command = editor.split_whitespace().collect::<Vec<&str>>();
    command.push(path);

    let editor = command.first().unwrap();
    let args = command.iter().skip(1);

    println!("Opening config with '{}'...", editor);

    std::process::Command::new(editor)
        .args(args)
        .spawn()
        .expect("Error: Failed to run editor, you can set 'editor_path' in the config")
        .wait()
        .expect("Error: Editor returned a non-zero status");

    println!("Editor closed.");
}

pub trait FromString<T, E> {
    fn try_from_string(yaml_str: &String) -> Result<T, E>;

    fn from_string(yaml_str: &String) -> T;
}

pub trait ToFile {
    fn get_path(&self) -> String;

    fn write(&self);
}

pub trait SafeFileEdit<T: FromString<T, E> + ToFile, E>: ToFile + FromString<T, E> {
    fn safe_edit_from_file(&self) {
        let std_path: String = self.get_path();
        let temp_path: String = (&std_path).to_string() + "-temp";
        copy(&std_path, &temp_path);
        edit_file(&temp_path);

        let yaml_str: String = read_file(&temp_path).unwrap();
        let new_result: Result<T, E> = T::try_from_string(&yaml_str);
        match new_result {
            Ok(new_value) => {
                remove_file(&std_path);
                new_value.write();
            }
            Err(_) => println!("Invalid Config created. Please try again"),
        };
        remove_file(temp_path);
    }
}
