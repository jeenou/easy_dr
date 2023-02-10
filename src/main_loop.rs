
use std::sync::mpsc::{Receiver};
use std::process::Command;

pub enum _Task {
    StartProcess,
    QuitProcess
}

pub fn task_loop(rx: Receiver<_Task>) {
    for received in rx {
        match received {
            _Task::StartProcess => {
                _start();
            },
            _Task::QuitProcess => {
                println!("quit process");
            }
        }
    }
}

fn _start() {
    /*Starts a new process.

    Args:
        message (dict): task message
        processes (dict): running processes
        logs (dict): process logs
    */

    _create_process();
    
}

fn _create_process() {
    //Palauttaa prosessin
    Command::new("mspaint")
    .spawn()
    .expect("failed to start paint program");
}

#[test]
fn test_create_process() {
    _create_process();
    let result = Command::new("tasklist")
        .arg("/fi")
        .arg("imagename eq mspaint.exe")
        .output()
        .expect("failed to execute tasklist command");

    let output = String::from_utf8_lossy(&result.stdout);
    assert!(output.contains("mspaint.exe"), "mspaint.exe not found in tasklist output");
}
