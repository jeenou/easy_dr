
use std::sync::mpsc::{Sender};
mod main_loop;
mod utilities;
mod julia_interface;
mod model_builder;
use jlrs::prelude::*;
use std::path::PathBuf;

pub fn start_sending(tx: Sender<main_loop::_Task>) {
    
    tx.send(main_loop::_Task::StartProcess).unwrap();
    tx.send(main_loop::_Task::QuitProcess).unwrap();
    tx.send(main_loop::_Task::StartProcess).unwrap();
}


fn main() {

    //model_builder::_node();

    // Julia must be initialized before it can be used.
    // This is safe because this we're not initializing Julia from another
    // thread and crate at the same time.
    let mut frame = StackFrame::new(); 
    let mut pending = unsafe { RuntimeBuilder::new().start().expect("Could not init Julia") };
    let mut julia = pending.instance(&mut frame);

    // Include some custom code defined in MyModule.jl.
    // This is safe because the included code doesn't do any strange things.
    unsafe {
        let path = PathBuf::from("structures.jl");
        //let _data = utilities::_generate_data();
        if path.exists() {
            julia.include(path).expect("Could not include file");
        } else {
            julia
                .include("src/Predicer/src/structures.jl")
                .expect("Could not include file");
        }
    }

    // An extended target provides a target for the result we want to return and a frame for
    // temporary data.
    let _x = julia.scope(|mut frame| {
        //let data = julia_interface::_to_ordered_dict(frame.as_extended_target(), &utilities::_generate_data()).unwrap();
        let data1 = Value::new(&mut frame, 4isize); 
        let data2 = Value::new(&mut frame, 4isize); //JuliaString, data->managed->string
        let data3 = Value::new(&mut frame, 4isize); 
        

            
            let module = "Structures"; //B
            let function = "Test"; //B
            let _y = model_builder::_call_julia_function(frame, module, function, data1, data2, data3);
            Ok(())


    }).expect("result is an error"); 

    //println!("{}", x);

    //system_data
    //timeseries_data
    //timeseries_data["scenarios"]
    //processes, string, Predicer.Process
    //nodes, String, Predicer.Node
    //markets, String, Predicer.Market
    //scenarios, String, Float64
    //reserve_type, String, Float64
    //risk, String, Float64
    //gen_constraints, String, Predicer.GenConstraint

    //nämä liittyy main_looppiin:
    //let (tx, rx) = channel();
    //start_sending(tx);
    //main_loop::task_loop(rx);

    //main_loop::_read_file(&path);
    //main_loop::_write_file(&path, data)

    //let data = main_loop::generate_data(5, 8);
    //println!("{:?}", data);
    /*
    let values: Vec<Vec<String>> = vec![
        vec!["otsikko1", "otsikko2"].iter().map(|&s| s.to_string()).collect(),
        vec!["data1", "data2"].iter().map(|&s| s.to_string()).collect(),
    ];
    */
    
    //main_loop::_write_file_vector2(values);

    //let devices = utilities::_read_devices();
    //println!("{:?}", devices);
    /*
    let mut parameter_map = HashMap::new();
    parameter_map.insert("wind turbine", "windturb");
    parameter_map.insert("natural gas", "ngchp");
    parameter_map.insert("parameter3", "value3");
    */

    
}