
use std::sync::mpsc::{Sender};
use std::collections::HashMap;
mod main_loop;
mod utilities;
mod juliainterface;
mod structures;

pub fn start_sending(tx: Sender<main_loop::_Task>) {
    
    tx.send(main_loop::_Task::StartProcess).unwrap();
    tx.send(main_loop::_Task::QuitProcess).unwrap();
    tx.send(main_loop::_Task::StartProcess).unwrap();
}


fn main() {

    let _nodes: HashMap<String, structures::Node>;
    let _processes: HashMap<String, structures::Process>;
    let _markets: HashMap<String, structures::Market>;
    let _scenarios: HashMap<String, f64>;
    let _reserve_type: HashMap<String, f64>;
    let _risk: HashMap<String, f64>;
    let _gen_constraints: HashMap<String, structures::GenConstraint>;
    let mut _sources: HashMap<&String, &structures::Topology> = HashMap::new();
    let mut _sinks: HashMap<&String, &structures::Topology> = HashMap::new();

    //Create timeseries

    let series1: Vec<(String, String)> = vec![
        ("Data1".to_string(), "Value1".to_string()),
        ("Data2".to_string(), "Value2".to_string()),
    ];

    let time_series1 = structures::TimeSeries {
        scenario: "Scenario1".to_string(),
        series: series1,
    };

    let series2: Vec<(String, String)> = vec![
        ("Data3".to_string(), "Value3".to_string()),
        ("Data4".to_string(), "Value4".to_string()),
    ];
    let time_series2 = structures::TimeSeries {
        scenario: "Scenario2".to_string(),
        series: series2,
    };

    // Step 2: Create a Vec<TimeSeries> containing the created TimeSeries instances
    let time_series_data_vec: Vec<structures::TimeSeries> = vec![time_series1, time_series2];

    // Step 3: Create a new TimeSeriesData instance with the Vec<TimeSeries>
    let time_series_data: structures::TimeSeriesData = structures::TimeSeriesData {
        ts_data: time_series_data_vec,
    };

    //Creating sinks and sources

    let _electricheater_sink = structures::Topology {
        source: false,
        sink: true,
        name: "electricheater".to_string(), //name of the sink
        capacity: 750.0,
        vom_cost: 0.0,
        ramp_up: 1.0,
        ramp_down: 1.0,
    };

    let _electricheater_source = structures::Topology {
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

    let _interiorair = structures::Node {
        name: String::from("interiorair"),
        is_commodity: false,
        is_state: true,
        is_res: false,
        is_market: false,
        is_inflow: false,
        cost: &time_series_data,
        inflow: &time_series_data,
    };

    let _building_envelope = structures::Node {
        name: String::from("buildingenvelope"),
        is_commodity: false,
        is_state: true,
        is_res: false,
        is_market: false,
        is_inflow: false,
        cost: &time_series_data,
        inflow: &time_series_data,
    };

    let _outside = structures::Node {
        name: String::from("outside"),
        is_commodity: false,
        is_state: true,
        is_res: false,
        is_market: false,
        is_inflow: true,
        cost: &time_series_data,
        inflow: &time_series_data,
    };

    let _electricitygrid = structures::Node {
        name: String::from("electricitygrid"),
        is_commodity: false,
        is_state: false,
        is_res: false,
        is_market: false,
        is_inflow: false,
        cost: &time_series_data,
        inflow: &time_series_data,
    };

}