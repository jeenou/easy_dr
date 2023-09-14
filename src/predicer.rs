use crate::julia_interface;
use jlrs::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;

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
    pub topos: &'a Vec<Topology>,
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

pub struct NodeDiffusion {
    pub name: String,
    pub node1: String,
    pub node2: String,
    pub diff_coeff: f64,
}

pub struct NodeDelay {
    pub name: String,
    pub node1: String,
    pub node2: String,
    pub delay: f64,
    pub min_flow: f64,
    pub max_flow: f64,
}

pub struct Scenario {
    name: String,
    probability: f64,
}

pub struct Topology {
    pub source: String,
    pub sink: String,
    pub capacity: f64,
    pub vom_cost: f64,
    pub ramp_up: f64,
    pub ramp_down: f64,
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

pub struct NodeHistory<'a> {
    pub node: String,
    pub steps: &'a TimeSeriesData,
}

pub struct GenConstraint<'a> {
    pub name: String,
    pub gc_type: String,
    pub is_setpoint: bool,
    pub penalty: f64,
    pub factors: Vec<ConFactor<'a>>,
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
        julia
            .scope(|mut frame| {
                let predicer_dir =
                    JuliaString::new(&mut frame, "use command line arg here").as_value();
                let _ = Module::main(&frame)
                    .function(&frame, "cd")?
                    .as_managed()
                    .call1(&mut frame, predicer_dir)
                    .expect("cd to Predicer dir failed");
                Ok(())
            })
            .expect("error when cding to Predicer dir");
        julia
            .scope(|mut frame| {
                let predicer_dir =
                    JuliaString::new(&mut frame, "use command line arg here").as_value();
                let _ = Value::eval_string(&mut frame, "using Pkg");
                let _ = Module::main(&frame)
                    .submodule(&frame, "Pkg")?
                    .as_managed()
                    .function(&frame, "activate")?
                    .as_managed()
                    .call1(&mut frame, predicer_dir)
                    .expect("activation failed");
                Ok(())
            })
            .expect("error when activating Julia environment");
        julia
            .scope(|mut frame| {
                Module::main(&frame)
                    .submodule(&frame, "Pkg")?
                    .as_managed()
                    .function(&frame, "instantiate")?
                    .as_managed()
                    .call0(&mut frame)
                    .expect("instatiation failed");
                Ok(())
            })
            .expect("error when instantiating Julia environment");
        julia
            .scope(|mut frame| {
                let wd = Module::main(&frame)
                    .function(&frame, "pwd")?
                    .as_managed()
                    .call0(&mut frame)
                    .into_jlrs_result()?
                    .unbox::<String>()
                    .expect("pwd error");
                println!("working directory {}", wd.expect("not ok"));
                Ok(())
            })
            .expect("error error on the wall");
        julia
            .scope(|mut frame| {
                let wd = Module::main(&frame)
                    .function(&frame, "using Predicer")?
                    .as_managed()
                    .call0(&mut frame)
                    .into_jlrs_result()?
                    .unbox::<String>()
                    .expect("Predicer error");
                println!("working directory {}", wd.expect("not ok"));
                Ok(())
            })
            .expect("error error on the wall");
        let path = PathBuf::from("src/structures.jl");
        julia.include(path).expect("Could not include file1");
    }

    // An extended target provides a target for the result we want to return and a frame for
    // temporary data.
    let _x = julia
        .scope(|mut frame| {
            let _result2 =
                julia_interface::to_ordered_dict(frame.as_extended_target(), &data).unwrap();

            let _result3 =
                julia_interface::call(&mut frame, &["Predicer", "print_ordered_dict"], &[_result2])
                    .into_jlrs_result();

            Ok(())
        })
        .expect("result is an error");
}

pub fn _predicer(
    contains_reserves: bool,
    contains_online: bool,
    contains_state: bool,
    contains_piecewise_eff: bool,
    contains_risk: bool,
    contains_delay: bool,
    contains_diffusion: bool,
    nodes: HashMap<&String, &Node>,
    processes: HashMap<&String, &Process>,
    markets: HashMap<&String, &Market>,
    groups: HashMap<&String, &Group>,
    gen_constraints: HashMap<&String, &GenConstraint>,
    node_diffusion: HashMap<&String, &NodeDiffusion>,
    node_delay: HashMap<&String, &NodeDelay>,
    predicer_dir: &str,
) {
    let mut frame = StackFrame::new();
    let mut pending = unsafe { RuntimeBuilder::new().start().expect("Could not init Julia") };
    let mut julia = pending.instance(&mut frame);

    unsafe {
        julia
            .scope(|mut frame| {
                let jl_predicer_dir = JuliaString::new(&mut frame, predicer_dir).as_value();
                Value::eval_string(&mut frame, "using Pkg");
                julia_interface::call(&frame, &["Pkg", "activate"], &[jl_predicer_dir]);
                julia_interface::call(&frame, &["Pkg", "instantiate"], &[]);
                Value::eval_string(&mut frame, "using Predicer");

                //Create processes TÄSSÄ TOPOLOGY ONGELMA

                for (key, value) in &processes {
                    let p_d1 = JuliaString::new(&mut frame, key).as_value();
                    let p_d2 = Value::new(&mut frame, value.conversion);

                    let process = julia_interface::call(
                        &mut frame,
                        &["Predicer", "create_process"],
                        &[p_d1, p_d2],
                    )
                    .into_jlrs_result();

                    let mut process_with_topo: Value<'_, '_>;

                    match process {
                        Ok(process_value) => {
                            process_with_topo = process_value;

                            for topo in value.topos {
                                let t_d1 = JuliaString::new(&mut frame, &topo.source).as_value();
                                let t_d2 = JuliaString::new(&mut frame, &topo.sink).as_value();
                                let t_d3 = Value::new(&mut frame, topo.capacity);
                                let t_d4 = Value::new(&mut frame, topo.vom_cost);
                                let t_d5 = Value::new(&mut frame, topo.ramp_up);
                                let t_d6 = Value::new(&mut frame, topo.ramp_down);

                                let _create_topology = julia_interface::call(
                                    &mut frame,
                                    &["Predicer", "create_topology"],
                                    &[t_d1, t_d2, t_d3, t_d4, t_d5, t_d6],
                                );

                                //Miten tämä pitäisi tehdä?

                                //Tässä pitäisi luoda prosessi ja lisätä topologyt process-muuttujaan kohtaan Process.topos::Vector{Topology}
                            }

                            //Tässä lisätään prosessi h_processes ordered dictiin
                            //let _add_to_processes = julia_interface::call(&mut frame, &["Predicer", "add_to_processes"], &[process_with_topo]).into_jlrs_result();
                        }
                        Err(error) => println!("Error creating process: {:?}", error),
                    }
                }

                let _processes =
                    julia_interface::call(&mut frame, &["Predicer", "return_processes"], &[])
                        .into_jlrs_result();
                let mut list: Vec<Value> = Vec::new();

                match _processes {
                    Ok(processes_value) => {
                        //list[0]
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

                    let _node = julia_interface::call(
                        &mut frame,
                        &["Predicer", "create_node"],
                        &[n_d1, n_d2, n_d3],
                    )
                    .into_jlrs_result();

                    match _node {
                        Ok(node_value) => {
                            let _add_to_nodes_result = julia_interface::call(
                                &mut frame,
                                &["Predicer", "add_to_nodes"],
                                &[node_value],
                            );
                            match _add_to_nodes_result {
                                Ok(_) => println!("Added to nodes!"),
                                Err(error) => println!("Error adding node to nodes: {:?}", error),
                            }
                        }
                        Err(error) => println!("Error adding node to nodes2: {:?}", error),
                    }
                }

                let n_args = [];
                let _nodes =
                    julia_interface::call(&mut frame, &["Predicer", "return_nodes"], &n_args)
                        .into_jlrs_result();

                match _nodes {
                    Ok(nodes_value) => {
                        //list[1]
                        list.push(nodes_value);
                    }
                    Err(error) => println!("Error returning nodes: {:?}", error),
                }

                //Node diffusion

                for (key, value) in &node_diffusion {
                    // An extended target provides a target for the result we want to return and a frame for
                    // temporary data.

                    let nd_d1 = JuliaString::new(&mut frame, &value.node1).as_value();
                    let nd_d2 = JuliaString::new(&mut frame, &value.node2).as_value();
                    let nd_d3 = Value::new(&mut frame, value.diff_coeff);

                    let _node_diffusion_tuple = julia_interface::call(
                        &mut frame,
                        &["Predicer", "create_node_diffusion_tuple"],
                        &[nd_d1, nd_d2, nd_d3],
                    )
                    .into_jlrs_result();

                    match _node_diffusion_tuple {
                        Ok(_) => println!("Added node diffusion tuple!"),
                        Err(error) => println!("Error adding node diffusion tuple: {:?}", error),
                    }
                }

                let nd_args = [];
                let _node_diffusion_tuples = julia_interface::call(
                    &mut frame,
                    &["Predicer", "return_node_diffusion_tuples"],
                    &nd_args,
                )
                .into_jlrs_result();

                match _node_diffusion_tuples {
                    Ok(node_diffusion_value) => {
                        //list[2]
                        list.push(node_diffusion_value);
                    }
                    Err(error) => println!("Error returning node diffusion tuples: {:?}", error),
                }

                //node_delay

                for (key, value) in &node_delay {
                    // An extended target provides a target for the result we want to return and a frame for
                    // temporary data.

                    let nde_d1 = JuliaString::new(&mut frame, &value.node1).as_value();
                    let nde_d2 = JuliaString::new(&mut frame, &value.node2).as_value();
                    let nde_d3 = Value::new(&mut frame, value.delay);
                    let nde_d4 = Value::new(&mut frame, value.min_flow);
                    let nde_d5 = Value::new(&mut frame, value.max_flow);

                    let _node_delay_tuple = julia_interface::call(
                        &mut frame,
                        &["Predicer", "create_node_delay_tuple"],
                        &[nde_d1, nde_d2, nde_d3, nde_d4, nde_d5],
                    )
                    .into_jlrs_result();

                    match _node_delay_tuple {
                        Ok(_) => println!("Added node delay tuple!"),
                        Err(error) => println!("Error adding node delay tuple: {:?}", error),
                    }
                }

                let nde_args = [];
                let _node_delay_tuples = julia_interface::call(
                    &mut frame,
                    &["Predicer", "return_node_delay_tuples"],
                    &nde_args,
                )
                .into_jlrs_result();

                match _node_delay_tuples {
                    Ok(node_delay_value) => {
                        //list[3]
                        list.push(node_delay_value);
                    }
                    Err(error) => println!("Error returning node diffusion tuples: {:?}", error),
                }

                //node_histories

                let args1 = [];
                let node_histories = julia_interface::call(
                    &mut frame,
                    &["Predicer", "return_node_histories"],
                    &args1,
                )
                .into_jlrs_result();

                match node_histories {
                    Ok(nodehistory_value) => {
                        //list[4]
                        list.push(nodehistory_value);
                        println!("Added node histories")
                    }
                    Err(error) => println!("Error adding node histories: {:?}", error),
                }

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

                    let m_args = [d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11];

                    let _market =
                        julia_interface::call(&mut frame, &["Predicer", "create_market"], &m_args)
                            .into_jlrs_result();

                    match _market {
                        Ok(market_value) => {
                            let _add_to_markets_result = julia_interface::call(
                                &mut frame,
                                &["Predicer", "add_to_markets"],
                                &[market_value],
                            );
                            match _add_to_markets_result {
                                Ok(_) => println!("Added to markets!"),
                                Err(error) => {
                                    println!("Error adding market to markets: {:?}", error)
                                }
                            }
                        }
                        Err(error) => println!("Error adding market to markets2: {:?}", error),
                    }
                }

                let m_args = [];
                let _markets =
                    julia_interface::call(&mut frame, &["Predicer", "return_markets"], &m_args)
                        .into_jlrs_result();

                match _markets {
                    Ok(markets_value) => {
                        //list[5]
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

                    let g_args = [d1, d2, d3];

                    let _group =
                        julia_interface::call(&mut frame, &["Predicer", "create_group"], &g_args)
                            .into_jlrs_result();

                    match _group {
                        Ok(group_value) => {
                            let _add_to_groups_result = julia_interface::call(
                                &mut frame,
                                &["Predicer", "add_to_groups"],
                                &[group_value],
                            );
                            match _add_to_groups_result {
                                Ok(_) => println!("Added to groups!"),
                                Err(error) => println!("Error adding group to groups: {:?}", error),
                            }
                        }
                        Err(error) => println!("Error adding group to groups2: {:?}", error),
                    }
                }

                let g_args = [];
                let _groups =
                    julia_interface::call(&mut frame, &["Predicer", "return_groups"], &g_args)
                        .into_jlrs_result();

                match _groups {
                    Ok(groups_value) => {
                        //list[6]
                        list.push(groups_value);
                    }
                    Err(error) => println!("Error returning groups: {:?}", error),
                }

                //gen_constraints

                for (key, value) in &gen_constraints {
                    // An extended target provides a target for the result we want to return and a frame for
                    // temporary data.

                    let d1 = JuliaString::new(&mut frame, key).as_value();
                    let d2 = JuliaString::new(&mut frame, &value.gc_type).as_value();
                    let d3 = Value::new(&mut frame, value.is_setpoint);
                    let d4 = Value::new(&mut frame, value.penalty);

                    let gc_args = [d1, d2, d3, d4];

                    let _genconstraint = julia_interface::call(
                        &mut frame,
                        &["Predicer", "create_genconstraint"],
                        &gc_args,
                    )
                    .into_jlrs_result();

                    match _genconstraint {
                        Ok(gc_value) => {
                            let _add_to_genconstraints_result = julia_interface::call(
                                &mut frame,
                                &["Predicer", "add_to_genconstraints"],
                                &[gc_value],
                            );
                            match _add_to_genconstraints_result {
                                Ok(_) => println!("Added to genconstraints!"),
                                Err(error) => println!(
                                    "Error adding genconstraint to genconstraints: {:?}",
                                    error
                                ),
                            }
                        }
                        Err(error) => println!(
                            "Error adding gen constraint to gen_constraints2: {:?}",
                            error
                        ),
                    }
                }

                let gc_args = [];
                let _genconstraints = julia_interface::call(
                    &mut frame,
                    &["Predicer", "return_genconstraints"],
                    &gc_args,
                )
                .into_jlrs_result();

                match _genconstraints {
                    Ok(genconstraints_value) => {
                        //list[7]
                        list.push(genconstraints_value);
                    }
                    Err(error) => println!("Error returning genconstraints: {:?}", error),
                }

                //Scenarios

                let r_scenarios: Vec<(String, f64)> =
                    vec![(String::from("s1"), 0.5), (String::from("s2"), 0.5)];

                let j_scenarios =
                    julia_interface::to_ordered_dict(frame.as_extended_target(), &r_scenarios)
                        .unwrap();

                //reserve_type on tyhjä

                let r_reserve_type: Vec<(String, f64)> = Vec::new();

                let j_reserve_type =
                    julia_interface::to_ordered_dict(frame.as_extended_target(), &r_reserve_type)
                        .unwrap();

                //Risk

                let r_risk: Vec<(String, f64)> =
                    vec![(String::from("alfa"), 0.1), (String::from("beta"), 0.0)];

                let j_risk =
                    julia_interface::to_ordered_dict(frame.as_extended_target(), &r_risk).unwrap();

                //inflow_blocks

                let args1 = [];
                let inflowblocks =
                    julia_interface::call(&mut frame, &["Predicer", "return_inflowblocks"], &args1)
                        .into_jlrs_result();

                match inflowblocks {
                    Ok(inflowblocks_value) => {
                        //list[8]
                        list.push(inflowblocks_value);
                        println!("Added inflowblocks")
                    }
                    Err(error) => println!("Error adding inflowblocks: {:?}", error),
                }

                //Boolean parameters

                let j_contains_reserves = Value::new(&mut frame, contains_reserves);
                let j_contains_online = Value::new(&mut frame, contains_online);
                let j_contains_state = Value::new(&mut frame, contains_state);
                let j_contains_piecewise_eff = Value::new(&mut frame, contains_piecewise_eff);
                let j_contains_risk = Value::new(&mut frame, contains_risk);
                let j_contains_delay = Value::new(&mut frame, contains_delay);
                let j_contains_diffusion = Value::new(&mut frame, contains_diffusion);

                let i_args = [
                    list[0], //processes
                    list[1], //nodes
                    list[2], //node diffusions
                    list[3], //node delay
                    list[4], //node histories
                    list[5], //markets
                    list[6], //groups
                    list[7], //gengonstraints
                    j_scenarios,
                    j_reserve_type,
                    j_risk,
                    list[8], //inflowblocks
                    j_contains_reserves,
                    j_contains_online,
                    j_contains_state,
                    j_contains_piecewise_eff,
                    j_contains_risk,
                    j_contains_delay,
                    j_contains_diffusion,
                ];

                /*

                InputData ei ole oikein, ei toimi vielä

                let _input_data = julia_interface::call(&mut frame, &["Predicer", "create_inputdata2"], &i_args).into_jlrs_result();

                match _input_data {
                    Ok(id_value) => {
                        let _generate_model_result = julia_interface::call(&mut frame, &["Predicer", "solve_hertta"], &[id_value]);
                        match _generate_model_result {
                            Ok(_gm_value) => {
                                println!("Generated model")},
                            Err(error) => println!("Error generating model: {:?}", error),
                        }
                    }
                    Err(error) => println!("Error generating model: {:?}", error),
                }

                */

                Ok(())
            })
            .expect("result is an error");
    }
}

pub fn _test(da1: i64, da2: i64, da3: i64, da4: i64, data: Vec<(String, f64)>) {
    let mut frame = StackFrame::new();
    let mut pending = unsafe { RuntimeBuilder::new().start().expect("Could not init Julia") };
    let mut julia = pending.instance(&mut frame);
    // Include some custom code defined in MyModule.jl.
    // This is safe because the included code doesn't do any strange things.
    unsafe {
        julia
            .scope(|mut frame| {
                let predicer_dir = JuliaString::new(
                    &mut frame,
                    "C:\\Users\\enessi\\Documents\\easy_dr\\Predicer",
                )
                .as_value();
                let _ = Module::main(&frame)
                    .function(&frame, "cd")?
                    .as_managed()
                    .call1(&mut frame, predicer_dir)
                    .expect("cd to Predicer dir failed");
                Ok(())
            })
            .expect("error when cding to Predicer dir");
        julia
            .scope(|mut frame| {
                let predicer_dir = JuliaString::new(
                    &mut frame,
                    "C:\\Users\\enessi\\Documents\\easy_dr\\Predicer",
                )
                .as_value();
                let _ = Value::eval_string(&mut frame, "using Pkg");
                let _ = Module::main(&frame)
                    .submodule(&frame, "Pkg")?
                    .as_managed()
                    .function(&frame, "activate")?
                    .as_managed()
                    .call1(&mut frame, predicer_dir)
                    .expect("activation failed");
                Ok(())
            })
            .expect("error when activating Julia environment");
        julia
            .scope(|mut frame| {
                Module::main(&frame)
                    .submodule(&frame, "Pkg")?
                    .as_managed()
                    .function(&frame, "instantiate")?
                    .as_managed()
                    .call0(&mut frame)
                    .expect("instatiation failed");
                Ok(())
            })
            .expect("error when instantiating Julia environment");
        julia
            .scope(|mut frame| {
                let wd = Module::main(&frame)
                    .function(&frame, "pwd")?
                    .as_managed()
                    .call0(&mut frame)
                    .into_jlrs_result()?
                    .unbox::<String>()
                    .expect("pwd error");
                println!("working directory {}", wd.expect("not ok"));
                Ok(())
            })
            .expect("error error on the wall");
        let path = PathBuf::from("src/structures.jl");
        julia.include(path).expect("Could not include file1");
    }

    // An extended target provides a target for the result we want to return and a frame for
    // temporary data.
    let _x = julia
        .scope(|mut frame| {
            let d1 = Value::new(&mut frame, da1);
            let d2 = Value::new(&mut frame, da2);
            let d3 = Value::new(&mut frame, da3);
            let d4 = Value::new(&mut frame, da4);

            //let module = "Predicer";
            let _result = julia_interface::call(
                &mut frame,
                &["Predicer", "print_message"],
                &[d1, d2, d3, d4],
            )
            .into_jlrs_result();

            let _result2 =
                julia_interface::to_ordered_dict(frame.as_extended_target(), &data).unwrap();

            let _result3 =
                julia_interface::call(&mut frame, &["Predicer", "print_ordered_dict"], &[_result2])
                    .into_jlrs_result();

            Ok(())
        })
        .expect("result is an error");
}
