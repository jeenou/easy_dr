
use std::sync::mpsc::{Sender};
use std::path::{PathBuf};
mod main_loop;

pub fn start_sending(tx: Sender<main_loop::_Task>) {
    
    tx.send(main_loop::_Task::StartProcess).unwrap();
    tx.send(main_loop::_Task::QuitProcess).unwrap();
    tx.send(main_loop::_Task::StartProcess).unwrap();
}

fn main() {
    //let (tx, rx) = channel();
    //start_sending(tx);
    //main_loop::task_loop(rx);

    let mut path = PathBuf::from("src");
    path.push("Predicer/input_data/input_data_6.xlsx");

    //tee datavektori, missä tarvittavat tiedot
    //laheta data write filulle

    //main_loop::_read_file(&path);
    //main_loop::_write_file(&path, data)

    //let data = main_loop::generate_data(5, 8);
    //println!("{:?}", data);

    let values: Vec<Vec<String>> = vec![
        vec!["otsikko1", "otsikko2"].iter().map(|&s| s.to_string()).collect(),
        vec!["data1", "data2"].iter().map(|&s| s.to_string()).collect(),
    ];
    
    main_loop::_write_file_vector2(values);

}