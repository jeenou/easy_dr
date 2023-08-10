
use std::sync::mpsc::{Sender};
use std::collections::HashMap;
mod main_loop;
mod utilities;
mod juliainterface;
mod structures;

use crate::structures::data;

pub fn start_sending(tx: Sender<main_loop::_Task>) {
    
    tx.send(main_loop::_Task::StartProcess).unwrap();
    tx.send(main_loop::_Task::QuitProcess).unwrap();
    tx.send(main_loop::_Task::StartProcess).unwrap();
}


fn main() {

    let _processes: HashMap<String, data::Process>;

    /*
    let _nodes: HashMap<String, data::Node>;
    let _markets: HashMap<String, data::Market>;
    let _scenarios: HashMap<String, f64>;
    let _reserve_type: HashMap<String, f64>; //tähän vaan key: reserve type ja value: ramp_factor
    let _risk: HashMap<String, f64>;
    let _gen_constraints: HashMap<String, data::GenConstraint>;
    let mut _sources: HashMap<&String, &data::Topology> = HashMap::new();
    let mut _sinks: HashMap<&String, &data::Topology> = HashMap::new();
    */

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

    //Creating process

    //Mitä eff_ops sisältää?

    let process_vec: Vec<String> = vec![
        ("eff_ops".to_string()),
    ];

    let _interiorair = data::Process {
        name: String::from("electricheater"),
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
        eff_ops: process_vec,
    };

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