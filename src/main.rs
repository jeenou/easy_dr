
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

    let n1 = "moi";
    let n2 = true;
    let n3 = false;
    model_builder::_node::<()>(n1,n2,n3);
    //model_builder::_node::<()>(node_name, is_commodity, is_market);


}