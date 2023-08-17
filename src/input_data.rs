

pub mod functions {

    use crate::structures::data;
    use std::collections::HashMap;
    use jlrs::{prelude::*};
    use std::{path::PathBuf};
    //use jlrs::memory::target::frame::GcFrame;
    //use jlrs::data::managed::value::ValueResult;
    use crate::juliainterface::juliainterface;
    use jlrs::error::JlrsError;

    //Input data: contains_reserves
    //Boolean arvon voi vain muuttaa julia-booleaniksi ja lähettää lopulta input_data:n tekoon soveltuvalle funktiolle
    pub fn contains_reserves(nodes: &HashMap<String, data::Node>) -> bool {
        if nodes.is_empty() {
            // Perform desired action when the HashMap is empty
            // For example, return false or throw an error
            return false;
        }

        for (_, value) in nodes.iter() {
            if value.is_res {
                return true;
            }
        }

        false
    }

    //Input data: contains_online
    pub fn contains_online(processes: &HashMap<String, data::Process>) -> bool {
        if processes.is_empty() {
            // Perform desired action when the HashMap is empty
            // For example, return false or throw an error
            return false;
        }

        for (_, value) in processes.iter() {
            if value.is_online {
                return true;
            }
        }

        false
    }

    //Input data: contains_states
    pub fn contains_states(nodes: &HashMap<String, data::Node>) -> bool {
        if nodes.is_empty() {
            // Perform desired action when the HashMap is empty
            // For example, return false or throw an error
            return false;
        }

        for (_, value) in nodes.iter() {
            if value.is_state {
                return true;
            }
        }

        false
    }

    //Input data: contains_piecewise_eff
    pub fn contains_piecewise_eff(processes: &HashMap<String, data::Process>) -> bool {
        if processes.is_empty() {
            return true;
        }

        for (_, value) in processes.iter() {
            if !value.eff_ops.is_empty() {
                return false;
            }
        }

        true
    }

    /*
    Input data: contains_risk
    pub fn contains_risk() {

    }
    */

    //Input data: contains_delay
    pub fn contains_delay(processes: &HashMap<String, data::Process>) -> bool {
        if processes.is_empty() {
            return false;
        }

        for (_, value) in processes.iter() {
            if value.delay != 0.0 {
                return true;
            }
        }

        false
    }

    //Input data: contains_diffusion MUOKKAA TÄMÄ
    pub fn contains_diffusion(processes: &HashMap<String, data::Process>) -> bool {
        if processes.is_empty() {
            return false;
        }

        for (_, value) in processes.iter() {
            if value.delay != 0.0 {
                return true;
            }
        }

        false
    }

    //Processes

    pub fn processes(processes: &HashMap<String, data::Process>) {

        for (key, value) in processes {
            println!("Key: {}", key);
            println!("Value: {:?}", value); // Print or use the value as needed

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

                
                let name = JuliaString::new(&mut frame, &value.name).as_value();
                let conversion = Value::new(&mut frame, value.conversion); 
                let delay = Value::new(&mut frame, value.delay);   

                //Create new process in julia        
                let module = "Structures"; 
                let function = "create_process"; 
                let _process_result = juliainterface::_call3(&mut frame, module, function, name, conversion, delay).unwrap().into_jlrs_result();

                //Add process to ordered dict in julia
                match _process_result {
                    Ok(_process) => {
                        let _add_to_processes = "add_to_processes";
                        let _add_to_processes_result = 
                        juliainterface::_call1(&mut frame, module, _add_to_processes, _process).unwrap();
                    }
                    Err(error) => println!("Error creating process: {:?}", error),
                }

                Ok(())    
                    
            }).expect("result is an error");
        }
    
    }

//Nodes

pub fn nodes(nodes: &HashMap<String, data::Node>) {

    for (key, value) in nodes {
        println!("Key: {}", key);
        println!("Value: {:?}", value); // Print or use the value as needed

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

            
            let name = JuliaString::new(&mut frame, &value.name).as_value();
            let is_commodity = Value::new(&mut frame, value.is_commodity); 
            let is_market = Value::new(&mut frame, value.is_market);   

            //Create new process in julia        
            let module = "Structures"; 
            let function = "create_node"; 
            let _node_result = juliainterface::_call3(&mut frame, module, function, name, is_commodity, is_market).unwrap().into_jlrs_result();

            //Add process to ordered dict in julia
            match _node_result {
                Ok(_node) => {
                    let _add_to_nodes = "add_to_nodes";
                    let _add_to_nodes_result = 
                    juliainterface::_call1(&mut frame, module, _add_to_nodes, _node).unwrap();
                }
                Err(error) => println!("Error creating process: {:?}", error),
            }

            Ok(())    
                
        }).expect("result is an error");
    }

}

//Node diffusion tuples

//Markets

/* 

pub fn markets(markets: &HashMap<String, data::Market>) {

    for (key, value) in markets {
        println!("Key: {}", key);
        println!("Value: {:?}", value); // Print or use the value as needed

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

            
            let name = JuliaString::new(&mut frame, value.name).as_value();
            let is_commodity = Value::new(&mut frame, value.is_commodity); 
            let is_market = Value::new(&mut frame, value.is_market);   

            //Create new process in julia        
            let module = "Structures"; 
            let function = "create_node"; 
            let _node_result = juliainterface::_call3(&mut frame, module, function, name, is_commodity, is_market).unwrap().into_jlrs_result();

            //Add process to ordered dict in julia
            match _node_result {
                Ok(_node) => {
                    let _add_to_nodes = "add_to_nodes";
                    let _add_to_nodes_result = 
                    juliainterface::_call1(&mut frame, module, _add_to_nodes, _node).unwrap();
                }
                Err(error) => println!("Error creating process: {:?}", error),
            }

            Ok(())    
                
        }).expect("result is an error");
    }

    

}

*/

//Groups

//Scenarios

//Reserve-type

//Risk

//Inflow blocks

//Gen constraints


}
