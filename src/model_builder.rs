
use jlrs::prelude::*;
use std::path::PathBuf;
use jlrs::memory::target::frame::GcFrame;
use jlrs::error::JlrsError;

// Calls a Julia function with the specified module name, function name, and data argument.
// Returns the result of the function as an i64 integer, or an error if the call fails.
pub fn _call_julia_function<'a>(mut frame: GcFrame<'a>, module: &str, function: &str, data1: Value<'a, 'a>, data2: Value<'a, 'a>, data3: Value<'a, 'a>) -> Result<jlrs::prelude::Value<'a, 'a>, std::boxed::Box<JlrsError>> 
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

pub fn _node<T>() {
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
        if path.exists() {
            julia.include(path).expect("Could not include file");
        } else {
            julia
                .include("src/Predicer/src/structures.jl")
                .expect("Could not include file");
        }
    }

    
}

 //let data = julia_interface::_to_ordered_dict(frame.as_extended_target(), &utilities::_generate_data()).unwrap();