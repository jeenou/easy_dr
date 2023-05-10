
use std::sync::mpsc::{Sender};
mod main_loop;
mod utilities;
use jlrs::prelude::*;
use std::path::PathBuf;

pub fn start_sending(tx: Sender<main_loop::_Task>) {
    
    tx.send(main_loop::_Task::StartProcess).unwrap();
    tx.send(main_loop::_Task::QuitProcess).unwrap();
    tx.send(main_loop::_Task::StartProcess).unwrap();
}

fn main() {

    // Julia must be initialized before it can be used.
    // This is safe because this we're not initializing Julia from another
    // thread and crate at the same time.
    let mut frame = StackFrame::new();
    let mut pending = unsafe { RuntimeBuilder::new().start().expect("Could not init Julia") };
    let mut julia = pending.instance(&mut frame);

    // Include some custom code defined in MyModule.jl.
    // This is safe because the included code doesn't do any strange things.
    unsafe {
        let path = PathBuf::from("MyModule.jl");
        let data = utilities::_generate_data();
        if path.exists() {
            julia.include(path).expect("Could not include file");
        } else {
            julia
                .include("src/Predicer/src/MyModule.jl")
                .expect("Could not include file");
        }
    }

    // An extended target provides a target for the result we want to return and a frame for
    // temporary data.
    let (target, frame) = target.split();
    frame.scope(|mut frame| {
        // Get OrderedDict, load OrderedCollections if it can't be found. An error is returned if
        // OrderedCollections hasn't been installed yet.
        // OrderedDict is a UnionAll because it has type parameters that must be set
        let ordered_dict = Module::main(&frame).global(&mut frame, "OrderedDict");
        let ordered_dict_ua = match ordered_dict {
            Ok(ordered_dict) => ordered_dict,
            Err(_) => {
                // Safety: using this package is fine.
                unsafe {
                    Value::eval_string(&mut frame, "using OrderedCollections")
                        .into_jlrs_result()?
                };
                Module::main(&frame).global(&mut frame, "OrderedDict")?
            }
        }
        .cast::<UnionAll>()?;
        // The key and value type.
        let types = [
            DataType::string_type(&frame).as_value(),
            DataType::int32_type(&frame).as_value(),
        ];
        // Apply the types to the OrderedDict UnionAll to create the OrderedDict{String, Int32}
        // DataType, and call its constructor.
        //
        // Safety: the types are correct and the constructor doesn't access any data that might
        // be in use.
        let ordered_dict = unsafe {
            let ordered_dict_ty = ordered_dict_ua
                .apply_types(&mut frame, types)
                .into_jlrs_result()?;
            ordered_dict_ty.call0(&mut frame).into_jlrs_result()?
        };
        let setindex_fn = Module::base(&target).function(&mut frame, "setindex!")?;
        for (key, value) in data {
            // Create the keys and values in temporary scopes to avoid rooting an arbitrarily
            // large number of pairs in the current frame.
            frame.scope(|mut frame| {
                let key = JuliaString::new(&mut frame, key).as_value();
                let value = Value::new(&mut frame, *value);
                // Safety: the ordered dict can only be used in this function until it is
                // returned, setindex! is a safe function.
                unsafe {
                    setindex_fn
                        .call3(&mut frame, ordered_dict, value, key)
                        .into_jlrs_result()?;
                }
                Ok(())
            })?;
        }
        Ok(ordered_dict.root(target))
    })

    // Do something with the returned ordered dictionary...

    //let (tx, rx) = channel();
    //start_sending(tx);
    //main_loop::task_loop(rx);

    //tee datavektori, missä tarvittavat tiedot
    //laheta data write filulle

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

    // Julia must be initialized before it can be used.
    // This is safe because this we're not initializing Julia from another
    // thread and crate at the same time.
    // Julia must be initialized before it can be used.
    // This is safe because this we're not initializing Julia from another
    // thread and crate at the same time.

    /*
    Calling julia function
    
    let mut frame = StackFrame::new();
    let mut pending = unsafe { RuntimeBuilder::new().start().expect("Could not init Julia") };
    let mut julia = pending.instance(&mut frame);
    let mut fpath = "input_data/input_data.xlsx";

    // Include some custom code defined in MyModule.jl.
    // This is safe because the included code doesn't do any strange things.
    unsafe {
        let path = PathBuf::from("MyModule.jl");
        if path.exists() {
            julia.include(path).expect("Could not include file");
        } else {
            julia
                .include("src/Predicer/src/MyModule.jl")
                .expect("Could not include file");
        }
    }

    // Create a scope, the closure provided to this method can use a `GcFrame` to ensure Julia
    // data is not cleaned up by the GC while it's in use.
    let result = julia
        .scope(|mut frame| {
            let dim = Value::new(&mut frame, 4isize);
            let iters = Value::new(&mut frame, 1_000_000isize);

            unsafe {
                Module::main(&frame)
                    // the submodule doesn't have to be rooted because it's never reloaded.
                    .submodule(&frame, "MyModule")?
                    .as_managed()
                    // the same holds true for the function: the module is never reloaded so it's
                    // globally rooted
                    .function(&frame, "complexfunc")?
                    .as_managed()
                    // Call the function with the two arguments it takes
                    .call2(&mut frame, dim, iters)
                    // If you don't want to use the exception, it can be converted to a `JlrsError`
                    // In this case the error message will contain the message that calling
                    // `display` in Julia would show
                    .into_jlrs_result()?
                    // The function that was called returns a `Float64`, which can be unboxed as `f64`
                    .unbox::<f64>()
            }
        })
        .expect("Result is an error");

    println!("Result: {}", result);

    */
}