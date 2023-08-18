

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
        pub direction: String,
        pub realisation: f64,
        pub reserve_type: String,
        pub is_bid: bool,
        pub price: TimeSeriesData,
        pub up_price: TimeSeriesData,
        pub down_price: TimeSeriesData,
        pub fixed: Vec<(String, String)>,
        
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
    
 
    
    pub fn _test(da1: i64, da2: i64, da3: i64, da4: i64, data: Vec<(String, i32)>) {
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
    
    
       /* 
    
    
    pub fn _julia_frame(node: Node, process: Process, scenario: Scenario, _sources: HashMap<&String, &Topology>, _sinks: HashMap<&String, &Topology>) {
        let mut frame = StackFrame::new();
        let mut pending = unsafe { RuntimeBuilder::new().start().expect("Could not init Julia") };
        let mut julia = pending.instance(&mut frame);
        
        
        //Included julia file structures
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
        
        // Creating julia scope
    
        let _x = julia.scope(|mut frame| {
    
            //Defining julia module
    
            let module = "Structures"; 
    
            //CREATING TIMESERIES
    
            //TimeSeries-muotoinen muuttuja säilötään _timeseries-muuttujaan
    
            let _scenario = JuliaString::new(&mut frame, scenario.name).as_value();
            let _create_timeseries = "create_timeseries";
    
            let _create_timeseries_value = 
            juliainterface::_call1(&mut frame, module, _create_timeseries, _scenario).unwrap();
            let _timeseries: Value<'_, '_>;
            match _create_timeseries_value {
                Ok(_value) => {
                    _timeseries = _value;
                    println!("Created TimeSeries")
                }
                Err(error) => println!("Error creating TimeSeries: {:?}", error),
            }
    
            //CREATING NODE
    
            let n_name = JuliaString::new(&mut frame, node.name).as_value();
            let n_commodity = Value::new(&mut frame, node.is_commodity); 
            let n_market = Value::new(&mut frame, node.is_market); 
    
                
            let _create_node = "create_node"; 
            let _create_node_result = 
            juliainterface::_call3(&mut frame, module, _create_node, n_name, n_commodity, n_market).unwrap();
        
            //Convert node to commodity if is_commodity is true
    
            if node.is_commodity {
                match _create_node_result {
                    Ok(_node) => {
                        let _convert_to_commodity = "convert_to_commodity";
                        let _convert_to_commodity_result = 
                        juliainterface::_call1(&mut frame, module, _convert_to_commodity, _node).unwrap();
    
                        //Add cost
    
                        match _convert_to_commodity_result {
                            Ok(result) => {
                                println!("Converted to commonity");
                                let _cost_function = "add_cost";
                                let _cost_function_result = 
                                juliainterface::_call2(&mut frame, module, _cost_function, result, _timeseries).unwrap();
                                println!("Added cost")
                            },
                            Err(error) => println!("Error while adding cost: {:?}", error),
                        }
                    }
                    Err(error) => println!("Error creating node: {:?}", error),
                }
            }
    
            //Convert node to market node if is_market is true
    
            else if node.is_market {
                match _create_node_result {
                    Ok(value) => {
                        let _convert_function = "convert_to_market";
                        let _convert_result = 
                        juliainterface::_call1(&mut frame, module, _convert_function, value).unwrap();
                        match _convert_result {
                            Ok(_) => println!("Node converted to market"),
                            Err(error) => println!("Error converting node to market: {:?}", error),
                        }
                    }
                    Err(error) => println!("Error converting to market: {:?}", error),
                }
            }
    
            //Convert node to inflow node if is_inflow is true
    
            else if node.is_inflow {
                match _create_node_result {
                    Ok(value) => {
                        let _convert_function = "add_inflow";
                        let _convert_result = 
                        juliainterface::_call2(&mut frame, module, _convert_function, value, _timeseries).unwrap();
                        match _convert_result {
                            Ok(_) => println!("Added inflow to node"),
                            Err(error) => println!("Error adding inflow to node: {:?}", error),
                        }
                    }
                    Err(error) => println!("Error adding inflow: {:?}", error),
                }
            }
    
            //Convert node to state node if is_state is true
    
            else if node.is_state {
    
                match _create_node_result {
                    Ok(node) => {
                        let _convert_function = "create_state";
                        let _result_state = 
                        juliainterface::_call1(&mut frame, module, _convert_function, node).unwrap();
    
                        //Add state
    
                        match _result_state {
                            Ok(state) => {
                                let _add_state = "add_state";
                                let _add_state_result = 
                                juliainterface::_call2(&mut frame, module, _convert_function, node, state).unwrap();
                            },
                            Err(error) => println!("Error adding inflow to node: {:?}", error),
                        }
                    }
                    Err(error) => println!("Error adding inflow: {:?}", error),
                }
            }
    
            //Convert node to res node if is_res is true
            
            else if node.is_res {
                match _create_node_result {
                    Ok(node) => {
                        let _add_node_to_reserve = "add_node_to_reserve";
                        let _convert_result = juliainterface::_call1(&mut frame, module, _add_node_to_reserve, node).unwrap();
                    }
                    Err(error) => println!("Error creating is_res node: {:?}", error),
                }
            }
    
            //CREATING PROCESSES
    
            let p_name = JuliaString::new(&mut frame, process.name).as_value();
            let p_conversion = Value::new(&mut frame, process.conversion);
            let p_delay = Value::new(&mut frame, process.delay);
            let mut _process_result: Value<'_, '_>;
    
            if process.conversion == 1 {
    
                //Predicer.Process
    
                let _create_process = "create_process"; 
                let _create_process_result = 
                juliainterface::_call3(&mut frame, module, _create_process, p_name, p_conversion, p_delay).unwrap();
    
                match _create_process_result {
                    Ok(result) => {
                        _process_result = result;
    
                        let _p_load_min = Value::new(&mut frame, process.load_min);
                        let _p_load_max = Value::new(&mut frame, process.load_max);
    
                        let _add_load_limits = "add_load_limits"; 
                        let _add_load_limits_result = 
                        juliainterface::_call3(&mut frame, module, _add_load_limits, _process_result, _p_load_min, _p_load_max).unwrap();
    
                        match _add_load_limits_result {
                            Ok(result) => {
    
                                _process_result = result;
            
                                let _p_eff = Value::new(&mut frame, process.eff);
            
                                let _add_eff = "add_load_limits"; 
                                let _add_eff_result = 
                                juliainterface::_call2(&mut frame, module, _add_eff, _process_result, _p_eff).unwrap();
            
                            }
                            Err(error) => println!("Error creating is_res node: {:?}", error),
                        }
    
                        //add_cf
    
                        //Muodostetaan sources ja sinks listat, koska add_topology funktio tehdään erikseen sourceille ja sinkeille
    
                        //add_topology
    
                        //ongelma, miten kutsua jotain funktiota, millä on useampi kuin 3 
    
                        for so in _sources {
    
                        }
    
                    }
                    Err(error) => println!("Error creating is_res node: {:?}", error),
                }
    
            }
    
            else if process.conversion == 2 {
    
                //Predicer.TransferProcess
    
                let _transfer_process = "TransferProcess"; 
                let _transfer_process_result = 
                juliainterface::_call1(&mut frame, module, _transfer_process, p_name).unwrap();
    
                match _transfer_process_result {
                    Ok(result) => {
                        _process_result = result;
    
                        let _p_load_min = Value::new(&mut frame, process.load_min);
                        let _p_load_max = Value::new(&mut frame, process.load_max);
    
                        let _add_load_limits = "add_load_limits"; 
                        let _add_load_limits_result = 
                        juliainterface::_call3(&mut frame, module, _add_load_limits, _process_result, _p_load_min, _p_load_max).unwrap();
    
                        match _add_load_limits_result {
                            Ok(result) => {
    
                                _process_result = result;
            
                                let _p_eff = Value::new(&mut frame, process.eff);
            
                                let _add_eff = "add_load_limits"; 
                                let _add_eff_result = 
                                juliainterface::_call2(&mut frame, module, _add_eff, _process_result, _p_eff).unwrap();
            
                            }
                            Err(error) => println!("Error creating is_res node: {:?}", error),
                        }
    
                    }
                    Err(error) => println!("Error creating is_res node: {:?}", error),
                }
    
            }
    
            else {
    
                //Predicer.MarketProcess
    
                let _market_process = "MarketProcess"; 
                let _market_process_result = 
                juliainterface::_call1(&mut frame, module, _market_process, p_name).unwrap();
    
                match _market_process_result {
                    Ok(result) => {
                        _process_result = result;
    
                        let _p_load_min = Value::new(&mut frame, process.load_min);
                        let _p_load_max = Value::new(&mut frame, process.load_max);
    
                        let _add_load_limits = "add_load_limits"; 
                        let _add_load_limits_result = 
                        juliainterface::_call3(&mut frame, module, _add_load_limits, _process_result, _p_load_min, _p_load_max).unwrap();
    
                        match _add_load_limits_result {
                            Ok(result) => {
    
                                _process_result = result;
            
                                let _p_eff = Value::new(&mut frame, process.eff);
            
                                let _add_eff = "add_load_limits"; 
                                let _add_eff_result = 
                                juliainterface::_call2(&mut frame, module, _add_eff, _process_result, _p_eff).unwrap();
            
                            }
                            Err(error) => println!("Error creating is_res node: {:?}", error),
                        }
    
                    }
                    Err(error) => println!("Error creating is_res node: {:?}", error),
                }
    
            }
    
    
            
    
            Ok(())
            
        }).expect("result is an error");
        
    
    }

    */
    
    
    

}



