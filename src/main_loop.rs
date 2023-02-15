
use std::sync::mpsc::{Receiver};
use std::process::{Command, Stdio};
use std::io::Write;
use std::path::{PathBuf, Path};

pub enum _Task {
    StartProcess,
    QuitProcess
}

fn _directory_exists(path: &str) -> bool {
    let dir = Path::new(path);
    dir.exists() && dir.is_dir()
}

pub fn task_loop(rx: Receiver<_Task>) {
    let mut running = true;

    while running {
        match rx.try_recv() {
            Ok(received) => {
                match received {
                    _Task::StartProcess => {
                        println!("start process");
                        _start();

                    },
                    _Task::QuitProcess => {
                        println!("quit process");
                        running = false;
                    }
                }
            }
            Err(_) => {
                break;
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

    open_predicer();
    
}

fn _create_process() {
    //Starts a new process.
    Command::new("mspaint")
    .spawn()
    .expect("failed to start paint program");
}

fn open_predicer() {
    //Starts Predicer.

    let mut path = PathBuf::from("src");
    path.push("Predicer");

    let mut child = Command::new("julia")
    .current_dir(path)
    .args(&[
        "--eval", "using Pkg; Pkg.activate(\".\"); Pkg.instantiate();",
        "--eval", "using Predicer",
        "--eval", "mc, input_data = Predicer.generate_model(\"input_data/input_data.xlsx\")",
        "--eval", "Predicer.solve_model(mc)",
        "--eval", "Predicer.write_bid_matrix(mc, input_data)"
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

fn _quit_process() {
    //Terminates a running process
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

#[test]
fn test_predicer_results_dir() {

    let mut dir_path = PathBuf::from("src");
    dir_path.push("Predicer/results");

    let dir_exists = dir_path.exists() && dir_path.is_dir();
    assert!(dir_exists, "Directory {} does not exist", dir_path.display());

    //open_predicer();
    /*
    let new_files = fs::read_dir("input_data").unwrap().count();
    assert_eq!(new_files, initial_files + 1, "No new file was created");
    */
}
