

pub mod data {

    use jlrs::{prelude::*};
    use std::{path::PathBuf};
    use jlrs::memory::target::frame::GcFrame;
    //use jlrs::data::managed::value::ValueResult;
    use crate::juliainterface::julia;
    use jlrs::error::JlrsError;
    use std::collections::HashMap;
    use std::process::{Command, Stdio};
    use std::env;

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


    pub struct ConFactor<'a> {
        pub var_type: String,
        pub flow: (String, String),
        pub data: &'a TimeSeriesData,
    }

    pub struct InflowBlock {
        pub name: String,
        pub node: String,
        pub data: TimeSeriesData,
    }

    pub struct GenConstraint<'a>{
        pub name: String,
        pub gc_type: String,
        pub is_setpoint: bool,
        pub penalty: f64,
        pub factors: Vec< ConFactor<'a>>,
        pub constant: &'a TimeSeriesData,
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

    pub fn _ordered_dict(data: Vec<(String, f64)>) {
        let mut frame = StackFrame::new();
        let mut pending = unsafe { RuntimeBuilder::new().start().expect("Could not init Julia") };
        let mut julia = pending.instance(&mut frame);
        // Include some custom code defined in MyModule.jl.
        // This is safe because the included code doesn't do any strange things.
        unsafe {
            julia.scope(|mut frame| {
                let predicer_dir = JuliaString::new(&mut frame, "use command line arg here").as_value();
                let _ = Module::main(&frame)
                    .function(&frame, "cd")?
                    .as_managed()
                    .call1(&mut frame, predicer_dir).expect("cd to Predicer dir failed");
                Ok(())
            }).expect("error when cding to Predicer dir");
            julia.scope(|mut frame| {
                let predicer_dir = JuliaString::new(&mut frame, "use command line arg here").as_value();
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
            julia.scope(|mut frame| {
                let wd = Module::main(&frame)
                    .function(&frame, "using Predicer")?
                    .as_managed()
                    .call0(&mut frame).into_jlrs_result()?.unbox::<String>().expect("Predicer error");
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

            let _result3 = julia::call1(&mut frame, &["Predicer", "print_ordered_dict"], _result2).into_jlrs_result();

            Ok(())

        }).expect("result is an error");

    }

    pub fn _predicer2(
        contains_reserves: bool,
        contains_online: bool,
        contains_state: bool,
        contains_piecewise_eff: bool,
        contains_risk: bool,
        contains_delay: bool,
        nodes: HashMap<&String, &Node>, processes: HashMap<&String, &Process>, markets: HashMap<&String, &Market>, groups: HashMap<&String, &Group>, gen_constraints: HashMap<&String, &GenConstraint>,
        predicer_dir: &str) {
        let mut frame = StackFrame::new();
        let mut pending = unsafe { RuntimeBuilder::new().start().expect("Could not init Julia") };
        let mut julia = pending.instance(&mut frame);
        // Include some custom code defined in MyModule.jl.
        // This is safe because the included code doesn't do any strange things.

        let mut list: Vec<Value> = Vec::new();


        unsafe {
            julia.scope(|mut frame| {
                let jl_predicer_dir = JuliaString::new(&mut frame, predicer_dir).as_value();
                Value::eval_string(&mut frame, "using Pkg");
                julia::call(&frame, &["Pkg", "activate"], &[jl_predicer_dir]);
                julia::call(&frame, &["Pkg", "instantiate"], &[]);
                Value::eval_string(&mut frame, "using Predicer");

                //Create processes

                for (key, value) in &processes {

                //name::String, conversion::Int=1, delay::Float64=0.0

                    let p_d1 = JuliaString::new(&mut frame, key).as_value();
                    let p_d2 = Value::new(&mut frame, value.conversion);
                    let p_d3 = Value::new(&mut frame, value.delay);

                    let process = julia::call(&mut frame, &["Predicer", "create_process"], &[p_d1, p_d2, p_d3]).into_jlrs_result();

                    match process {
                        Ok(process_value) => {
                            let _add_to_processes_result = julia::call(&mut frame, &["Predicer", "add_to_processes"], &[process_value]);
                            match _add_to_processes_result {
                                Ok(_) => println!("Added to processes!"),
                                Err(error) => println!("Error adding process to processes: {:?}", error),
                            }
                        }
                        Err(error) => println!("Error adding process to processes2: {:?}", error),
                    }
                }

                let _processes = julia::call(&mut frame, &["Predicer", "return_processes"], &[]).into_jlrs_result();
                let mut list: Vec<Value> = Vec::new();

                match _processes {
                    Ok(processes_value) => {
                        list.push(processes_value);
                    }
                    Err(error) => println!("Error returning processes: {:?}", error),
                }

                //Create nodes

                for (key, value) in &nodes {

                    // An extended target provides a target for the result we want to return and a frame for
                    // temporary data.


                    let n_d1 = JuliaString::new(&mut frame, key).as_value();
                    let n_d2 = Value::new(&mut frame, value.is_commodity);
                    let n_d3 = Value::new(&mut frame, value.is_market);


                    let _node = julia::call(&mut frame, &["Predicer", "create_node"], &[n_d1, n_d2, n_d3]).into_jlrs_result();

                    match _node {
                        Ok(node_value) => {
                            let _add_to_nodes_result = julia::call(&mut frame, &["Predicer", "add_to_nodes"], &[node_value]);
                            match _add_to_nodes_result {
                                Ok(_) => println!("Added to nodes!"),
                                Err(error) => println!("Error adding node to nodes: {:?}", error),
                            }
                        }
                        Err(error) => println!("Error adding node to nodes2: {:?}", error),
                    }
                }

                let n_args = [];
                let _nodes = julia::call(&mut frame, &["Predicer", "return_nodes"], &n_args).into_jlrs_result();

                match _nodes {
                    Ok(nodes_value) => {
                        list.push(nodes_value);
                    }
                    Err(error) => println!("Error returning nodes: {:?}", error),
                }

                //Täytyy lisätä funkti, joka palauttaa "nodes"

                //Nodes diffusion

                //Creating markets

                for (key, value) in &markets {

                    // An extended target provides a target for the result we want to return and a frame for
                    // temporary data.


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

                    let m_args = [d1,d2,d3,d4,d5,d6,d7,d8,d9,d10,d11];


                    let _market = julia::call(&mut frame, &["Predicer", "create_market"], &m_args).into_jlrs_result();

                    match _market {
                        Ok(market_value) => {
                            let _add_to_markets_result = julia::call(&mut frame, &["Predicer", "add_to_markets"], &[market_value]);
                            match _add_to_markets_result {
                                Ok(_) => println!("Added to markets!"),
                                Err(error) => println!("Error adding market to markets: {:?}", error),
                            }
                        }
                        Err(error) => println!("Error adding market to markets2: {:?}", error),
                    }

                }

                let m_args = [];
                let _markets = julia::call(&mut frame, &["Predicer", "return_markets"], &m_args).into_jlrs_result();

                match _markets {
                    Ok(markets_value) => {
                        list.push(markets_value);
                    }
                    Err(error) => println!("Error returning markets: {:?}", error),
                }

                //Creating groups

                for (key, value) in &groups {

                    // An extended target provides a target for the result we want to return and a frame for
                    // temporary data.


                    let d1 = JuliaString::new(&mut frame, key).as_value();
                    let d2 = JuliaString::new(&mut frame, &value.g_type).as_value();
                    let d3 = JuliaString::new(&mut frame, &value.entity).as_value();

                    let g_args = [d1,d2,d3];


                    let _group = julia::call(&mut frame, &["Predicer", "create_group"], &g_args).into_jlrs_result();

                    match _group {
                        Ok(group_value) => {
                            let _add_to_groups_result = julia::call(&mut frame, &["Predicer", "add_to_groups"], &[group_value]);
                            match _add_to_groups_result {
                                Ok(_) => println!("Added to groups!"),
                                Err(error) => println!("Error adding group to groups: {:?}", error),
                            }
                        }
                        Err(error) => println!("Error adding group to groups2: {:?}", error),
                    }

                }

                let g_args = [];
                let _groups = julia::call(&mut frame, &["Predicer", "return_groups"], &g_args).into_jlrs_result();

                match _groups {
                    Ok(groups_value) => {
                        list.push(groups_value);
                    }
                    Err(error) => println!("Error returning groups: {:?}", error),
                }

                for (key, value) in &gen_constraints {

                // An extended target provides a target for the result we want to return and a frame for
                // temporary data.


                    let d1 = JuliaString::new(&mut frame, key).as_value();
                    let d2 = JuliaString::new(&mut frame, &value.gc_type).as_value();
                    let d3 = Value::new(&mut frame, value.is_setpoint);
                    let d4 = Value::new(&mut frame, value.penalty);

                    let gc_args = [d1,d2,d3, d4];


                    let _genconstraint = julia::call(&mut frame, &["Predicer", "create_genconstraint"], &gc_args).into_jlrs_result();

                    match _genconstraint {
                        Ok(gc_value) => {
                            let _add_to_genconstraints_result = julia::call(&mut frame, &["Predicer", "add_to_genconstraints"], &[gc_value]);
                            match _add_to_genconstraints_result {
                                Ok(_) => println!("Added to genconstraints!"),
                                Err(error) => println!("Error adding genconstraint to genconstraints: {:?}", error),
                            }
                        }
                        Err(error) => println!("Error adding gen constraint to gen_constraints2: {:?}", error),
                    }

                }

                let gc_args = [];
                let _genconstraints = julia::call(&mut frame, &["Predicer", "return_genconstraints"], &gc_args).into_jlrs_result();

                match _genconstraints {
                    Ok(genconstraints_value) => {
                        list.push(genconstraints_value);
                    }
                    Err(error) => println!("Error returning genconstraints: {:?}", error),
                }

                //Boolean parameters

                let j_contains_reserves = Value::new(&mut frame, contains_reserves);
                let j_contains_online = Value::new(&mut frame, contains_online);
                let j_contains_state = Value::new(&mut frame, contains_state);
                let j_contains_piecewise_eff = Value::new(&mut frame, contains_piecewise_eff);
                let j_contains_risk = Value::new(&mut frame, contains_risk);
                let j_contains_delay = Value::new(&mut frame, contains_delay);
                //let j_contains_diffusion = Value::new(&mut frame, contains_diffusion);

                //Scenarios

                let r_scenarios: Vec<(String, f64)> = vec![
                    (String::from("s1"), 0.5),
                    (String::from("s2"), 0.5),
                ];

                let j_scenarios = julia::_to_ordered_dict(frame.as_extended_target(), &r_scenarios).unwrap();


                //reserve_type on tyhjä

                let r_reserve_type: Vec<(String, f64)> = Vec::new();

                let j_reserve_type = julia::_to_ordered_dict(frame.as_extended_target(), &r_reserve_type).unwrap();

                //Risk

                let r_risk: Vec<(String, f64)> = vec![
                    (String::from("alfa"), 0.1),
                    (String::from("beta"), 0.0),
                ];

                let j_risk = julia::_to_ordered_dict(frame.as_extended_target(), &r_risk).unwrap();

                //inflow_blocks

                //gen_constraints

                let args1 = [];
                let inflowblocks = julia::call(&mut frame, &["Predicer", "return_inflowblocks"], &args1).into_jlrs_result();

                match inflowblocks {
                    Ok(inflowblocks_value) => {
                        list.push(inflowblocks_value);
                        println!("Added inflowblocks")},
                    Err(error) => println!("Error adding inflowblocks: {:?}", error),
                }

                let i_args = [
                    j_contains_reserves,
                    j_contains_online,
                    j_contains_state,
                    j_contains_piecewise_eff,
                    j_contains_risk,
                    j_contains_delay,
                    list[0],
                    list[1],
                    list[2],
                    list[3],
                    j_scenarios,
                    j_reserve_type,
                    j_risk,
                    list[5],
                    list[4]

                ];

                let _input_data = julia::call(&mut frame, &["Predicer", "create_inputdata"], &i_args).into_jlrs_result();




                match _input_data {
                    Ok(id_value) => {
                        let _generate_model_result = julia::call(&mut frame, &["Predicer", "generate_model"], &[id_value]);
                        match _generate_model_result {
                            Ok(_gm_value) => {
                                println!("Generated model")},
                            Err(error) => println!("Error generating model: {:?}", error),
                        }
                    }
                    Err(error) => println!("Error generating model: {:?}", error),
                }



                Ok(())

            }).expect("result is an error");
        }
    }

    /*
    pub fn _predicer(
        contains_reserves: bool,
        contains_online: bool,
        contains_state: bool,
        contains_piecewise_eff: bool,
        contains_risk: bool,
        contains_delay: bool,
        nodes: HashMap<&String, &Node>, processes: HashMap<&String, &Process>, markets: HashMap<&String, &Market>, groups: HashMap<&String, &Group>, gen_constraints: HashMap<&String, &GenConstraint>) {
        let mut frame = StackFrame::new();
        let mut pending = unsafe { RuntimeBuilder::new().start().expect("Could not init Julia") };
        let mut julia = pending.instance(&mut frame);
        // Include some custom code defined in MyModule.jl.
        // This is safe because the included code doesn't do any strange things.

        let mut list: Vec<Value> = Vec::new();


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

        //Temporals


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
                            Ok(value) => {
                                let j_processes = value;
                                list.push(j_processes);
                                println!("Added process to processes")
                            },
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
                            Ok(_) => {
                                let j_nodes = value;
                                list.push(j_nodes);
                                println!("Added node to nodes")},
                            Err(error) => println!("Error adding node to nodes: {:?}", error),
                        }
                    }
                    Err(error) => println!("Error adding node to nodes2: {:?}", error),
                }

                Ok(())

            }).expect("result is an error");

        }

        //Nodes diffusion

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
                            Ok(value) => {
                                let j_markets = value;
                                list.push(j_markets);
                                println!("Added market to markets")},
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
                            Ok(value) => {
                                let j_groups = value;
                                list.push(j_groups);
                                println!("Added group to groups")},
                            Err(error) => println!("Error adding group to groups: {:?}", error),
                        }
                    }
                    Err(error) => println!("Error adding group to groups2: {:?}", error),
                }

                Ok(())

            }).expect("result is an error");

        }

        //Creating inflow_blocks (empty)

        //Creating gen_constraints

        for (key, value) in &gen_constraints {

            // An extended target provides a target for the result we want to return and a frame for
            // temporary data.
            let _x = julia.scope(|mut frame| {

                let d1 = JuliaString::new(&mut frame, key).as_value();
                let d2 = JuliaString::new(&mut frame, &value.gc_type).as_value();
                let d3 = Value::new(&mut frame, value.is_setpoint);
                let d4 = Value::new(&mut frame, value.penalty);

                let args = [d1,d2,d3, d4];


                let function = "create_genconstraint";
                let _genconstraint = julia::_call(&mut frame, function, &args).unwrap().into_jlrs_result();

                match _genconstraint {
                    Ok(value) => {
                        let _convert_function = "add_to_genconstraints";
                        let _convert_result = julia::_call1(&mut frame, _convert_function, value).unwrap();
                        match _convert_result {
                            Ok(value) => {
                                let j_genconstraints = value;
                                list.push(j_genconstraints);
                                println!("Added genconstraint to genconstraints")},
                            Err(error) => println!("Error adding gen constraint to gen_constraints: {:?}", error),
                        }
                    }
                    Err(error) => println!("Error adding gen constraint to gen_constraints2: {:?}", error),
                }

                Ok(())

            }).expect("result is an error");

        }

        let _x = julia.scope(|mut frame| {

            //Boolean parameters

            let j_contains_reserves = Value::new(&mut frame, contains_reserves);
            let j_contains_online = Value::new(&mut frame, contains_online);
            let j_contains_state = Value::new(&mut frame, contains_state);
            let j_contains_piecewise_eff = Value::new(&mut frame, contains_piecewise_eff);
            let j_contains_risk = Value::new(&mut frame, contains_risk);
            let j_contains_delay = Value::new(&mut frame, contains_delay);
            //let j_contains_diffusion = Value::new(&mut frame, contains_diffusion);


            //Scenarios

            let r_scenarios: Vec<(String, f64)> = vec![
                (String::from("s1"), 0.5),
                (String::from("s2"), 0.5),
            ];

            let j_scenarios = julia::_to_ordered_dict(frame.as_extended_target(), &r_scenarios).unwrap();


            //reserve_type on tyhjä

            let r_reserve_type: Vec<(String, f64)> = Vec::new();

            let j_reserve_type = julia::_to_ordered_dict(frame.as_extended_target(), &r_reserve_type).unwrap();

            //Risk

            let r_risk: Vec<(String, f64)> = vec![
                (String::from("alfa"), 0.1),
                (String::from("beta"), 0.0),
            ];

            let j_risk = julia::_to_ordered_dict(frame.as_extended_target(), &r_risk).unwrap();

            //inflow_blocks

            //gen_constraints

            let _return_inflowblocks = "return_inflowblocks";
            let args1 = [];
            let inflowblocks = julia::_call(&mut frame, _return_inflowblocks, &args1).unwrap().into_jlrs_result();

            match inflowblocks {
                Ok(value) => {
                    let j_inflowblocks = value;
                    list.push(j_inflowblocks);
                    println!("Added inflowblocks")},
                Err(error) => println!("Error adding inflowblocks: {:?}", error),
            }

            let args = [
                j_contains_reserves,
                j_contains_online,
                j_contains_state,
                j_contains_piecewise_eff,
                j_contains_risk,
                j_contains_delay,
                list[0],
                list[1],
                list[2],
                list[3],
                j_scenarios,
                j_risk,
                list[4],
                list[5]

            ];

            let function = "create_inputdata";

            let _input_data = julia::_call(&mut frame, function, &args).unwrap().into_jlrs_result();

            Ok(())



        }).expect("result is an error");





    }

    */


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
            let _result = julia::call(&mut frame, &["Predicer", "print_message"], &[d1, d2, d3, d4]).into_jlrs_result();

            let _result2 = julia::_to_ordered_dict(frame.as_extended_target(), &data).unwrap();

            let _result3 = julia::call(&mut frame, &["Predicer", "print_ordered_dict"], &[_result2]).into_jlrs_result();

            Ok(())
        }).expect("result is an error");

    }
}
