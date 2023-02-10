
use std::sync::mpsc::{Receiver};

enum Field {
    Task,
    ExecutionId,
    ProcessCommand,
    ProcessArguments
}

pub fn task_loop(rx: Receiver<&str>) {
    for received in rx {
        if received == "start" {
            println!("start")
        }
        else if received == "quit" {
            println!("quit")
        } else {
            println!("jotain meni vikaan")
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