use crate::julia_interface;
use jlrs::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;
use jlrs::memory::target::frame;
use jlrs::convert::into_julia;
use jlrs::data::managed::union_all::UnionAll;

pub struct Process<'a> {
    pub name: String,
    pub group: String,
    pub delay: f64,
    pub is_cf: bool,
    pub is_cf_fix: bool,
    pub is_online: bool,
    pub is_res: bool,
    pub conversion: i64,
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
    pub state: State,
}

impl<'a> std::fmt::Debug for Node<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write your custom formatting logic here
        write!(f, "Node {{ /* ... */ }}")
    }
}

pub struct Market<'a> {
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
    pub price: &'a TimeSeriesData,
    pub up_price: &'a TimeSeriesData,
    pub down_price: &'a TimeSeriesData,
}

pub struct Group {
    pub name: String,
    pub g_type: String,
    pub entity: String,
}

impl<'a> std::fmt::Debug for Market<'a> {
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

pub struct Topology {
    pub source: String,
    pub sink: String,
    pub capacity: f64,
    pub vom_cost: f64,
    pub ramp_up: f64,
    pub ramp_down: f64,
}

#[derive(Default)]
pub struct State {
    pub in_max: f64,
    pub out_max: f64,
    pub state_loss_proportional: f64,
    pub state_max: f64,
    pub state_min: f64,
    pub initial_state: f64,
    pub is_temp: bool,
    pub t_e_conversion: f64,
    pub residual_value: f64,
}

pub struct TimeSeries {
    pub scenario: String,
    pub series: Vec<(String, f64)>,
}

pub struct TimeSeriesData {
    pub ts_data: Vec<TimeSeries>,
}

pub struct ConFactor<'a> {
    pub var_type: String,
    pub flow: (String, String),
    pub data: &'a TimeSeriesData,
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
    pub factors: &'a Vec<ConFactor<'a>>,
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

//voiko unrootata julia valuen sisemmästä scopesta? voiko siirtää scopesta toiseen, managed tai unmanaged

fn make_vector<'target, 'data, V: Copy + into_julia::IntoJulia>(frame: &mut frame::GcFrame<'target>, values: &[V], processes: HashMap<&String, &Process>) -> ValueResult<'target, 'data, frame::GcFrame<'target>> {
    let vector_type_generic = unsafe {
        Module::main(&frame).global(&frame, "Vector").unwrap().as_managed().cast::<UnionAll>().unwrap()
    };
    let types = [
        DataType::int64_type(frame).as_value(),
    ];
    let vector = unsafe {
        let vector_type = vector_type_generic
            .apply_types(&frame, types)
            .unwrap()
            .as_managed();
        vector_type.call0(&frame).unwrap().as_managed()
    };
    let push = unsafe {
        Module::base(&frame).function(&frame, "push!").expect("failed!").as_managed()
    };
    for x in values {
        frame.scope(|mut frame| {
            let value = Value::new(&mut frame, *x);
            unsafe {
                push.call2(&mut frame, vector, value)
            }.unwrap();
            Ok(())
        }).unwrap();
    }
    Ok(vector)
}

pub fn add_topology<'target, 'data>(frame: &mut frame::GcFrame<'target>, process: Value<'_, '_>, topos: &Vec<Topology>) {

    frame.scope(|mut frame| {

        for topo in topos {
            let t_source = JuliaString::new(&mut frame, &topo.source).as_value();
            let t_sink = JuliaString::new(&mut frame, &topo.sink).as_value();
            let t_capacity = Value::new(&mut frame, topo.capacity);
            let t_vom_cost = Value::new(&mut frame, topo.vom_cost);
            let t_ramp_up = Value::new(&mut frame, topo.ramp_up);
            let t_ramp_down = Value::new(&mut frame, topo.ramp_down);

            let _create_topology = julia_interface::call(
                &mut frame,
                &["Predicer", "create_topology"],
                &[t_source, t_sink, t_capacity, t_vom_cost, t_ramp_up, t_ramp_down],
            );

            match _create_topology {
                Ok(topology) => {
                    let _add_topology = julia_interface::call(
                        &mut frame, 
                        &["Predicer", "add_topology"], 
                        &[process, topology]
                    );
                }
                Err(error) => println!("Error adding topology to process: {:?}", error),
            }

        }

        Ok(())

    }).unwrap();

}

pub fn add_state<'target, 'data>(frame: &mut frame::GcFrame<'target>, node: Value<'_, '_>, value: &&Node<'_>) {

    frame.scope(|mut frame| {

        //create state

        let s_in_max = Value::new(&mut frame, value.state.in_max);
        let s_out_max = Value::new(&mut frame, value.state.out_max);
        let s_state_loss_proportional = Value::new(&mut frame, value.state.state_loss_proportional);
        let s_state_max = Value::new(&mut frame, value.state.state_max);
        let s_state_min = Value::new(&mut frame, value.state.state_min);
        let s_initial_state = Value::new(&mut frame, value.state.initial_state);
        let s_is_temp = Value::new(&mut frame, value.state.is_temp);
        let s_t_e_conversion = Value::new(&mut frame, value.state.t_e_conversion);
        let s_residual_value = Value::new(&mut frame, value.state.residual_value);


        let _create_state = julia_interface::call(
            &mut frame,
            &["Predicer", "create_state"],
            &[s_in_max, s_out_max, s_state_loss_proportional, s_state_max, s_state_min, s_initial_state, s_is_temp, s_t_e_conversion, s_residual_value],
        );

        //add state to node

        match _create_state {
            Ok(state) => {
                let _add_state = julia_interface::call(
                    &mut frame,
                    &["Predicer", "add_state"],
                    &[node, state],
                );
            }
            Err(error) => println!("Error creating state: {:?}", error),
        }

        Ok(())

    }).unwrap();

}

pub fn create_groups<'target, 'data>(frame: &mut frame::GcFrame<'target>, groups: &HashMap<&String, &Group>) {

    frame.scope(|mut frame| {

        //Creating groups

        for (key, value) in groups {
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
                }
                Err(error) => println!("Error adding group to groups: {:?}", error),
            }
        }

        Ok(())

    }).unwrap();

}

pub fn add_timeseries<'target, 'data>(frame: &mut frame::GcFrame<'target>, julia_value: Value<'_, '_>, ts_data: &Vec<TimeSeries>, function: &str) {

    frame.scope(|mut frame| {

        let _create_timeseriesdata = julia_interface::call(
            &mut frame,
            &["Predicer", "create_timeseriesdata"],
            &[],
        );

        match _create_timeseriesdata {
            Ok(timeseriesdata) => {

                for _time_serie in ts_data {

                    let ts_scenario = JuliaString::new(&mut frame, &_time_serie.scenario).as_value();

                    let _create_timeseries = julia_interface::call(
                        &mut frame,
                        &["Predicer", "create_timeseries"],
                        &[ts_scenario],
                    );

                    match _create_timeseries {
                        Ok(timeserie) => {

                            for time_point in &_time_serie.series {

                                let j_timestamp = JuliaString::new(&mut frame, &time_point.0).as_value();
                                let j_value = Value::new(&mut frame, time_point.1);

                                let _make_time_point = julia_interface::call(
                                    &mut frame,
                                    &["Predicer", "make_time_point"],
                                    &[j_timestamp, j_value],
                                );

                                match _make_time_point {
                                    Ok(time_point) => {
                                        let _push_time_point = julia_interface::call(
                                            &mut frame,
                                            &["Predicer", "push_time_point"],
                                            &[timeserie, time_point],
                                        );
                                    }
                                    Err(error) => println!("Error creating time point: {:?}", error),
                                } 
                                
                            }

                            let _push_timeseries = julia_interface::call(
                                &mut frame,
                                &["Predicer", "push_timeseries"],
                                &[timeseriesdata, timeserie],
                            );
                            
                        }
                        Err(error) => println!("Error creating timeseries: {:?}", error),
                    }       
                }

                //use timeseries in function

                let _function = julia_interface::call(
                    &mut frame,
                    &["Predicer", function],
                    &[julia_value, timeseriesdata],
                );

            }
            Err(error) => println!("Error creating timeseriesdata: {:?}", error),
        }

        Ok(())

    }).unwrap();

}

pub fn create_node_diffusion<'target, 'data>(frame: &mut frame::GcFrame<'target>, node_diffusion: HashMap<&String, &NodeDiffusion>) {

    frame.scope(|mut frame| {

        //Node diffusion

        for (_key, value) in &node_diffusion {
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
        }

        Ok(())

    }).unwrap();

}

pub fn create_node_delay<'target, 'data>(frame: &mut frame::GcFrame<'target>, node_delay: HashMap<&String, &NodeDelay>) {

    frame.scope(|mut frame| {

        //Node delay

        for (_key, value) in &node_delay {
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
        }

        Ok(())

    }).unwrap();

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
                Value::eval_string(&mut frame, "using Pkg").unwrap();
                julia_interface::call(&frame, &["Pkg", "activate"], &[jl_predicer_dir]).unwrap();
                julia_interface::call(&frame, &["Pkg", "instantiate"], &[]).unwrap();
                Value::eval_string(&mut frame, "using Predicer").unwrap();

                for (key, value) in &processes {
                    let p_name = JuliaString::new(&mut frame, key).as_value();
                    let p_conversion = Value::new(&mut frame, value.conversion);
                    let p_group = JuliaString::new(&mut frame, &value.group).as_value();
                    let p_delay = Value::new(&mut frame, value.delay);


                    let process_result = julia_interface::call(
                        &mut frame,
                        &["Predicer", "create_process"],
                        &[p_name, p_conversion, p_delay],
                    )
                    .into_jlrs_result();

                    match process_result {
                        Ok(process) => {

                            add_topology(&mut frame, process, value.topos); 

                            let _add_group_to_processes = julia_interface::call(
                                &mut frame,
                                &["Predicer", "add_group_to_process"],
                                &[process, p_group],
                            );

                            let _add_to_processes = julia_interface::call(
                                &mut frame, 
                                &["Predicer", "add_to_processes"], 
                                &[process]
                            ).into_jlrs_result();

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

                    let n_name = JuliaString::new(&mut frame, key).as_value();
                    let n_is_commodity = Value::new(&mut frame, value.is_commodity);
                    let n_is_market = Value::new(&mut frame, value.is_market);
                    let n_is_inflow = Value::new(&mut frame, value.is_inflow);
                    let n_is_state = Value::new(&mut frame, value.is_state);

                    let _create_node = julia_interface::call(
                        &mut frame,
                        &["Predicer", "create_node"],
                        &[n_name, n_is_commodity, n_is_market, n_is_inflow, n_is_state],
                    )
                    .into_jlrs_result();

                    match _create_node {
                        Ok(node) => {

                            //create state

                            if value.is_state {

                                add_state(&mut frame, node, value);

                            }

                            //add inflow

                            if value.is_inflow {

                                let function = "add_inflow_to_node";
                                add_timeseries(&mut frame, node, &value.inflow.ts_data, function)

                            }

                            let _add_to_nodes_result = julia_interface::call(
                                &mut frame,
                                &["Predicer", "add_to_nodes"],
                                &[node],
                            );
                        }
                        Err(error) => println!("Error creating node: {:?}", error),
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

                create_node_diffusion(&mut frame, node_diffusion);

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

                create_node_delay(&mut frame, node_delay);

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
                        Ok(market) => {

                            //ADD TIMESERIES TO MARKET

                            let function = "add_market_prices";
                            add_timeseries(&mut frame, market, &value.price.ts_data, function);

                            //Create market up prices

                            let function = "add_market_up_prices";
                            add_timeseries(&mut frame, market, &value.up_price.ts_data, function);

                            //Create market down prices

                            let function = "add_market_down_prices";
                            add_timeseries(&mut frame, market, &value.down_price.ts_data, function);

                            let _add_to_markets_result = julia_interface::call(
                                &mut frame,
                                &["Predicer", "add_to_markets"],
                                &[market],
                            );
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

                create_groups(&mut frame, &groups);


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

                            //CREATE VARTUPLE FOR CONFACTOR

                            for confactor in value.factors {

                                                            //ADD TIMESERIES TO CONFACTOR

                                //create confactor

                                let s1 = JuliaString::new(&mut frame, &confactor.flow.0).as_value();
                                let s2 = JuliaString::new(&mut frame, &confactor.flow.1).as_value();

                                let g_args = [s1, s2];

                                //create vartuple

                                let _create_vartuple =
                                julia_interface::call(&mut frame, &["Predicer", "create_vartuple"], &g_args)
                                    .into_jlrs_result();

                                match _create_vartuple {
                                    Ok(vartuple) => {

                                        let vartype = JuliaString::new(&mut frame, &confactor.var_type).as_value();

                                        //create confactor

                                        let _create_confactor = julia_interface::call(
                                            &mut frame,
                                            &["Predicer", "create_confactor"],
                                            &[vartype, vartuple],
                                        );
                                        match _create_confactor {
                                            Ok(confactor_value) => {

                                                let function = "add_ts_to_confactor";
                                                add_timeseries(&mut frame, confactor_value, &confactor.data.ts_data, function);        

                                                //add confactor to gen constraints

                                                let _add_confactor_to_gc = julia_interface::call(
                                                    &mut frame,
                                                    &["Predicer", "add_confactor_to_gc"],
                                                    &[confactor_value,  gc_value],
                                                );
                                            }
                                            Err(error) => println!("Error creating confactor: {:?}", error),
                                        }
                                    }
                                    Err(error) => println!("Error creating vartuple: {:?}", error),
                                }

                            }


                            let _add_to_genconstraints_result = julia_interface::call(
                                &mut frame,
                                &["Predicer", "add_to_genconstraints"],
                                &[gc_value],
                            );
                        }
                        Err(error) => println!(
                            "Error adding gen constraint to gen_constraints: {:?}",
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

                
                let _input_data = julia_interface::call(&mut frame, &["Predicer", "create_inputdata"], &i_args).into_jlrs_result();

                match _input_data {
                    Ok(id_value) => {
                        let _generate_model_result = julia_interface::call(
                            &mut frame, 
                            &["Predicer", "solve_hertta"], 
                            &[id_value]
                        );


                        let _p_type = JuliaString::new(&mut frame, String::from("v_flow")).as_value();
                        let _name = JuliaString::new(&mut frame, String::from("electricheater")).as_value();
                        let _scenario = JuliaString::new(&mut frame, String::from("s1")).as_value();

                        let result_time_series: Vec<(String, f64)> = Vec::new();

                        match _generate_model_result {
                            Ok(df) => {
                                let _get_result_vec = julia_interface::call(
                                    &mut frame, 
                                    &["Predicer", "convert_df_to_vector"], 
                                    &[df]
                                );

                                match _get_result_vec {
                                    Ok(result_vec) => {
                                        let _get_value = julia_interface::call(
                                            &mut frame, 
                                            &["Predicer", "get_first_tuple_value"], 
                                            &[result_vec]
                                        );

                                        //add result 

                                    }
                                    Err(error) => println!("Error generating result: {:?}", error),
                                }
                            }
                            Err(error) => println!("Error generating result: {:?}", error),
                        }

                        

                    }
                    Err(error) => println!("Error solving model: {:?}", error),
                }

                //Get result dataframe

                //get_result_dataframe returns dataframe
                //in julia we have to convert the dataframe to something that can be used in rust
                //Lets change dataframe to a Vec<Tuple<String, f64>>
                

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
