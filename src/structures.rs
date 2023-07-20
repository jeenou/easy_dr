use jlrs::{prelude::*};
use std::{path::PathBuf};
//use jlrs::memory::target::frame::GcFrame;
//use jlrs::data::managed::value::ValueResult;
use crate::juliainterface::{juliainterface};
//use jlrs::error::JlrsError;
use std::collections::HashMap;

pub struct Process {
    name: String,
    is_cf: bool,
    is_cf_fix: bool,
    is_online: bool,
    is_res: bool,
    conversion: i64, //1,2 tai 3
    eff: f64,
    load_min: f64,
    load_max: f64,
    start_cost: f64,
    min_online: f64,
    min_offline: f64,
    max_online: f64,
    max_offline: f64,
    initial_state: f64,
    delay: f64,
    eff_ops: Vec<String>,
}

pub struct Scenario {
    name: String,
    probability: f64,
}


pub struct Topology {
    source: bool, 
    sink: bool, 
    pub name: String,
    capacity: f64,
    vom_cost: f64,
    ramp_up: f64,
    ramp_down: f64,
}

pub struct State {
    in_max: f64,
    out_max: f64,
    state_loss_proportional: f64,
    state_max: f64,
    state_min: f64,
    initial_state: f64,
    residual_value: f64,
}

pub struct TimeSeries {
    scenario: String,
    series: Vec<(String, String)>,
}

pub struct TimeSeriesData {
    ts_data: Vec<TimeSeries>,
}

pub struct Node<'a> {
    name: String,
    is_commodity: bool,
    is_state: bool,
    is_res: bool,
    is_market: bool,
    is_inflow: bool,
    cost: &'a TimeSeriesData,
    inflow: &'a TimeSeriesData,
    
}


pub struct Market<'a> {
    name: String,
    m_type: String,
    node: Node<'a>, //mikä tyyppi
    direction: String,
    realisation: f64,
    reserve_type: String,
    is_bid: bool,
    price: TimeSeriesData,
    up_price: TimeSeriesData,
    down_price: TimeSeriesData,
    fixed: Vec<(String, String)>,
    
}

pub struct ConFactor {
    flow: (String, String),
    data: TimeSeriesData,
}

pub struct GenConstraint {
    name: String,
    g_type: String,
    factors: Vec<ConFactor>,
    constant: TimeSeriesData,
    
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



pub fn contains_reserves(nodes: &HashMap<String, Node>) -> bool {
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

pub fn contains_online(processes: &HashMap<String, Process>) -> bool {
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

pub fn contains_states(nodes: &HashMap<String, Node>) -> bool {
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


pub fn contains_piecewise_eff(processes: &HashMap<String, Process>) -> bool {
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
pub fn contains_risk() {

}
*/

pub fn contains_delay(processes: &HashMap<String, Process>) -> bool {
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