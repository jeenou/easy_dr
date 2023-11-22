use std::collections::HashMap;
use std::env;
mod predicer;
mod utilities;
mod input_data;
use hertta::julia_interface;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use serde_json::json;
//use tokio::time::{self, Duration};
//use std::net::SocketAddr;
//use std::fs;
//use warp::Filter;
use serde::{Deserialize, Serialize};
use serde_json;
//use tokio::sync::mpsc;
//use tokio::sync::Mutex;
//use std::sync::Arc;
use std::{num::NonZeroUsize, path::PathBuf};
use jlrs::prelude::*;
use predicer::RunPredicer;



pub fn create_data() -> input_data::InputData {

    let args: Vec<String> = env::args().collect();
    let predicer_dir = args
        .get(1)
        .expect("first argument should be path to Predicer");

    //Example time series

    let mut series1: Vec<(String, f64)> = Vec::new();
    let mut series2: Vec<(String, f64)> = Vec::new();

    let timepoint1 = input_data::create_time_point("Data1".to_string(), 0.0);
    let timepoint2 = input_data::create_time_point("Data2".to_string(), 0.0);

    input_data::add_time_point(&mut series1, timepoint1.clone());
    input_data::add_time_point(&mut series1, timepoint2.clone());
    input_data::add_time_point(&mut series2, timepoint1.clone());
    input_data::add_time_point(&mut series2, timepoint2.clone());

    let time_series1 = input_data::TimeSeries {
        scenario: "Scenario1".to_string(),
        series: series1,
    };

    let time_series2 = input_data::TimeSeries {
        scenario: "Scenario2".to_string(),
        series: series2,
    };

    // Step 2: Create a Vec<TimeSeries> containing the created TimeSeries instances
    let mut time_series_data_vec: Vec<input_data::TimeSeries> = Vec::new();
    input_data::add_time_serie(&mut time_series_data_vec, time_series1);
    input_data::add_time_serie(&mut time_series_data_vec, time_series2);


    // Step 3: Create a new TimeSeriesData instance with the Vec<TimeSeries>
    let time_series_data: input_data::TimeSeriesData = input_data::TimeSeriesData {
        ts_data: time_series_data_vec,
    };

    //Outside temperatures (time series)

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

    let outside_ts_s1 = input_data::TimeSeries {
        scenario: "s1".to_string(),
        series: outside_timeseries_s1,
    };

    let outside_ts_s2 = input_data::TimeSeries {
        scenario: "s2".to_string(),
        series: outside_timeseries_s2,
    };

    let mut outside_ts_vec: Vec<input_data::TimeSeries> = Vec::new();
    input_data::add_time_serie(&mut outside_ts_vec, outside_ts_s1);
    input_data::add_time_serie(&mut outside_ts_vec, outside_ts_s2);

    let outside_ts: input_data::TimeSeriesData = input_data::TimeSeriesData {
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

    let npe_ts_s1 = input_data::TimeSeries {
        scenario: "s1".to_string(),
        series: npe_timeseries_s1,
    };

    let npe_ts_s2 = input_data::TimeSeries {
        scenario: "s2".to_string(),
        series: npe_timeseries_s2,
    };

    let npe_ts_vec: Vec<input_data::TimeSeries> = vec![npe_ts_s1, npe_ts_s2];

    let npe_ts: input_data::TimeSeriesData = input_data::TimeSeriesData {
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

    let npe_up_s1 = input_data::TimeSeries {
        scenario: "s1".to_string(),
        series: npe_up_prices_s1,
    };

    let npe_up_s2 = input_data::TimeSeries {
        scenario: "s2".to_string(),
        series: npe_up_prices_s2,
    };

    let npe_up_vec: Vec<input_data::TimeSeries> = vec![npe_up_s1, npe_up_s2];

    let npe_up_ts: input_data::TimeSeriesData = input_data::TimeSeriesData {
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

    let npe_down_s1 = input_data::TimeSeries {
        scenario: "s1".to_string(),
        series: npe_down_prices_s1,
    };

    let npe_down_s2 = input_data::TimeSeries {
        scenario: "s2".to_string(),
        series: npe_down_prices_s2,
    };

    let npe_down_vec: Vec<input_data::TimeSeries> = vec![npe_down_s1, npe_down_s2];

    let npe_down_ts: input_data::TimeSeriesData = input_data::TimeSeriesData {
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

    let interiorair_up_s1 = input_data::TimeSeries {
        scenario: "s1".to_string(),
        series: c_interiorair_up_s1, 
    };

    let interiorair_up_s2 = input_data::TimeSeries {
        scenario: "s2".to_string(),
        series: c_interiorair_up_s2,
    };

    let interiorair_down_s1 = input_data::TimeSeries {
        scenario: "s1".to_string(),
        series: c_interiorair_down_s1, 
    };

    let interiorair_down_s2 = input_data::TimeSeries {
        scenario: "s2".to_string(),
        series: c_interiorair_down_s2,
    };

    let gc_interiorair_up_vec: Vec<input_data::TimeSeries> = vec![interiorair_up_s1, interiorair_up_s2];
    let gc_interiorair_down_vec: Vec<input_data::TimeSeries> = vec![interiorair_down_s1, interiorair_down_s2];

    let interiorair_up_ts: input_data::TimeSeriesData = input_data::TimeSeriesData {
        ts_data: gc_interiorair_up_vec,
    };

    let interiorair_down_ts: input_data::TimeSeriesData = input_data::TimeSeriesData {
        ts_data: gc_interiorair_down_vec,
    };

    //Creating node_diffusion

    let diffusion_1 = input_data::NodeDiffusion {
        name: String::from("diffusion_1"),
        node1: String::from("interiorair"),
        node2: String::from("buildingenvelope"),
        diff_coeff: 0.5,
    };

    let diffusion_2 = input_data::NodeDiffusion {
        name: String::from("diffusion_2"),
        node1: String::from("buildingenvelope"),
        node2: String::from("outside"),
        diff_coeff: 0.4,
    };

    //Creating node_delay

    let delay_1 = input_data::NodeDelay {
        name: String::from("delay_1"),
        node1: String::from("dh1"),
        node2: String::from("dh2"),
        delay: 2.0,
        min_flow: 0.0,
        max_flow: 20.0,
    };

    //Creating state

    let interiorair_state = input_data::State {

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

    //Creating nodes

    let _interiorair = input_data::Node {
        name: String::from("interiorair"),
        is_commodity: false,
        is_state: true,
        is_res: false,
        is_market: false,
        is_inflow: false,
        cost: time_series_data.clone(),
        inflow: time_series_data.clone(),
        state: interiorair_state,
    };

    let building_envelope_state = input_data::State {

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

    let _building_envelope_state = input_data::State {

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

    let _building_envelope = input_data::Node {
        name: String::from("buildingenvelope"),
        is_commodity: false,
        is_state: true,
        is_res: false,
        is_market: false,
        is_inflow: false,
        cost: time_series_data.clone(),
        inflow: time_series_data.clone(),
        state: building_envelope_state,
    };

    let outside_state = input_data::State {

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

    let _outside = input_data::Node {
        name: String::from("outside"),
        is_commodity: false,
        is_state: true,
        is_res: false,
        is_market: false,
        is_inflow: true,
        cost: time_series_data.clone(),
        inflow: outside_ts.clone(),
        state: outside_state,
    };

    let empty_state: input_data::State = Default::default();

    let _electricitygrid = input_data::Node {
        name: String::from("electricitygrid"),
        is_commodity: false,
        is_state: false,
        is_res: false,
        is_market: false,
        is_inflow: false,
        cost: time_series_data.clone(),
        inflow: time_series_data.clone(),
        state: empty_state,
    };

    let _node_history_1 = input_data::NodeHistory {
        node: String::from("electricitygrid"),
        steps: time_series_data.clone(),
    };

    let mut _nodes: HashMap<String, input_data::Node> = HashMap::new();
    let mut _node_diffusion: HashMap<String, input_data::NodeDiffusion> = HashMap::new();
    let mut _node_delay: HashMap<String, input_data::NodeDelay> = HashMap::new();

    _nodes.insert(_interiorair.name.clone(), _interiorair.clone());
    _nodes.insert(_building_envelope.name.clone(), _building_envelope.clone());
    _nodes.insert(_outside.name.clone(), _outside.clone());
    _nodes.insert(_electricitygrid.name.clone(), _electricitygrid.clone());

    _node_diffusion.insert(diffusion_1.name.clone(), diffusion_1.clone());
    _node_diffusion.insert(diffusion_2.name.clone(), diffusion_2.clone());

    _node_delay.insert(delay_1.name.clone(), delay_1.clone());

    let mut _processes: HashMap<String, input_data::Process> = HashMap::new();

    //Creating topology for processes

    let topology1 = input_data::Topology {
        source: String::from("electricitygrid"),
        sink: String::from("electricheater"),
        capacity: 7.5,
        vom_cost: 0.0,
        ramp_up: 1.0,
        ramp_down: 1.0,
    };

    let topology2 = input_data::Topology {
        source: String::from("electricheater"),
        sink: String::from("interiorair"),
        capacity: 7.5,
        vom_cost: 0.0,
        ramp_up: 1.0,
        ramp_down: 1.0,
    };

    let topo_vec: Vec<input_data::Topology> = vec![topology1, topology2];

    //Creating process

    let process_vec: Vec<String> = vec![("eff_ops".to_string())];

    let _electricheater1 = input_data::Process {
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
        topos: topo_vec.clone(),
        eff_ops: process_vec.clone(),
    };

    _processes.insert(_electricheater1.name.clone(), _electricheater1.clone());

    let mut _markets: HashMap<String, input_data::Market> = HashMap::new();
    let mut _groups: HashMap<String, input_data::Group> = HashMap::new();
    let mut _genconstraints: HashMap<String, input_data::GenConstraint> = HashMap::new();

    let _npe = input_data::Market {
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
        price: npe_ts.clone(),
        up_price: npe_up_ts.clone(),
        down_price: npe_down_ts.clone(),
    };

    _markets.insert(_npe.name.clone(), _npe.clone());

    let _p1 = input_data::Group {
        name: String::from("p1"),
        g_type: String::from("process"),
        entity: String::from("electricheater"),
    };

    _groups.insert(_p1.name.clone(), _p1.clone());

    let interiorair_up_cf = input_data::ConFactor {
        var_type: String::from("state"),
        flow: (String::from("interiorair"), String::from("")),
        data: interiorair_up_ts.clone(),
    };

    let interiorair_up_cf_vec: Vec<input_data::ConFactor> = vec![interiorair_up_cf];

    let interiorair_down_cf = input_data::ConFactor {
        var_type: String::from("state"),
        flow: (String::from("interiorair"), String::from("")),
        data: interiorair_down_ts.clone(),
    };

    let interiorair_down_cf_vec: Vec<input_data::ConFactor> = vec![interiorair_down_cf];

    let _c_interiorair_up = input_data::GenConstraint {
        name: String::from("c_interiorair_up"),
        gc_type: String::from("st"),
        is_setpoint: true,
        penalty: 1000.0,
        factors: interiorair_up_cf_vec.clone(),
        constant: time_series_data.clone(),
    };

    let _c_interiorair_down = input_data::GenConstraint {
        name: String::from("c_interiorair_down"),
        gc_type: String::from("gt"),
        is_setpoint: true,
        penalty: 1000.0,
        factors: interiorair_down_cf_vec.clone(),
        constant: time_series_data.clone(),
    };

    _genconstraints.insert(_c_interiorair_up.name.clone(), _c_interiorair_up.clone());
    _genconstraints.insert(_c_interiorair_down.name.clone(), _c_interiorair_down.clone());

    let mut _solution: Vec<(String, f64)> = Vec::new();
    
    
     
    let data = input_data::InputData {
        contains_reserves: false,
        contains_online: false,
        contains_state: true,
        contains_piecewise_eff: false,
        contains_risk: false,
        contains_delay: false,
        contains_diffusion: true,
        nodes: _nodes,
        processes: _processes,
        markets: _markets,
        groups: _groups,
        gen_constraints: _genconstraints,
        node_diffusion: _node_diffusion,
        node_delay: _node_delay,
    };

    return data

}


fn _print_tuple_vector(vec: &Vec<(String, f64)>) {
    for (s, num) in vec {
        println!("{}: {}", s, num);
    }
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

/* 
async fn _run_logic(hass_token: String, tx: mpsc::Sender<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting logic execution...");
    let vector: Vec<(String, f64)> = run_predicer();

    let brightness_values: Vec<f64> = vector.iter().map(|(_, value)| *value * 20.0).collect();

    println!("Results obtained.");

    let url = "http://192.168.1.171:8123/api/services/light/turn_on";
    let entity_id = "light.katto1";
    
    for brightness in brightness_values {
        println!("Setting brightness to: {}", brightness);
        if let Err(err) = make_post_request_light(url, entity_id, &hass_token, brightness).await {
            eprintln!("Error in making POST request for brightness {}: {:?}", brightness, err);
        } else {
            println!("POST request successful for brightness: {}", brightness);
        }

        // Wait for 5 seconds before sending the next request
        println!("Waiting for 5 seconds before next request...");
        time::sleep(Duration::from_secs(2)).await;
    }

    println!("Completed.");

    // Notify the main thread that the logic is complete
    match tx.send("Logic complete".to_owned()).await {
        Ok(_) => println!("Notification sent successfully."),
        Err(e) => eprintln!("Failed to send notification: {:?}", e),
    }

    Ok(())

}
*/

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

#[tokio::main]
async fn main() {

    let data = create_data();

    let args: Vec<String> = env::args().collect();
    let predicer_dir = args
        .get(1)
        .expect("first argument should be path to Predicer");

    let (julia, handle) = unsafe {
        RuntimeBuilder::new()
            .async_runtime::<Tokio>()
            .channel_capacity(NonZeroUsize::new(4).unwrap())
            .start_async::<1>()
            .expect("Could not init Julia")
    };

    {
        // Include the custom code MyTask needs by registering it.
        let (sender, receiver) = tokio::sync::oneshot::channel();
        julia
            .register_task::<RunPredicer, _>(sender)
            .dispatch_any()
            .await;
        receiver.await.unwrap().unwrap();
    }

    // Send task to the runtime.
    let (sender1, receiver1) = tokio::sync::oneshot::channel();

    julia
        .task(
            RunPredicer {
                data: data,
                predicer_dir: predicer_dir.clone(),
            },
            sender1,
        )
        .dispatch_any()
        .await;

    // Receive the results of the tasks.
    let res1 = receiver1.await.unwrap().unwrap();

    for (str_val, num_val) in &res1 {
        println!("{}: {}", str_val, num_val);
    }

    // Dropping `julia` causes the runtime to shut down Julia and itself if it was the final
    // handle to the runtime. Await the runtime handle handle to wait for everything to shut
    // down cleanly.
    std::mem::drop(julia);
    handle
        .await
        .expect("Julia exited with an error")
        .expect("The runtime thread panicked");
    
}

