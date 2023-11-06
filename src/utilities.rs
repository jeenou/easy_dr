

// Function to read and print the contents of a directory in the server.
fn _print_directory_contents(dir_path: &str) {
    match fs::read_dir(dir_path) {
        Ok(entries) => {
            println!("Contents of directory '{}':", dir_path);
            for entry in entries {
                match entry {
                    Ok(dir_entry) => {
                        // Get the file name as a string
                        if let Some(file_name) = dir_entry.file_name().to_str() {
                            // Check if it's a file or a directory
                            if dir_entry.file_type().map_or(false, |ft| ft.is_dir()) {
                                println!("Directory: {}", file_name);
                            } else {
                                println!("File: {}", file_name);
                            }
                        }
                    }
                    Err(err) => eprintln!("Error reading directory entry: {}", err),
                }
            }
        }
        Err(err) => eprintln!("Error reading directory: {}", err),
    }
}

fn _print_vector<T: std::fmt::Display>(vec: &Vec<T>) {
    for item in vec.iter() {
        println!("{}", item);
    }
}

fn _print_tuple_vector(v: &Vec<(String, f64)>) {
    for (name, value) in v {
        println!("{}: {}", name, value);
    }
}