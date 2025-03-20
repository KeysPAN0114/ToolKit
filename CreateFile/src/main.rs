use std::{self, fs::{self, File}};
use std::io;

fn main() -> std::io::Result<()> {
    let mut _input = String::new();
    let mut _folder_name = String::new();
    let mut _path_string = String::new();

    println!("Do you need to create a folder?(Y/N):");
    io::stdin().read_line(&mut _input).unwrap();
    match _input.trim() {
        "Y" | "y" | "Yes" | "yes" | "YES" => {
            println!("Enter the folder name:");
            io::stdin().read_line(&mut _folder_name).unwrap();
            fs::create_dir(&_folder_name.trim());
        },
        "N" | "n" | "No" | "no" | "NO" => {},
        _ => {},
    }
    println!("Enter the file name:");
    io::stdin().read_line(&mut _path_string).unwrap();
    let _path_string = if !_folder_name.is_empty() {
        format!("{}/{}",_folder_name.trim(), _path_string.trim())
    } else {
        _path_string.trim().to_string()
    };
    match File::create(&_path_string) {
        Ok(_) => {
            println!("File created successfully");
            fs::write(&_path_string, "hello world")?;
            Ok(())
        }
        Err(e) => {
            println!("Error creating file: {}", e);
            Err(e)
        }
    }
}