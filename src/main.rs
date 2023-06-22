use std::sync::mpsc::{channel, Sender};
mod main_loop;
use std::path::PathBuf;
use std::path::Path;

pub fn start_sending(tx: Sender<main_loop::_Task>) {
    
    tx.send(main_loop::_Task::StartProcess).unwrap();
    tx.send(main_loop::_Task::QuitProcess).unwrap();
    tx.send(main_loop::_Task::StartProcess).unwrap();
}

fn main() {
    let (tx, rx) = channel();
    start_sending(tx);
    main_loop::task_loop(rx);
    let mut folder_path = PathBuf::from("src");
    folder_path.push("Predicer/results");

    let file_path_var: Option<PathBuf>; // Declare the variable here

    if let Some(newest_file) = main_loop::_get_newest_file(folder_path) {
        let file_path: &Path = newest_file.as_ref();
        println!("Newest file: {:?}", file_path);

        // Store the file path in the variable
        file_path_var = Some(newest_file.clone());
    } else {
        println!("No files found in the folder or unable to access the folder.");
        return; // Exit early if no files are found
    }

    if let Some(file_path) = file_path_var {
        main_loop::_read_file(&file_path);
    }
}
