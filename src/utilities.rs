use std::io;

pub fn _insert_into_vector(bool_val: bool, int_val: i32, vec: &mut Vec<Vec<(bool, i32)>>) {
    let row = vec.len();
    if row == 0 || vec[row - 1].len() >= 2 {
        vec.push(Vec::new());
    }
    vec[row - 1].push((bool_val, int_val));
}

pub fn _read_file_to_vector(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
        .expect("Error reading file");
    let lines = contents.split("\n")
        .map(|s| s.to_string())
        .collect();
    lines
}

pub fn _read_file(path: &PathBuf) {
    //read a file
    let mut book = reader::xlsx::read(path).unwrap();
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

pub fn read_devices() -> Vec<String> {
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

