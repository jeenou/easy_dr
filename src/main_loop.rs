use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use std::sync::mpsc::Receiver;
//use std::sync::mpsc;
//use std::time::Duration;



// Define an enum for representing different types of tasks
pub enum _Task {
    StartProcess, // A task to start a process
    QuitProcess,  // A task to quit a process
}

// This function runs an infinite loop that receives tasks from a channel
// and processes them accordingly until it receives a QuitProcess task.
pub fn _task_loop(rx: Receiver<_Task>) {
    let mut running = true;

    while running {
        match rx.try_recv() {
            Ok(received) => match received {
                _Task::StartProcess => {
                    println!("start process");

                    }
                _Task::QuitProcess => {
                    println!("quit process");
                    running = false;
                }
            },
            Err(_) => {
                break;
            }
        }
    }
}

//This function starts a new process
fn _start() {
    /*
    Args:
        message (dict): task message
        processes (dict): running processes
        logs (dict): process logs
    */
     println!("Process started.")
    
}

//Creates a new process.
fn _create_process() {
    Command::new("mspaint")
        .spawn()
        .expect("failed to start paint program");
}

fn _open_predicer() {
    //Starts Predicer.

    let mut path = PathBuf::from("src");
    path.push("Predicer");

    let mut child = Command::new("julia")
        .current_dir(path)
        .args(&[
            "--eval",
            "using Pkg; Pkg.activate(\".\"); Pkg.instantiate();",
            "--eval",
            "using Predicer",
            "--eval",
            "mc, input_data = Predicer.generate_model(\"input_data/input_data.xlsx\")",
            "--eval",
            "Predicer.solve_model(mc)",
            "--eval",
            "Predicer.write_bid_matrix(mc, input_data)",
        ])
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    let input = b"]\nactivate .\nbackspace\n";
    let child_stdin = child.stdin.as_mut().unwrap();
    child_stdin.write_all(input).unwrap();

    // Wait for Julia to finish executing the commands
    let output = child.wait_with_output().expect("failed to wait on child");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}


/*
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
*/
