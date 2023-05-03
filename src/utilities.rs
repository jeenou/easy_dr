use std::io;
use std::fs;
use std::error::Error;
use std::path::PathBuf;
use std::io::BufReader;
use umya_spreadsheet::*;
use std::collections::HashMap;
//use indexmap::IndexMap;
use std::fs::File;

pub type Result<T, E> = std::result::Result<T, E>;

// This function takes three arguments:
//   - bool_val: a boolean value to insert into the vector
//   - int_val: an integer value to insert into the vector
//   - vec: a mutable reference to a vector of vectors of tuples
pub fn _insert_into_vector(bool_val: bool, int_val: i32, vec: &mut Vec<Vec<(bool, i32)>>) {
    // Get the current row index by getting the length of the vector.
    let row = vec.len();

    // If the vector is empty or the last row is already full (i.e., has length >= 2),
    // add a new row to the vector.
    if row == 0 || vec[row - 1].len() >= 2 {
        vec.push(Vec::new());
    }

    // Push the new tuple (bool_val, int_val) into the last row of the vector.
    vec[row - 1].push((bool_val, int_val));
}

// This function takes a single argument:
//   - filename: a string slice containing the name of the file to read
// The function reads the contents of the file into a vector of strings and returns it.

pub fn _read_file_to_vector(filename: &str) -> Vec<String> {

    let contents = fs::read_to_string(filename)
        .expect("Error reading file");

    let lines = contents.split("\n")
        .map(|s| s.to_string())
        .collect();

    lines
}

// This function takes a single argument:
//   - path: a mutable reference to a `PathBuf` object representing the path to a file to read
// The function reads an Excel file at the given path, modifies its contents, and writes it back to the same file.
pub fn _read_file(path: &PathBuf) {

    let mut book = reader::xlsx::read(path).unwrap();

    // Modify the contents of the file by setting the value of cell A1 in the first sheet to "TEST1".

    book.get_sheet_by_name_mut("Sheet1").unwrap().get_cell_mut("A1").set_value("TEST1");

    let _ = writer::xlsx::write(&book, path);
}

pub fn _write_file(_path: &PathBuf, _data: Vec<String>) {
    let mut book = new_file();

    // new worksheet
    let _ = book.new_sheet("timeseries");

    for (row, value) in _data.iter().enumerate() {
        book.get_sheet_mut(&1).unwrap().get_cell_by_column_and_row_mut(&1, &(row as u32)).set_value(value);
    }

    let _ = writer::xlsx::write(&book, _path);
}

pub fn _read_devices() -> Vec<String> {
    let mut devices = Vec::new();
    let mut input = String::new();

    println!("Enter a list of device names (one per line).");
    println!("When finished, enter an empty line:");

    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        if input.trim().is_empty() {
            break;
        }

        devices.push(input.trim().to_string());
    }

    devices
}

fn _create_2d_vector(devices: Vec<String>, parameters: Vec<String>) -> Vec<Vec<String>> {
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


pub fn _csv_to_hashmap(file_path: PathBuf) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(reader);

    let mut map = HashMap::new();
    for result in csv_reader.records() {
        let record = result?;
        if let (Some(key), Some(value)) = (record.get(0), record.get(1)) {
            map.insert(key.to_string(), value.to_string());
        }
    }

    Ok(map)
}



/*
pub fn get_processes(parameter_map: &HashMap<&str, &str>, process: &str) -> Vec<&str> {
    let mut parameter2_vec = Vec::new();
    for (key, value) in parameter_map {
        let question = format!("Does the system contain {}? Answer yes/no", key);
        println!("{}", question);
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Failed to read line");

        if answer.trim().to_lowercase() == "yes" && process.contains(key) {
            parameter2_vec.push(value);
        }
    }
    parameter2_vec
}
*/






