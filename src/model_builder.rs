
use jlrs::prelude::*;
use std::path::PathBuf;
use jlrs::memory::target::frame::GcFrame;
use jlrs::error::JlrsError;

// Calls a Julia function with the specified module name, function name, and data argument.
// Returns the result of the function as an i64 integer, or an error if the call fails.
pub fn _call_julia_function<'scope, 'a>(mut frame: GcFrame<'scope>, module: &str, function: &str, data1: Value<'scope, 'a>, data2: Value<'scope, 'a>, data3: Value<'scope, 'a>) -> Result<jlrs::prelude::Value<'scope, 'a>, std::boxed::Box<JlrsError>>
{
    unsafe {
        Module::main(&frame)
            .submodule(&frame, module)?
            .as_managed()
            .function(&frame, function)?
            .as_managed()
            //geneerinen call on olemassa myös
            .call3(&mut frame, data1, data2, data3)
            .into_jlrs_result()
            
            //result is not always i64
            //result would be Julia struct in some cases, how can we handle it? 
            //how we can use the result of the julia function in another julia function?
    }
}

pub fn _test<T>() {
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
            let _y = _call_julia_function(frame, module, function, data1, data2, data3);
            Ok(())


    }).expect("result is an error"); 
}

pub fn _node<T>() {
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
            let _y = _call_julia_function(frame, module, function, data1, data2, data3);
            Ok(())


    }).expect("result is an error"); 
}

 //let data = julia_interface::_to_ordered_dict(frame.as_extended_target(), &utilities::_generate_data()).unwrap();