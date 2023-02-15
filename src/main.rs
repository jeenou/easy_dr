
use std::sync::mpsc::{channel, Sender};
mod main_loop;

pub fn start_sending(tx: Sender<main_loop::_Task>) {
    
    tx.send(main_loop::_Task::StartProcess).unwrap();
    tx.send(main_loop::_Task::QuitProcess).unwrap();
    tx.send(main_loop::_Task::StartProcess).unwrap();
}

fn main() {
    let (tx, rx) = channel();
    start_sending(tx);
    main_loop::task_loop(rx);
}