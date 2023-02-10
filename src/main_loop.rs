
use std::sync::mpsc::{Receiver};

pub enum _Task {
    StartProcess,
    QuitProcess
}

pub fn task_loop(rx: Receiver<_Task>) {
    for received in rx {
        match received {
            _Task::StartProcess => {
                println!("start process");
            },
            _Task::QuitProcess => {
                println!("quit process");
            }
        }
    }
}

/*
fn _task_loop() {
    let running = true;

    while running {

        for received in receiver {
            println!("Received: {}", received);
        }
    }
}
*/