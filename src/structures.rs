

pub mod data {

    use jlrs::{prelude::*};
    use std::{path::PathBuf};
    use jlrs::memory::target::frame::GcFrame;
    //use jlrs::data::managed::value::ValueResult;
    use crate::juliainterface::julia;
    use jlrs::error::JlrsError;
    use std::collections::HashMap;

    pub struct Process<'a> {
        pub name: String,
        pub is_cf: bool,
        pub is_cf_fix: bool,
        pub is_online: bool,
        pub is_res: bool,
        pub conversion: i64, //1,2 tai 3
        pub eff: f64,
        pub load_min: f64,
        pub load_max: f64,
        pub start_cost: f64,
        pub min_online: f64,
        pub min_offline: f64,
        pub max_online: f64,
        pub max_offline: f64,
        pub initial_state: f64,
        pub delay: f64,
        pub eff_ops: &'a Vec<String>,
    }

    impl<'a> std::fmt::Debug for Process<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // Write your custom formatting logic here
            write!(f, "Process {{ /* ... */ }}")
        }
    }

    pub struct Node<'a> {
        pub name: String,
        pub is_commodity: bool,
        pub is_state: bool,
        pub is_res: bool,
        pub is_market: bool,
        pub is_inflow: bool,
        pub cost: &'a TimeSeriesData,
        pub inflow: &'a TimeSeriesData,
        
    }

    impl<'a> std::fmt::Debug for Node<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // Write your custom formatting logic here
            write!(f, "Node {{ /* ... */ }}")
        }
    }

    pub struct Market {
        pub name: String,
        pub m_type: String,
        pub node: String, //mikä tyyppi
        pub pgroup: String,
        pub direction: String,
        pub realisation: f64,
        pub reserve_type: String,
        pub is_bid: bool,
        pub is_limited: bool,
        pub min_bid: f64,
        pub max_bid: f64,
        pub fee: f64,
    }

    pub struct Group {
        pub name: String,
        pub g_type: String,
        pub entity: String,
    }

    impl<'a> std::fmt::Debug for Market {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // Write your custom formatting logic here
            write!(f, "Node {{ /* ... */ }}")
        }
    }
    
    pub struct Scenario {
        pub name: String,
        pub probability: f64,
    }
    
    
    pub struct Topology {
        pub source: bool, 
        pub sink: bool, 
        pub name: String,
        pub capacity: f64,
        pub vom_cost: f64,
        pub ramp_up: f64,
        pub ramp_down: f64,
    }
    
    pub struct State {
        pub in_max: f64,
        pub out_max: f64,
        pub state_loss_proportional: f64,
        pub state_max: f64,
        pub state_min: f64,
        pub initial_state: f64,
        pub residual_value: f64,
    }
    
    pub struct TimeSeries {
        pub scenario: String,
        pub series: Vec<(String, String)>,
    }
    
    pub struct TimeSeriesData {
        pub ts_data: Vec<TimeSeries>,
    }

    
    pub struct ConFactor {
        pub flow: (String, String),
        pub data: TimeSeriesData,
    }
    
    pub struct GenConstraint {
        pub name: String,
        pub g_type: String,
        pub factors: Vec<ConFactor>,
        pub constant: TimeSeriesData,
        
    }
    
    /*
    processes::OrderedDict{String, Process}
    nodes::OrderedDict{String, Node}
    markets::OrderedDict{String, Market}
    scenarios::OrderedDict{String, Float64}
    reserve_type::OrderedDict{String, Float64}
    risk::OrderedDict{String, Float64}
    gen_constraints::OrderedDict{String, GenConstraint}
    */

    pub fn _predicer(nodes: HashMap<&String, &Node>, processes: HashMap<&String, &Process>, markets: HashMap<&String, &Market>, groups: HashMap<&String, &Group>) {
        let mut frame = StackFrame::new();
        let mut pending = unsafe { RuntimeBuilder::new().start().expect("Could not init Julia") };
        let mut julia = pending.instance(&mut frame);
        // Include some custom code defined in MyModule.jl.
        // This is safe because the included code doesn't do any strange things.
        unsafe {
            julia.scope(|mut frame| {
                let predicer_dir = JuliaString::new(&mut frame, "C:\\Users\\enessi\\Documents\\easy_dr\\Predicer").as_value();
                let _ = Module::main(&frame)
                    .function(&frame, "cd")?
                    .as_managed()
                    .call1(&mut frame, predicer_dir).expect("cd to Predicer dir failed");
                Ok(())
            }).expect("error when cding to Predicer dir");
            julia.scope(|mut frame| {
                let predicer_dir = JuliaString::new(&mut frame, "C:\\Users\\enessi\\Documents\\easy_dr\\Predicer").as_value();
                let _ = Value::eval_string(&mut frame, "using Pkg");
                let _ = Module::main(&frame)
                    .submodule(&frame, "Pkg")?
                    .as_managed()
                    .function(&frame, "activate")?
                    .as_managed()
                    .call1(&mut frame, predicer_dir).expect("activation failed");
                Ok(())
            }).expect("error when activating Julia environment");
            julia.scope(|mut frame| {
                Module::main(&frame)
                    .submodule(&frame, "Pkg")?
                    .as_managed()
                    .function(&frame, "instantiate")?
                    .as_managed()
                    .call0(&mut frame).expect("instatiation failed");
                    Ok(())
            }).expect("error when instantiating Julia environment");
            julia.scope(|mut frame| {
                let wd = Module::main(&frame)
                    .function(&frame, "pwd")?
                    .as_managed()
                    .call0(&mut frame).into_jlrs_result()?.unbox::<String>().expect("pwd error");
                println!("working directory {}", wd.expect("not ok"));
                Ok(())
            }).expect("error error on the wall");
            let path = PathBuf::from("src/structures.jl");
            julia.include(path).expect("Could not include file1");
        }

        //Create processes

        for (key, value) in &processes {

            // An extended target provides a target for the result we want to return and a frame for
            // temporary data.
            let _x = julia.scope(|mut frame| {

                //name::String, conversion::Int=1, delay::Float64=0.0

                let d1 = JuliaString::new(&mut frame, key).as_value();
                let d2 = Value::new(&mut frame, value.conversion); 
                let d3 = Value::new(&mut frame, value.delay);
  
                        
                let function = "create_process"; 
                let _process = julia::_call3(&mut frame, function,d1, d2, d3).unwrap().into_jlrs_result();

                match _process {
                    Ok(value) => {
                        let _convert_function = "add_to_processes";
                        let _convert_result = julia::_call1(&mut frame, _convert_function, value).unwrap();
                        match _convert_result {
                            Ok(_) => println!("Added process to processes"),
                            Err(error) => println!("Error adding process to processes: {:?}", error),
                        }
                    }
                    Err(error) => println!("Error adding process to processes2: {:?}", error),
                }  
                
                Ok(())            
            
            }).expect("result is an error");
            
        }

        //Create nodes

        for (key, value) in &nodes {

            // An extended target provides a target for the result we want to return and a frame for
            // temporary data.
            let _x = julia.scope(|mut frame| {

                let d1 = JuliaString::new(&mut frame, key).as_value();
                let d2 = Value::new(&mut frame, value.is_commodity); 
                let d3 = Value::new(&mut frame, value.is_market);
  
                        
                let function = "create_node"; 
                let _node = julia::_call3(&mut frame, function,d1, d2, d3).unwrap().into_jlrs_result();

                match _node {
                    Ok(value) => {
                        let _convert_function = "add_to_nodes";
                        let _convert_result = julia::_call1(&mut frame, _convert_function, value).unwrap();
                        match _convert_result {
                            Ok(_) => println!("Added node to nodes"),
                            Err(error) => println!("Error adding node to nodes: {:?}", error),
                        }
                    }
                    Err(error) => println!("Error adding node to nodes2: {:?}", error),
                }  
                
                Ok(())            
            
            }).expect("result is an error");
            
        }

        //Creating markets

        //name::String, type::String, node::Any, pgroup::Any, direction::String, reserve_type::String, is_bid::Bool, is_limited::Bool, min_bid::Float64, max_bid::Float64, fee::Float64

        for (key, value) in &markets {

            // An extended target provides a target for the result we want to return and a frame for
            // temporary data.
            let _x = julia.scope(|mut frame| {

                let d1 = JuliaString::new(&mut frame, key).as_value();
                let d2 = JuliaString::new(&mut frame, &value.m_type).as_value(); 
                let d3 = JuliaString::new(&mut frame, &value.node).as_value();
                let d4 = JuliaString::new(&mut frame, &value.pgroup).as_value();
                let d5 = JuliaString::new(&mut frame, &value.direction).as_value();
                let d6 = JuliaString::new(&mut frame, &value.reserve_type).as_value();
                let d7 = Value::new(&mut frame, value.is_bid);
                let d8 = Value::new(&mut frame, value.is_limited);
                let d9 = Value::new(&mut frame, value.min_bid);
                let d10 = Value::new(&mut frame, value.max_bid);
                let d11 = Value::new(&mut frame, value.fee);

                let args = [d1,d2,d3,d4,d5,d6,d7,d8,d9,d10,d11];
  
                        
                let function = "create_market"; 
                let _market = julia::_call(&mut frame, function, &args).unwrap().into_jlrs_result();

                match _market {
                    Ok(value) => {
                        let _convert_function = "add_to_markets";
                        let _convert_result = julia::_call1(&mut frame, _convert_function, value).unwrap();
                        match _convert_result {
                            Ok(_) => println!("Added market to markets"),
                            Err(error) => println!("Error adding market to markets: {:?}", error),
                        }
                    }
                    Err(error) => println!("Error adding market to markets2: {:?}", error),
                }  
                
                Ok(())            
            
            }).expect("result is an error");

        }

            //Creating groups

        for (key, value) in &groups {

            // An extended target provides a target for the result we want to return and a frame for
            // temporary data.
            let _x = julia.scope(|mut frame| {

                let d1 = JuliaString::new(&mut frame, key).as_value();
                let d2 = JuliaString::new(&mut frame, &value.g_type).as_value(); 
                let d3 = JuliaString::new(&mut frame, &value.entity).as_value(); 

                let args = [d1,d2,d3];

                        
                let function = "create_group"; 
                let _group = julia::_call(&mut frame, function, &args).unwrap().into_jlrs_result();

                match _group {
                    Ok(value) => {
                        let _convert_function = "add_to_groups";
                        let _convert_result = julia::_call1(&mut frame, _convert_function, value).unwrap();
                        match _convert_result {
                            Ok(_) => println!("Added group to groups"),
                            Err(error) => println!("Error adding group to groups: {:?}", error),
                        }
                    }
                    Err(error) => println!("Error adding group to groups2: {:?}", error),
                }  
                
                Ok(())            
            
            }).expect("result is an error");
        
        }
        
    }
    
    pub fn _ordered_dict(data: Vec<(String, f64)>) {
        let mut frame = StackFrame::new();
        let mut pending = unsafe { RuntimeBuilder::new().start().expect("Could not init Julia") };
        let mut julia = pending.instance(&mut frame);
        // Include some custom code defined in MyModule.jl.
        // This is safe because the included code doesn't do any strange things.
        unsafe {
            julia.scope(|mut frame| {
                let predicer_dir = JuliaString::new(&mut frame, "C:\\Users\\enessi\\Documents\\easy_dr\\Predicer").as_value();
                let _ = Module::main(&frame)
                    .function(&frame, "cd")?
                    .as_managed()
                    .call1(&mut frame, predicer_dir).expect("cd to Predicer dir failed");
                Ok(())
            }).expect("error when cding to Predicer dir");
            julia.scope(|mut frame| {
                let predicer_dir = JuliaString::new(&mut frame, "C:\\Users\\enessi\\Documents\\easy_dr\\Predicer").as_value();
                let _ = Value::eval_string(&mut frame, "using Pkg");
                let _ = Module::main(&frame)
                    .submodule(&frame, "Pkg")?
                    .as_managed()
                    .function(&frame, "activate")?
                    .as_managed()
                    .call1(&mut frame, predicer_dir).expect("activation failed");
                Ok(())
            }).expect("error when activating Julia environment");
            julia.scope(|mut frame| {
                Module::main(&frame)
                    .submodule(&frame, "Pkg")?
                    .as_managed()
                    .function(&frame, "instantiate")?
                    .as_managed()
                    .call0(&mut frame).expect("instatiation failed");
                    Ok(())
            }).expect("error when instantiating Julia environment");
            julia.scope(|mut frame| {
                let wd = Module::main(&frame)
                    .function(&frame, "pwd")?
                    .as_managed()
                    .call0(&mut frame).into_jlrs_result()?.unbox::<String>().expect("pwd error");
                println!("working directory {}", wd.expect("not ok"));
                Ok(())
            }).expect("error error on the wall");
            let path = PathBuf::from("src/structures.jl");
            julia.include(path).expect("Could not include file1");
        }
        
        // An extended target provides a target for the result we want to return and a frame for
        // temporary data.
        let _x = julia.scope(|mut frame| {

    
            let _result2 = julia::_to_ordered_dict(frame.as_extended_target(), &data).unwrap();

            let function2 = "print_ordered_dict";
            let _result3 = julia::_call1(&mut frame, function2, _result2).unwrap().into_jlrs_result();
   
            Ok(())    
            
        }).expect("result is an error");
        
    } 
    
    pub fn _test(da1: i64, da2: i64, da3: i64, da4: i64, data: Vec<(String, f64)>) {
        let mut frame = StackFrame::new();
        let mut pending = unsafe { RuntimeBuilder::new().start().expect("Could not init Julia") };
        let mut julia = pending.instance(&mut frame);
        // Include some custom code defined in MyModule.jl.
        // This is safe because the included code doesn't do any strange things.
        unsafe {
            julia.scope(|mut frame| {
                let predicer_dir = JuliaString::new(&mut frame, "C:\\Users\\enessi\\Documents\\easy_dr\\Predicer").as_value();
                let _ = Module::main(&frame)
                    .function(&frame, "cd")?
                    .as_managed()
                    .call1(&mut frame, predicer_dir).expect("cd to Predicer dir failed");
                Ok(())
            }).expect("error when cding to Predicer dir");
            julia.scope(|mut frame| {
                let predicer_dir = JuliaString::new(&mut frame, "C:\\Users\\enessi\\Documents\\easy_dr\\Predicer").as_value();
                let _ = Value::eval_string(&mut frame, "using Pkg");
                let _ = Module::main(&frame)
                    .submodule(&frame, "Pkg")?
                    .as_managed()
                    .function(&frame, "activate")?
                    .as_managed()
                    .call1(&mut frame, predicer_dir).expect("activation failed");
                Ok(())
            }).expect("error when activating Julia environment");
            julia.scope(|mut frame| {
                Module::main(&frame)
                    .submodule(&frame, "Pkg")?
                    .as_managed()
                    .function(&frame, "instantiate")?
                    .as_managed()
                    .call0(&mut frame).expect("instatiation failed");
                    Ok(())
            }).expect("error when instantiating Julia environment");
            julia.scope(|mut frame| {
                let wd = Module::main(&frame)
                    .function(&frame, "pwd")?
                    .as_managed()
                    .call0(&mut frame).into_jlrs_result()?.unbox::<String>().expect("pwd error");
                println!("working directory {}", wd.expect("not ok"));
                Ok(())
            }).expect("error error on the wall");
            let path = PathBuf::from("src/structures.jl");
            julia.include(path).expect("Could not include file1");
        }
        
        // An extended target provides a target for the result we want to return and a frame for
        // temporary data.
        let _x = julia.scope(|mut frame| {

        
            let d1 = Value::new(&mut frame, da1);
            let d2 = Value::new(&mut frame, da2); 
            let d3 = Value::new(&mut frame, da3); 
            let d4 = Value::new(&mut frame, da4);  
                      
            //let module = "Predicer";
            let function = "print_message"; 
            let _result = julia::_call4(&mut frame, function,d1, d2, d3, d4).unwrap().into_jlrs_result();

            let _result2 = julia::_to_ordered_dict(frame.as_extended_target(), &data).unwrap();

            let function2 = "print_ordered_dict";
            let _result3 = julia::_call1(&mut frame, function2, _result2).unwrap().into_jlrs_result();
            
            Ok(())    
        
    
            
        }).expect("result is an error");
        
    }
    

}



