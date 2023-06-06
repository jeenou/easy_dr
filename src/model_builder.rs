mod julia_interface;
use jlrs::prelude::*;
use std::path::PathBuf;

pub fn _form_julia_frame() {
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
        let _data = utilities::_generate_data();
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
    let x = julia.scope(|mut frame| {
        let data = julia_interface::_to_ordered_dict(frame.as_extended_target(), &utilities::_generate_data()).unwrap();

            
            let module = "MyModule";
            let function = "print_ordered_dict";
            julia_interface::_call_julia_function(frame, module, function, data)


    }).expect("result is an error"); 

    println!("{}", x);
}