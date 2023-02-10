
use std::sync::mpsc::{channel, Sender};
mod main_loop;

/*
enum Task {
    Quit,
    StartProcess
}
*/

pub fn start_sending(tx: Sender<&str>) {
    
    tx.send("start").unwrap();
    tx.send("quit").unwrap();
    
}

fn main() {
    let (tx, rx) = channel();
    start_sending(tx);
    main_loop::task_loop(rx);
}