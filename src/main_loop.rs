use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc::Receiver;
use umya_spreadsheet::*;

// Define an enum for representing different types of tasks
pub enum _Task {
    StartProcess, // A task to start a process
    QuitProcess,  // A task to quit a process
}

// Check if a directory exists at the given path
pub fn _directory_exists(path: &str) -> bool {
    let dir = Path::new(path); // Create a Path object from the path string
    dir.exists() && dir.is_dir() // Check if the path exists and is a directory
}

// This function takes a vector of vectors of strings as input
// and writes the data to an Excel file.
pub fn _write_file_vector2(values: Vec<Vec<std::string::String>>) {
    // Create a new Excel workbook.
    let mut book = new_file();

    // Write the values to a new worksheet.
    for (i, row_values) in values.iter().enumerate() {
        for (j, cell_value) in row_values.iter().enumerate() {
            let cell_ref = format!("{}{}", (j as u8 + b'A') as char, i + 1);
            book.get_sheet_by_name_mut("Sheet1")
                .unwrap()
                .get_cell_mut(&cell_ref)
                .set_value(cell_value);
        }
    }

    // Write the workbook to a file at the specified path.
    let path = std::path::Path::new("C:/spread_test_data/ccc.xlsx");
    let _ = writer::xlsx::write(&book, path);
}

//This function creates vector that contains model parameters
pub fn _create_model(devices: Vec<String>, parameters: Vec<String>) -> Vec<Vec<String>> {
    // Create a 2D vector with one row for each device and two columns.
    let mut result = vec![vec!["".to_string(); 2]; devices.len()];

    // Fill in the first column with the device names.
    for (i, device) in devices.iter().enumerate() {
        result[i][0] = device.to_string();
    }

    // Fill in the second column with the parameter names.
    for (i, param) in parameters.iter().enumerate() {
        result[i][1] = param.to_string();
    }

    result
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
                    _start();
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

    _open_predicer();
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
