
use std::sync::mpsc::{Receiver};
use std::process::{Command, Stdio};
use std::io::Write;
use std::path::PathBuf;
use std::fs;
use std::path::Path;
use umya_spreadsheet::*;

pub enum _Task {
    StartProcess,
    QuitProcess
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

pub fn _get_newest_file(folder_path: PathBuf) -> Option<PathBuf> {
    let path = folder_path.as_path();
    let folder = Path::new(path);

    // Read the contents of the folder
    let entries = match fs::read_dir(folder) {
        Ok(entries) => entries,
        Err(_) => return None, // Unable to read the folder
    };

    let mut newest_file: Option<PathBuf> = None;
    let mut newest_modified_time: Option<std::time::SystemTime> = None;

    // Iterate through the files in the folder
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();

            // Retrieve metadata for the file
            let metadata = match fs::metadata(&path) {
                Ok(metadata) => metadata,
                Err(_) => continue, // Unable to access metadata, skip the file
            };

            // Check if the file is newer than the current newest file
            let modified_time = metadata.modified().ok()?;
            if newest_modified_time.is_none() || modified_time > newest_modified_time.unwrap() {
                newest_file = Some(path.clone());
                newest_modified_time = Some(modified_time);
            }
        }
    }

    newest_file
}

pub fn open_predicer() {
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

pub fn _read_file(path: &PathBuf) -> Result<f32, ()> {
    // Read a file
    let book = reader::xlsx::read(path).unwrap();
    let a1_value = book.get_sheet_by_name("npe").unwrap().get_value("C2");
    println!("{}", a1_value);

    let result: Result<f32, _> = a1_value.parse::<f32>();

    match result {
        Ok(num) => {
            println!("Parsed floating-point: {}", num);
            // Now you can use the parsed floating-point number in your code
            Ok(num) // Return the parsed floating-point number
        }

        Err(_) => {
            println!("Failed to parse the string as a floating-point number");
            Err(()) // Return an error value
        }
    }
}


pub fn _write_file(path: &PathBuf) {
    //read a file
    let mut book = reader::xlsx::read(path).unwrap();
    book.get_sheet_by_name_mut("Sheet1").unwrap().get_cell_mut("A1").set_value("TEST1");
    let _ = writer::xlsx::write(&book, path);
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
