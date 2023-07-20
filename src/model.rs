
use jlrs::prelude::*;
use std::path::PathBuf;

mod julia_interface {
    use crate::julia_interface::_call1;
    use crate::julia_interface::_call2;
    use crate::julia_interface::_call3;
}


//NODE EXAMPLES

pub fn _node<T>(name: &str, is_commodity: bool, is_market: bool) {
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

    // An extended target provides a target for the result we want to return and a frame for
    // temporary data.
    let _x = julia.scope(|mut frame| {

        let n_name = JuliaString::new(&mut frame, name).as_value(); 
        let n_commodity = Value::new(&mut frame, is_commodity); 
        let n_market = Value::new(&mut frame, is_market); 
        

            
        let module = "Structures"; 
        let function = "create_node"; 
        let _result_node = _call3(&mut frame, module, function, n_name, n_commodity, n_market);
        Ok(()) 
        
    }).expect("result is an error");

}


pub fn _node2(name: &str, is_commodity: bool, is_market: bool) {
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
    
    // An extended target provides a target for the result we want to return and a frame for
    // temporary data.
    let _x = julia.scope(|mut frame| {
    
        let n_name = JuliaString::new(&mut frame, name).as_value(); 
        let n_commodity = Value::new(&mut frame, is_commodity); 
        let n_market = Value::new(&mut frame, is_market); 
        
    
            
        let module = "Structures"; 
        let function = "create_node"; 
        let _result_node = _call3(&mut frame, module, function, n_name, n_commodity, n_market).unwrap();
        
    
        //Convert node to commodity if is_commodity is true

    
        if is_commodity {
            match _result_node {
                Ok(value) => {
                    let _convert_function = "convert_to_commodity";
                    let _convert_result = _call1(&mut frame, module, _convert_function, value).unwrap();
                    match _convert_result {
                        Ok(_) => println!("Node converted to commodity"),
                        Err(error) => println!("Error converting node to commodity: {:?}", error),
                    }
                }
                Err(error) => println!("Error creating node: {:?}", error),
            }
        }
    
         Ok(())
        
    }).expect("result is an error");
    

}

