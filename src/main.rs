
use std::sync::mpsc::{Sender};
mod main_loop;
mod utilities;
mod julia_interface;
mod model_builder;

pub fn start_sending(tx: Sender<main_loop::_Task>) {
    
    tx.send(main_loop::_Task::StartProcess).unwrap();
    tx.send(main_loop::_Task::QuitProcess).unwrap();
    tx.send(main_loop::_Task::StartProcess).unwrap();
}


fn main() {

    model_builder::_test::<()>();

    
}