
use std::sync::mpsc::{Sender};
use std::collections::HashMap;
mod main_loop;
mod utilities;
mod juliainterface;
mod structures;
mod input_data;

use crate::structures::data;
//use crate::input_data::functions;


pub fn start_sending(tx: Sender<main_loop::_Task>) {
    
    tx.send(main_loop::_Task::StartProcess).unwrap();
    tx.send(main_loop::_Task::QuitProcess).unwrap();
    tx.send(main_loop::_Task::StartProcess).unwrap();
}


fn main() {



    //Create timeseries

    let series1: Vec<(String, String)> = vec![
        ("Data1".to_string(), "Value1".to_string()),
        ("Data2".to_string(), "Value2".to_string()),
    ];

    let time_series1 = data::TimeSeries {
        scenario: "Scenario1".to_string(),
        series: series1,
    };

    let series2: Vec<(String, String)> = vec![
        ("Data3".to_string(), "Value3".to_string()),
        ("Data4".to_string(), "Value4".to_string()),
    ];
    let time_series2 = data::TimeSeries {
        scenario: "Scenario2".to_string(),
        series: series2,
    };

    // Step 2: Create a Vec<TimeSeries> containing the created TimeSeries instances
    let time_series_data_vec: Vec<data::TimeSeries> = vec![time_series1, time_series2];

    // Step 3: Create a new TimeSeriesData instance with the Vec<TimeSeries>
    let time_series_data: data::TimeSeriesData = data::TimeSeriesData {
        ts_data: time_series_data_vec,
    };

    //Creating nodes

    let _interiorair = data::Node {
        name: String::from("interiorair"),
        is_commodity: false,
        is_state: true,
        is_res: false,
        is_market: false,
        is_inflow: false,
        cost: &time_series_data,
        inflow: &time_series_data,
    };

    let _building_envelope = data::Node {
        name: String::from("buildingenvelope"),
        is_commodity: false,
        is_state: true,
        is_res: false,
        is_market: false,
        is_inflow: false,
        cost: &time_series_data,
        inflow: &time_series_data,
    };

    let _outside = data::Node {
        name: String::from("outside"),
        is_commodity: false,
        is_state: true,
        is_res: false,
        is_market: false,
        is_inflow: true,
        cost: &time_series_data,
        inflow: &time_series_data,
    };

    let _electricitygrid = data::Node {
        name: String::from("electricitygrid"),
        is_commodity: false,
        is_state: false,
        is_res: false,
        is_market: false,
        is_inflow: false,
        cost: &time_series_data,
        inflow: &time_series_data,
    };

    let mut _nodes: HashMap<&String, &data::Node> = HashMap::new();

    _nodes.insert(&_interiorair.name, &_interiorair);
    _nodes.insert(&_building_envelope.name, &_building_envelope);
    _nodes.insert(&_outside.name, &_outside);
    _nodes.insert(&_electricitygrid.name, &_electricitygrid);

    let da1 = 10;
    let da2 = 20;
    let da3 = 30;
    let da4 = 40;
    let data = vec![
        ("2022-08-21".to_string(), 10.0),
        ("2022-08-22".to_string(), 15.0),
    ];

    data::_test(da1, da2, da3, da4, data);


    //data::_predicer(_nodes);

    /*let mut _processes: HashMap<String, data::Process> = HashMap::new();

    //Creating process

    //Mitä eff_ops sisältää?

    let process_vec: Vec<String> = vec![
        ("eff_ops".to_string()),
    ];

    let _electricheater1 = data::Process {
        name: String::from("electricheater1"),
        is_cf: false,
        is_cf_fix: false,
        is_online: false,
        is_res: false,
        conversion: 1, //1,2 tai 3
        eff: 1.0,
        load_min: 0.0,
        load_max: 1.0,
        start_cost: 0.0,
        min_online: 0.0,
        min_offline: 0.0,
        max_online: 0.0,
        max_offline: 0.0,
        initial_state: 0.0,
        delay: 0.0,
        eff_ops: &process_vec,
    };

    let _electricheater2 = data::Process {
        name: String::from("electricheater2"),
        is_cf: false,
        is_cf_fix: false,
        is_online: false,
        is_res: false,
        conversion: 2, //1,2 tai 3
        eff: 1.0,
        load_min: 0.0,
        load_max: 1.0,
        start_cost: 0.0,
        min_online: 0.0,
        min_offline: 0.0,
        max_online: 0.0,
        max_offline: 0.0,
        initial_state: 0.0,
        delay: 0.0,
        eff_ops: &process_vec,
    };

    _processes.insert(_electricheater1.name.clone(), _electricheater1);
    _processes.insert(_electricheater2.name.clone(), _electricheater2);

    //functions::processes(&_processes);
    */

    //scenarios

    let _scenarios: Vec<(String, f64)> = vec![
        (String::from("s1"), 0.5),
        (String::from("s2"), 0.5),
    ];

    //reserve_type on tyhjä

    let _reserve_type: Vec<(String, f64)> = Vec::new();

    let _risk: Vec<(String, f64)> = vec![
        (String::from("alfa"), 0.1),
        (String::from("beta"), 0.0),
    ];

    data::_ordered_dict(_reserve_type);


    //risk




    /*
    let _markets: HashMap<String, data::Market>;
    let _scenarios: HashMap<String, f64>;
    let _reserve_type: HashMap<String, f64>; //tähän vaan key: reserve type ja value: ramp_factor
    let _risk: HashMap<String, f64>;
    let _gen_constraints: HashMap<String, data::GenConstraint>;
    let mut _sources: HashMap<&String, &data::Topology> = HashMap::new();
    let mut _sinks: HashMap<&String, &data::Topology> = HashMap::new();
    */

    /*

    

    //Creating Topology: sinks and sources

    let _electricheater_sink = data::Topology {
        source: false,
        sink: true,
        name: "electricheater".to_string(), //name of the sink
        capacity: 750.0,
        vom_cost: 0.0,
        ramp_up: 1.0,
        ramp_down: 1.0,
    };

    let _electricheater_source = data::Topology {
        source: true,
        sink: false,
        name: "electricheater".to_string(), //name of the sink
        capacity: 750.0,
        vom_cost: 0.0,
        ramp_up: 1.0,
        ramp_down: 1.0,
    };

    _sources.insert(&_electricheater_source.name, &_electricheater_source);
    _sinks.insert(&_electricheater_sink.name, &_electricheater_sink);

    */

    /* 

    //Creating markets

    let market_vec: Vec<(String, String)> = vec![
        ("key1".to_string(), "value1".to_string()),
        ("key2".to_string(), "value2".to_string()),
        ("key3".to_string(), "value3".to_string()),
    ];

    let _npe = data::Market {
        name: String::from("npe"),
        m_type: String::from("energy"),
        node: String::from("electricitygrid"), //mikä tyyppi
        direction: String::from("none"),
        realisation: 0.0,
        reserve_type: String::from("none"),
        is_bid: true,
        price: &time_series_data, //mitä tähän
        up_price: &time_series_data, //mitä tähän
        down_price: &time_series_data, // mitä tähän
        fixed: market_vec, //mitä tähän
    };

    */



}