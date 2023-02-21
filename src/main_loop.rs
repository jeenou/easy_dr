
use std::sync::mpsc::{Receiver};
use std::process::{Command, Stdio};
use std::io::Write;
use std::path::{PathBuf, Path};
//use std::fs;
use umya_spreadsheet::*;

pub enum _Task {
    StartProcess,
    QuitProcess
}

fn _directory_exists(path: &str) -> bool {
    let dir = Path::new(path);
    dir.exists() && dir.is_dir()
}

pub fn read_file(path: &PathBuf) {
    //read a file
    let mut book = reader::xlsx::read(path).unwrap();
    book.get_sheet_by_name_mut("Sheet1").unwrap().get_cell_mut("A1").set_value("TEST1");
    let _ = writer::xlsx::write(&book, path);
}

pub fn _task_loop(rx: Receiver<_Task>) {
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

    _open_predicer();
    
}

fn _create_process() {
    //Starts a new process.
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

#[test]
fn test_open_predicer() {
    let mut dir = PathBuf::from("src");
    dir.push("Predicer/results");

    if !dir.exists() {
        let mut dir_path = PathBuf::from("src");
        dir_path.push("Predicer/results");
        _open_predicer();
        let dir_exists = dir_path.exists() && dir_path.is_dir();
        assert!(dir_exists, "Directory {} does not exist", dir_path.display());

    }
    else {
        let new_dir_path = PathBuf::from("src/Predicer/results");

        let before_files = fs::read_dir(&new_dir_path).unwrap().count();
        _open_predicer();
        let after_files = fs::read_dir(&new_dir_path).unwrap().count();

        assert!(after_files > before_files, "Error: no new files were created");
    }

}