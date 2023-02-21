
use std::sync::mpsc::{Sender};
use std::path::{PathBuf};
mod main_loop;

pub fn start_sending(tx: Sender<main_loop::_Task>) {
    
    tx.send(main_loop::_Task::StartProcess).unwrap();
    tx.send(main_loop::_Task::QuitProcess).unwrap();
    tx.send(main_loop::_Task::StartProcess).unwrap();
}

fn main() {
    //let (tx, rx) = channel();
    //start_sending(tx);
    //main_loop::task_loop(rx);

    let mut path = PathBuf::from("src");
    path.push("Predicer/input_data/input_data_6.xlsx");

    main_loop::read_file(&path);
}