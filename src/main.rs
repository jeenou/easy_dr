use std::collections::HashMap;
use std::env;
mod predicer;
use hertta::julia_interface;
use warp::Filter;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use serde_json;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use serde_json::json;
use std::fs;
use tokio::time::{self, Duration};
use tokio::task;


pub fn create_time_point(string: String, number: f64) -> (String, f64) {

    return (string, number)

}

pub fn add_time_point(ts_vec: &mut Vec<(String, f64)>, time_point: (String, f64)) {

    ts_vec.push(time_point);

}

pub fn add_time_serie(ts_data_vec: &mut Vec<predicer::TimeSeries>, time_series: predicer::TimeSeries) {

    ts_data_vec.push(time_series);

}

pub fn run_predicer() -> Vec<(String, f64)> {

    let args: Vec<String> = env::args().collect();
    let predicer_dir = args
        .get(1)
        .expect("first argument should be path to Predicer");

    //Example time series

    let mut series1: Vec<(String, f64)> = Vec::new();
    let mut series2: Vec<(String, f64)> = Vec::new();

    let timepoint1 = create_time_point("Data1".to_string(), 0.0);
    let timepoint2 = create_time_point("Data2".to_string(), 0.0);

    add_time_point(&mut series1, timepoint1.clone());
    add_time_point(&mut series1, timepoint2.clone());
    add_time_point(&mut series2, timepoint1.clone());
    add_time_point(&mut series2, timepoint2.clone());

    let time_series1 = predicer::TimeSeries {
        scenario: "Scenario1".to_string(),
        series: series1,
    };

    let time_series2 = predicer::TimeSeries {
        scenario: "Scenario2".to_string(),
        series: series2,
    };

    // Step 2: Create a Vec<TimeSeries> containing the created TimeSeries instances
    let mut time_series_data_vec: Vec<predicer::TimeSeries> = Vec::new();
    add_time_serie(&mut time_series_data_vec, time_series1);
    add_time_serie(&mut time_series_data_vec, time_series2);


    // Step 3: Create a new TimeSeriesData instance with the Vec<TimeSeries>
    let time_series_data: predicer::TimeSeriesData = predicer::TimeSeriesData {
        ts_data: time_series_data_vec,
    };

    //Outside temperatures (time series)

    //let mut outside_timeseries_s1: Vec<(String, f64)> = Vec::new();
    //let mut outside_timeseries_s2: Vec<(String, f64)> = Vec::new();

    //These outside temperatures come from HASS, we need a function that takes data from HASS and put that timeserie in to a vec

    let outside_timeseries_s1: Vec<(String, f64)> = vec![
        ("2022-04-20T00:00:00+00:00".to_string(), 3.0),
        ("2022-04-20T01:00:00+00:00".to_string(), 0.0),
        ("2022-04-20T02:00:00+00:00".to_string(), 4.0),
        ("2022-04-20T03:00:00+00:00".to_string(), -1.0),
        ("2022-04-20T04:00:00+00:00".to_string(), 5.0),
        ("2022-04-20T05:00:00+00:00".to_string(), -4.0),
        ("2022-04-20T06:00:00+00:00".to_string(), -5.0),
        ("2022-04-20T07:00:00+00:00".to_string(), -2.0),
        ("2022-04-20T08:00:00+00:00".to_string(), 4.0),
        ("2022-04-20T09:00:00+00:00".to_string(), 0.0),
    ];

    let outside_timeseries_s2: Vec<(String, f64)> = vec![
        ("2022-04-20T00:00:00+00:00".to_string(), -2.0),
        ("2022-04-20T01:00:00+00:00".to_string(), 4.0),
        ("2022-04-20T02:00:00+00:00".to_string(), 4.0),
        ("2022-04-20T03:00:00+00:00".to_string(), -1.0),
        ("2022-04-20T04:00:00+00:00".to_string(), 1.0),
        ("2022-04-20T05:00:00+00:00".to_string(), -3.0),
        ("2022-04-20T06:00:00+00:00".to_string(), 0.0),
        ("2022-04-20T07:00:00+00:00".to_string(), -5.0),
        ("2022-04-20T08:00:00+00:00".to_string(), -3.0),
        ("2022-04-20T09:00:00+00:00".to_string(), -2.0),
    ];

    let outside_ts_s1 = predicer::TimeSeries {
        scenario: "s1".to_string(),
        series: outside_timeseries_s1,
    };

    let outside_ts_s2 = predicer::TimeSeries {
        scenario: "s2".to_string(),
        series: outside_timeseries_s2,
    };

    let mut outside_ts_vec: Vec<predicer::TimeSeries> = Vec::new();
    add_time_serie(&mut outside_ts_vec, outside_ts_s1);
    add_time_serie(&mut outside_ts_vec, outside_ts_s2);

    let outside_ts: predicer::TimeSeriesData = predicer::TimeSeriesData {
        ts_data: outside_ts_vec,
    };

    //Market prices (time series)

    let npe_timeseries_s1: Vec<(String, f64)> = vec![
        ("2022-04-20T00:00:00+00:00".to_string(), 18.0),
        ("2022-04-20T01:00:00+00:00".to_string(), 5.0),
        ("2022-04-20T02:00:00+00:00".to_string(), 8.0),
        ("2022-04-20T03:00:00+00:00".to_string(), 6.0),
        ("2022-04-20T04:00:00+00:00".to_string(), 19.0),
        ("2022-04-20T05:00:00+00:00".to_string(), 24.0),
        ("2022-04-20T06:00:00+00:00".to_string(), 24.0),
        ("2022-04-20T07:00:00+00:00".to_string(), 21.0),
        ("2022-04-20T08:00:00+00:00".to_string(), 20.0),
        ("2022-04-20T09:00:00+00:00".to_string(), 10.0),
    ];

    let npe_timeseries_s2: Vec<(String, f64)> = vec![
        ("2022-04-20T00:00:00+00:00".to_string(), 8.0),
        ("2022-04-20T01:00:00+00:00".to_string(), 4.0),
        ("2022-04-20T02:00:00+00:00".to_string(), 8.0),
        ("2022-04-20T03:00:00+00:00".to_string(), 2.0),
        ("2022-04-20T04:00:00+00:00".to_string(), 24.0),
        ("2022-04-20T05:00:00+00:00".to_string(), 2.0),
        ("2022-04-20T06:00:00+00:00".to_string(), 10.0),
        ("2022-04-20T07:00:00+00:00".to_string(), 16.0),
        ("2022-04-20T08:00:00+00:00".to_string(), 11.0),
        ("2022-04-20T09:00:00+00:00".to_string(), 12.0),
    ];

    let npe_ts_s1 = predicer::TimeSeries {
        scenario: "s1".to_string(),
        series: npe_timeseries_s1,
    };

    let npe_ts_s2 = predicer::TimeSeries {
        scenario: "s2".to_string(),
        series: npe_timeseries_s2,
    };

    let npe_ts_vec: Vec<predicer::TimeSeries> = vec![npe_ts_s1, npe_ts_s2];

    let npe_ts: predicer::TimeSeriesData = predicer::TimeSeriesData {
        ts_data: npe_ts_vec,
    };

    //Market up prices (time series)

    let npe_up_prices_s1: Vec<(String, f64)> = vec![
        ("2022-04-20T00:00:00+00:00".to_string(), 19.8),
        ("2022-04-20T01:00:00+00:00".to_string(), 5.5),
        ("2022-04-20T02:00:00+00:00".to_string(), 8.8),
        ("2022-04-20T03:00:00+00:00".to_string(), 6.6),
        ("2022-04-20T04:00:00+00:00".to_string(), 20.9),
        ("2022-04-20T05:00:00+00:00".to_string(), 26.4),
        ("2022-04-20T06:00:00+00:00".to_string(), 26.4),
        ("2022-04-20T07:00:00+00:00".to_string(), 23.1),
        ("2022-04-20T08:00:00+00:00".to_string(), 22.0),
        ("2022-04-20T09:00:00+00:00".to_string(), 11.0),
    ];

    let npe_up_prices_s2: Vec<(String, f64)> = vec![
        ("2022-04-20T00:00:00+00:00".to_string(), 8.8),
        ("2022-04-20T01:00:00+00:00".to_string(), 4.4),
        ("2022-04-20T02:00:00+00:00".to_string(), 8.8),
        ("2022-04-20T03:00:00+00:00".to_string(), 2.2),
        ("2022-04-20T04:00:00+00:00".to_string(), 26.4),
        ("2022-04-20T05:00:00+00:00".to_string(), 2.2),
        ("2022-04-20T06:00:00+00:00".to_string(), 11.0),
        ("2022-04-20T07:00:00+00:00".to_string(), 17.6),
        ("2022-04-20T08:00:00+00:00".to_string(), 12.1),
        ("2022-04-20T09:00:00+00:00".to_string(), 13.2),
    ];

    let npe_up_s1 = predicer::TimeSeries {
        scenario: "s1".to_string(),
        series: npe_up_prices_s1,
    };

    let npe_up_s2 = predicer::TimeSeries {
        scenario: "s2".to_string(),
        series: npe_up_prices_s2,
    };

    let npe_up_vec: Vec<predicer::TimeSeries> = vec![npe_up_s1, npe_up_s2];

    let npe_up_ts: predicer::TimeSeriesData = predicer::TimeSeriesData {
        ts_data: npe_up_vec,
    };

    //Market down prices (time series)

    let npe_down_prices_s1: Vec<(String, f64)> = vec![
        ("2022-04-20T00:00:00+00:00".to_string(), 16.2),
        ("2022-04-20T01:00:00+00:00".to_string(), 4.5),
        ("2022-04-20T02:00:00+00:00".to_string(), 7.2),
        ("2022-04-20T03:00:00+00:00".to_string(), 5.4),
        ("2022-04-20T04:00:00+00:00".to_string(), 17.1),
        ("2022-04-20T05:00:00+00:00".to_string(), 21.6),
        ("2022-04-20T06:00:00+00:00".to_string(), 21.6),
        ("2022-04-20T07:00:00+00:00".to_string(), 18.9),
        ("2022-04-20T08:00:00+00:00".to_string(), 18.0),
        ("2022-04-20T09:00:00+00:00".to_string(), 9.0),
    ];

    let npe_down_prices_s2: Vec<(String, f64)> = vec![
        ("2022-04-20T00:00:00+00:00".to_string(), 7.2),
        ("2022-04-20T01:00:00+00:00".to_string(), 3.6),
        ("2022-04-20T02:00:00+00:00".to_string(), 7.2),
        ("2022-04-20T03:00:00+00:00".to_string(), 1.8),
        ("2022-04-20T04:00:00+00:00".to_string(), 21.6),
        ("2022-04-20T05:00:00+00:00".to_string(), 1.8),
        ("2022-04-20T06:00:00+00:00".to_string(), 9.0),
        ("2022-04-20T07:00:00+00:00".to_string(), 14.4),
        ("2022-04-20T08:00:00+00:00".to_string(), 9.9),
        ("2022-04-20T09:00:00+00:00".to_string(), 10.8),
    ];

    let npe_down_s1 = predicer::TimeSeries {
        scenario: "s1".to_string(),
        series: npe_down_prices_s1,
    };

    let npe_down_s2 = predicer::TimeSeries {
        scenario: "s2".to_string(),
        series: npe_down_prices_s2,
    };

    let npe_down_vec: Vec<predicer::TimeSeries> = vec![npe_down_s1, npe_down_s2];

    let npe_down_ts: predicer::TimeSeriesData = predicer::TimeSeriesData {
        ts_data: npe_down_vec,
    };

    //Gen constraints time series

    let c_interiorair_up_s1: Vec<(String, f64)> = vec![
        ("2022-04-20T00:00:00+00:00".to_string(), 298.15),
        ("2022-04-20T01:00:00+00:00".to_string(), 298.15),
        ("2022-04-20T02:00:00+00:00".to_string(), 298.15),
        ("2022-04-20T03:00:00+00:00".to_string(), 298.15),
        ("2022-04-20T04:00:00+00:00".to_string(), 298.15),
        ("2022-04-20T05:00:00+00:00".to_string(), 298.15),
        ("2022-04-20T06:00:00+00:00".to_string(), 298.15),
        ("2022-04-20T07:00:00+00:00".to_string(), 298.15),
        ("2022-04-20T08:00:00+00:00".to_string(), 298.15),
        ("2022-04-20T09:00:00+00:00".to_string(), 298.15),
    ];

    let c_interiorair_up_s2: Vec<(String, f64)> = vec![
        ("2022-04-20T00:00:00+00:00".to_string(), 298.15),
        ("2022-04-20T01:00:00+00:00".to_string(), 298.15),
        ("2022-04-20T02:00:00+00:00".to_string(), 298.15),
        ("2022-04-20T03:00:00+00:00".to_string(), 298.15),
        ("2022-04-20T04:00:00+00:00".to_string(), 298.15),
        ("2022-04-20T05:00:00+00:00".to_string(), 298.15),
        ("2022-04-20T06:00:00+00:00".to_string(), 298.15),
        ("2022-04-20T07:00:00+00:00".to_string(), 298.15),
        ("2022-04-20T08:00:00+00:00".to_string(), 298.15),
        ("2022-04-20T09:00:00+00:00".to_string(), 298.15),
    ];

    let c_interiorair_down_s1: Vec<(String, f64)> = vec![
        ("2022-04-20T00:00:00+00:00".to_string(), 292.15),
        ("2022-04-20T01:00:00+00:00".to_string(), 292.15),
        ("2022-04-20T02:00:00+00:00".to_string(), 292.15),
        ("2022-04-20T03:00:00+00:00".to_string(), 292.15),
        ("2022-04-20T04:00:00+00:00".to_string(), 292.15),
        ("2022-04-20T05:00:00+00:00".to_string(), 292.15),
        ("2022-04-20T06:00:00+00:00".to_string(), 292.15),
        ("2022-04-20T07:00:00+00:00".to_string(), 292.15),
        ("2022-04-20T08:00:00+00:00".to_string(), 292.15),
        ("2022-04-20T09:00:00+00:00".to_string(), 292.15),
    ];

    let c_interiorair_down_s2: Vec<(String, f64)> = vec![
        ("2022-04-20T00:00:00+00:00".to_string(), 292.15),
        ("2022-04-20T01:00:00+00:00".to_string(), 292.15),
        ("2022-04-20T02:00:00+00:00".to_string(), 292.15),
        ("2022-04-20T03:00:00+00:00".to_string(), 292.15),
        ("2022-04-20T04:00:00+00:00".to_string(), 292.15),
        ("2022-04-20T05:00:00+00:00".to_string(), 292.15),
        ("2022-04-20T06:00:00+00:00".to_string(), 292.15),
        ("2022-04-20T07:00:00+00:00".to_string(), 292.15),
        ("2022-04-20T08:00:00+00:00".to_string(), 292.15),
        ("2022-04-20T09:00:00+00:00".to_string(), 292.15),
    ];

    let interiorair_up_s1 = predicer::TimeSeries {
        scenario: "s1".to_string(),
        series: c_interiorair_up_s1, 
    };

    let interiorair_up_s2 = predicer::TimeSeries {
        scenario: "s2".to_string(),
        series: c_interiorair_up_s2,
    };

    let interiorair_down_s1 = predicer::TimeSeries {
        scenario: "s1".to_string(),
        series: c_interiorair_down_s1, 
    };

    let interiorair_down_s2 = predicer::TimeSeries {
        scenario: "s2".to_string(),
        series: c_interiorair_down_s2,
    };

    let gc_interiorair_up_vec: Vec<predicer::TimeSeries> = vec![interiorair_up_s1, interiorair_up_s2];
    let gc_interiorair_down_vec: Vec<predicer::TimeSeries> = vec![interiorair_down_s1, interiorair_down_s2];

    let interiorair_up_ts: predicer::TimeSeriesData = predicer::TimeSeriesData {
        ts_data: gc_interiorair_up_vec,
    };

    let interiorair_down_ts: predicer::TimeSeriesData = predicer::TimeSeriesData {
        ts_data: gc_interiorair_down_vec,
    };

    //Creating node_diffusion

    let diffusion_1 = predicer::NodeDiffusion {
        name: String::from("diffusion_1"),
        node1: String::from("interiorair"),
        node2: String::from("buildingenvelope"),
        diff_coeff: 0.5,
    };

    let diffusion_2 = predicer::NodeDiffusion {
        name: String::from("diffusion_2"),
        node1: String::from("buildingenvelope"),
        node2: String::from("outside"),
        diff_coeff: 0.4,
    };

    //Creating node_delay

    let delay_1 = predicer::NodeDelay {
        name: String::from("delay_1"),
        node1: String::from("dh1"),
        node2: String::from("dh2"),
        delay: 2.0,
        min_flow: 0.0,
        max_flow: 20.0,
    };

    //Creating nodes

    let interiorair_state = predicer::State {

        in_max: 1.0e10,
        out_max: 1.0e10,
        state_loss_proportional: 0.0,
        state_max: 308.15,
        state_min: 273.15,
        initial_state: 296.15,
        is_temp: true,
        t_e_conversion: 0.5,
        residual_value: 0.0,

    };

    let _interiorair = predicer::Node {
        name: String::from("interiorair"),
        is_commodity: false,
        is_state: true,
        is_res: false,
        is_market: false,
        is_inflow: false,
        cost: &time_series_data,
        inflow: &time_series_data,
        state: interiorair_state,
    };

    let building_envelope_state = predicer::State {

        in_max: 1.0e10,
        out_max: 1.0e10,
        state_loss_proportional: 0.0,
        state_max: 308.15,
        state_min: 238.15,
        initial_state: 273.15,
        is_temp: true,
        t_e_conversion: 1.0,
        residual_value: 0.0,

    };


    let _building_envelope_state = predicer::State {

        in_max: 1.0e10,
        out_max: 1.0e10,
        state_loss_proportional: 0.0,
        state_max: 308.15,
        state_min: 238.15,
        initial_state: 273.15,
        is_temp: true,
        t_e_conversion: 1.0,
        residual_value: 0.0,

    };

    let _building_envelope = predicer::Node {
        name: String::from("buildingenvelope"),
        is_commodity: false,
        is_state: true,
        is_res: false,
        is_market: false,
        is_inflow: false,
        cost: &time_series_data,
        inflow: &time_series_data,
        state: building_envelope_state,
    };

    let outside_state = predicer::State {

        in_max: 1.0e10,
        out_max: 1.0e10,
        state_loss_proportional: 0.0,
        state_max: 308.15,
        state_min: 238.15,
        initial_state: 268.15,
        is_temp: true,
        t_e_conversion: 1000000000.0,
        residual_value: 0.0,

    };

    let _outside = predicer::Node {
        name: String::from("outside"),
        is_commodity: false,
        is_state: true,
        is_res: false,
        is_market: false,
        is_inflow: true,
        cost: &time_series_data,
        inflow: &outside_ts,
        state: outside_state,
    };

    let empty_state: predicer::State = Default::default();

    let _electricitygrid = predicer::Node {
        name: String::from("electricitygrid"),
        is_commodity: false,
        is_state: false,
        is_res: false,
        is_market: false,
        is_inflow: false,
        cost: &time_series_data,
        inflow: &time_series_data,
        state: empty_state,
    };

    let _node_history_1 = predicer::NodeHistory {
        node: String::from("electricitygrid"),
        steps: &time_series_data,
    };

    let mut _nodes: HashMap<&String, &predicer::Node> = HashMap::new();
    let mut _node_diffusion: HashMap<&String, &predicer::NodeDiffusion> = HashMap::new();
    let mut _node_delay: HashMap<&String, &predicer::NodeDelay> = HashMap::new();

    _nodes.insert(&_interiorair.name, &_interiorair);
    _nodes.insert(&_building_envelope.name, &_building_envelope);
    _nodes.insert(&_outside.name, &_outside);
    _nodes.insert(&_electricitygrid.name, &_electricitygrid);

    _node_diffusion.insert(&diffusion_1.name, &diffusion_1);
    _node_diffusion.insert(&diffusion_2.name, &diffusion_2);

    _node_delay.insert(&delay_1.name, &delay_1);

    let mut _processes: HashMap<&String, &predicer::Process> = HashMap::new();

    //Creating topology for processes

    let topology1 = predicer::Topology {
        source: String::from("electricitygrid"),
        sink: String::from("electricheater"),
        capacity: 7.5,
        vom_cost: 0.0,
        ramp_up: 1.0,
        ramp_down: 1.0,
    };

    let topology2 = predicer::Topology {
        source: String::from("electricheater"),
        sink: String::from("interiorair"),
        capacity: 7.5,
        vom_cost: 0.0,
        ramp_up: 1.0,
        ramp_down: 1.0,
    };

    let topo_vec: Vec<predicer::Topology> = vec![topology1, topology2];

    //Creating process

    let process_vec: Vec<String> = vec![("eff_ops".to_string())];

    let _electricheater1 = predicer::Process {
        name: String::from("electricheater"),
        group: String::from("p1"),
        delay: 0.0,
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
        topos: &topo_vec,
        eff_ops: &process_vec,
    };

    _processes.insert(&_electricheater1.name, &_electricheater1);

    let mut _markets: HashMap<&String, &predicer::Market> = HashMap::new();
    let mut _groups: HashMap<&String, &predicer::Group> = HashMap::new();
    let mut _genconstraints: HashMap<&String, &predicer::GenConstraint> = HashMap::new();

    let _npe = predicer::Market {
        name: String::from("npe"),
        m_type: String::from("energy"),
        node: String::from("electricitygrid"),
        pgroup: String::from("p1"),
        direction: String::from("none"),
        realisation: 0.0,
        reserve_type: String::from("none"),
        is_bid: true,
        is_limited: false,
        min_bid: 0.0,
        max_bid: 0.0,
        fee: 0.0,
        price: &npe_ts,
        up_price: &npe_up_ts,
        down_price: &npe_down_ts,
    };

    _markets.insert(&_npe.name, &_npe);

    let _p1 = predicer::Group {
        name: String::from("p1"),
        g_type: String::from("process"),
        entity: String::from("electricheater"),
    };

    _groups.insert(&_p1.name, &_p1);

    let interiorair_up_cf = predicer::ConFactor {
        var_type: String::from("state"),
        flow: (String::from("interiorair"), String::from("")),
        data: &interiorair_up_ts,
    };

    let interiorair_up_cf_vec: Vec<predicer::ConFactor> = vec![interiorair_up_cf];

    let interiorair_down_cf = predicer::ConFactor {
        var_type: String::from("state"),
        flow: (String::from("interiorair"), String::from("")),
        data: &interiorair_down_ts,
    };

    let interiorair_down_cf_vec: Vec<predicer::ConFactor> = vec![interiorair_down_cf];

    let _c_interiorair_up = predicer::GenConstraint {
        name: String::from("c_interiorair_up"),
        gc_type: String::from("st"),
        is_setpoint: true,
        penalty: 1000.0,
        factors: &interiorair_up_cf_vec,
        constant: &time_series_data,
    };

    let _c_interiorair_down = predicer::GenConstraint {
        name: String::from("c_interiorair_down"),
        gc_type: String::from("gt"),
        is_setpoint: true,
        penalty: 1000.0,
        factors: &interiorair_down_cf_vec,
        constant: &time_series_data,
    };

    _genconstraints.insert(&_c_interiorair_up.name, &_c_interiorair_up);
    _genconstraints.insert(&_c_interiorair_down.name, &_c_interiorair_down);

    let mut _solution: Vec<(String, f64)> = Vec::new();
    
     
    _solution = predicer::predicer(
        false,
        false,
        true,
        false,
        false,
        false,
        true,
        _nodes,
        _processes,
        _markets,
        _groups,
        _genconstraints,
        _node_diffusion,
        _node_delay,
        predicer_dir,
    );

    return _solution
    

}

fn _print_tuple_vector(vec: &Vec<(String, f64)>) {
    for (s, num) in vec {
        println!("{}: {}", s, num);
    }
}

// Data structure for messaging between Home Assistant UI.
#[derive(Deserialize, Serialize, Debug)]
struct DataHass {
	entity_cat: i32,
	entity_id: String,
	data_type: i32,
	data_unit: String,
	data_str: String,
	data_int: i32,
	data_float: f32,
	data_bool: bool,
	date_time: String,
}

// Configuration options saved into a json file in the addon data directory.
#[derive(Deserialize, Debug)]
struct Options {
	floor_area: i32,
	stories: i32,
	insulation_u_value: f32,
    listen_ip: String,
    port: String,
    hass_token: String,
}

async fn _make_post_request(url: &str, data: &str, token: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Construct the request headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("{}", token)).unwrap(),
    );

    // Construct the payload as a JSON object
    let payload = json!({
        "title": "REST Call Received",
        "message": format!("data: {}", data),
    });
	
    // Send the POST request
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .headers(headers)
        .json(&payload) // Use the correct json! macro
        .send()
        .await?;

    // Check the response status
    if let Err(err) = response.error_for_status() {
        eprintln!("Error making POST request: {:?}", err);
        return Err(Box::new(err));
    }

    Ok(())
}

async fn make_post_request_light(url: &str, entity_id: &str, token: &str, brightness: f64) -> Result<(), Box<dyn std::error::Error>> {
    // Construct the request headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("{}", token)).unwrap(),
    );

    // Construct the payload as a JSON object
    let payload = json!({
        "entity_id": entity_id,
        "brightness": brightness,
    });
    
	
    // Send the POST request
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .headers(headers)
        .json(&payload) // Use the correct json! macro
        .send()
        .await?;

    // Check the response status
    if let Err(err) = response.error_for_status() {
        eprintln!("Error making POST request: {:?}", err);
        return Err(Box::new(err));
    }

    Ok(())
}

fn print_f64_vector(vector: &Vec<f64>) {
    for value in vector.iter() {
        println!("{}", value);
    }
}

async fn _run_logic(hass_token: String) -> Result<impl warp::Reply, warp::Rejection> {
    let vector: Vec<(String, f64)> = run_predicer();

    let brightness_values: Vec<f64> = vector.iter().map(|(_, value)| *value * 20.0).collect();
    print_f64_vector(&brightness_values);

    let url = "http://192.168.1.171:8123/api/services/light/turn_on";
    let entity_id = "light.katto1";
    
    for brightness in brightness_values {
        if let Err(err) = make_post_request_light(url, entity_id, &hass_token, brightness).await {
            eprintln!("Error in making POST request for brightness {}: {:?}", brightness, err);
        }

        // Wait for 5 seconds before sending the next request
        time::sleep(Duration::from_secs(5)).await;
    }

    // You can return some confirmation if needed
    Ok(warp::reply::json(&"Logic executed successfully"))
}

#[tokio::main]
async fn main() {
    // Uncomment and use the following line for checking directory contents

    
    //print_directory_contents("/app");
	//print_directory_contents("/usr/local/bin");

    //Tarkista missä options.jsonin pitäisi olla
	
    // Define the path to the options.json file
    let options_path = "/data/options.json";
    //let options_path = "./src/options.json";

    // Read the options.json file as a string
    let options_str = match fs::read_to_string(options_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading options.json: {}", err);
            return;
        }
    };

    // Parse the options JSON string into an Options struct
    let options: Options = match serde_json::from_str(&options_str) {
        Ok(parsed_options) => parsed_options,
        Err(err) => {
            eprintln!("Error parsing options.json: {}", err);
            return;
        }
    };
	
    // Extract option data from the options.json file.
	let _floor_area = &options.floor_area;
	let _stories = &options.stories;
	let _insulation_u_value = &options.insulation_u_value;
    let listen_ip = &options.listen_ip;
    let port = &options.port;
	let hass_token = &options.hass_token;
	
	// Partially mask the hass token for printing.
	let _masked_token = if options.hass_token.len() > 4 {
		let last_part = &options.hass_token[options.hass_token.len() - 4..];
		let masked_part = "*".repeat(options.hass_token.len() - 4);
		format!("{}{}", masked_part, last_part)
	} else {
		// If the token is too short, just print it as is
		options.hass_token.clone()
	};

    
	
    // Combine IP address and port into a single string
    let ip_port = format!("{}:{}", listen_ip, port);

    // Parse the combined string into a SocketAddr
    let ip_address: SocketAddr = ip_port.parse().unwrap();

    let hass_token_clone = hass_token.clone();

    let my_route = warp::path!("from_hass" / "post")
    .and(warp::post())
    .map(move || {
        // Clone the token for the spawned task
        let token = hass_token_clone.clone();
        // Spawn a new asynchronous task
        task::spawn(async move {
            // Here you call your logic function that contains the code you want to run
            if let Err(e) = _run_logic(token).await {
                // Handle any errors that might occur
                eprintln!("Error running logic: {:?}", e);
            }
        });

        // Immediately respond to the POST request
        warp::reply::json(&"Request received, logic is running")
    });

    /* 
	
    // Define a filter for the specific path and POST method
    let my_route = warp::path!("from_hass" / "post")
        .and(warp::post())
        .and(warp::body::json()) // Automatically deserialize JSON data
        .map(|data: DataHass| {
            // Print the received data to the console
            println!("Received data: {:?}", data);
            // Handle the received data
            warp::reply::json(&data) // Echo the received data as JSON
        });

    */
	
    // Print a message indicating that the server is starting
    println!("Server started at {}", ip_address);
    
    let vector: Vec<(String, f64)> = run_predicer();

    let brightness_values: Vec<f64> = vector.iter().map(|(_, value)| *value * 20.0).collect();
    print_f64_vector(&brightness_values);

    // Make a test POST call to the Home Assistant User Interface.
    println!("Make test POST call to the Home Assistant User Interface:");
    let url = "http://192.168.1.171:8123/api/services/light/turn_on";
    let entity_id = "light.katto1";
    
    
    for brightness in brightness_values {
        if let Err(err) = make_post_request_light(url, entity_id, hass_token, brightness).await {
            eprintln!("Error in making POST request for brightness {}: {:?}", brightness, err);
        }

        // Wait for 10 seconds before sending the next request
        time::sleep(Duration::from_secs(5)).await;
    }

    // Combine filters and start the warp server
    warp::serve(my_route).run(ip_address).await;
    
}