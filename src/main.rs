use std::collections::HashMap;
use std::env;
use std::sync::mpsc::{channel, Sender};
mod predicer;
use hertta::julia_interface;
use hertta::utilities;
use hertta::main_loop;

pub fn start_sending(tx: Sender<main_loop::_Task>) {
    tx.send(main_loop::_Task::StartProcess).unwrap();
    tx.send(main_loop::_Task::QuitProcess).unwrap();
    tx.send(main_loop::_Task::StartProcess).unwrap();
}

pub fn run_predicer() {

    let args: Vec<String> = env::args().collect();
    let predicer_dir = args
        .get(1)
        .expect("first argument should be path to Predicer");

    //Create timeseries

    /*
    TimeSeries(
"s1", Tuple{AbstractString, Number}[
("2022-04-20T00:00:00+00:00", 3), 
("2022-04-20T01:00:00+00:00", 0), 
("2022-04-20T02:00:00+00:00", 4), 
("2022-04-20T03:00:00+00:00", -1),
("2022-04-20T04:00:00+00:00", 5), 
("2022-04-20T05:00:00+00:00", -4), 
("2022-04-20T06:00:00+00:00", -5), 
("2022-04-20T07:00:00+00:00", -2), 
("2022-04-20T08:00:00+00:00", 4), 
("2022-04-20T09:00:00+00:00", 0)])
     */

    let series1: Vec<(String, i64)> = vec![
        ("Data1".to_string(), 0),
        ("Data2".to_string(), 0),
    ];

    let outside_timeseries_s1: Vec<(String, i64)> = vec![
        ("2022-04-20T00:00:00+00:00".to_string(), 3),
        ("2022-04-20T01:00:00+00:00".to_string(), 0),
        ("2022-04-20T02:00:00+00:00".to_string(), 4),
        ("2022-04-20T03:00:00+00:00".to_string(), -1),
        ("2022-04-20T04:00:00+00:00".to_string(), 5),
        ("2022-04-20T05:00:00+00:00".to_string(), -4),
        ("2022-04-20T06:00:00+00:00".to_string(), -5),
        ("2022-04-20T07:00:00+00:00".to_string(), -2),
        ("2022-04-20T08:00:00+00:00".to_string(), 4),
        ("2022-04-20T09:00:00+00:00".to_string(), 0),
    ];

    let outside_timeseries_s2: Vec<(String, i64)> = vec![
        ("2022-04-20T00:00:00+00:00".to_string(), -2),
        ("2022-04-20T01:00:00+00:00".to_string(), 4),
        ("2022-04-20T02:00:00+00:00".to_string(), 4),
        ("2022-04-20T03:00:00+00:00".to_string(), -1),
        ("2022-04-20T04:00:00+00:00".to_string(), 1),
        ("2022-04-20T05:00:00+00:00".to_string(), -3),
        ("2022-04-20T06:00:00+00:00".to_string(), 0),
        ("2022-04-20T07:00:00+00:00".to_string(), -5),
        ("2022-04-20T08:00:00+00:00".to_string(), -3),
        ("2022-04-20T09:00:00+00:00".to_string(), -2),
    ];

    let outside_ts_s1 = predicer::TimeSeries {
        scenario: "s1".to_string(),
        series: outside_timeseries_s1,
    };

    let outside_ts_s2 = predicer::TimeSeries {
        scenario: "s2".to_string(),
        series: outside_timeseries_s2,
    };

    let series2: Vec<(String, i64)> = vec![
        ("Data3".to_string(), 0),
        ("Data4".to_string(), 0),
    ];

    let time_series1 = predicer::TimeSeries {
        scenario: "Scenario1".to_string(),
        series: series1,
    };

    let time_series2 = predicer::TimeSeries {
        scenario: "Scenario2".to_string(),
        series: series2,
    };


    // Step 2: Create a Vec<TimeSeries> containing the created TimeSeries instances
    let time_series_data_vec: Vec<predicer::TimeSeries> = vec![time_series1, time_series2];

    // Step 3: Create a new TimeSeriesData instance with the Vec<TimeSeries>
    let time_series_data: predicer::TimeSeriesData = predicer::TimeSeriesData {
        ts_data: time_series_data_vec,
    };

    let outside_ts_vec: Vec<predicer::TimeSeries> = vec![outside_ts_s1, outside_ts_s2];

    let outside_ts: predicer::TimeSeriesData = predicer::TimeSeriesData {
        ts_data: outside_ts_vec,
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

    //Mitä eff_ops sisältää?

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
    };

    _markets.insert(&_npe.name, &_npe);

    let _p1 = predicer::Group {
        name: String::from("p1"),
        g_type: String::from("process"),
        entity: String::from("electricheater"),
    };

    /*Key: c_interiorair_up,
        Value: GenConstraint("c_interiorair_up", "st", true, 1000.0,
        ConFactor[ConFactor("state", ("interiorair", ""),
        Predicer.TimeSeriesData(TimeSeries[TimeSeries("s1", Tuple{AbstractString, Number}[("2022-04-20T00:00:00+00:00", 298.15), ("2022-04-20T01:00:00+00:00", 298.15), ("2022-04-20T02:00:00+00:00", 298.15), ("2022-04-20T03:00:00+00:00", 298.15), ("2022-04-20T04:00:00+00:00", 298.15), ("2022-04-20T05:00:00+00:00", 298.15), ("2022-04-20T06:00:00+00:00", 298.15), ("2022-04-20T07:00:00+00:00", 298.15), ("2022-04-20T08:00:00+00:00", 298.15), ("2022-04-20T09:00:00+00:00", 298.15)]),
    TimeSeries("s2", Tuple{AbstractString, Number}[("2022-04-20T00:00:00+00:00", 298.15), ("2022-04-20T01:00:00+00:00", 298.15), ("2022-04-20T02:00:00+00:00", 298.15), ("2022-04-20T03:00:00+00:00", 298.15), ("2022-04-20T04:00:00+00:00", 298.15), ("2022-04-20T05:00:00+00:00", 298.15), ("2022-04-20T06:00:00+00:00", 298.15), ("2022-04-20T07:00:00+00:00", 298.15), ("2022-04-20T08:00:00+00:00", 298.15), ("2022-04-20T09:00:00+00:00", 298.15)])]))], Predicer.TimeSeriesData(TimeSeries[])) */

    _groups.insert(&_p1.name, &_p1);

    let _confactor = predicer::ConFactor {
        var_type: String::from(""),
        flow: (String::from(""), String::from("")),
        data: &time_series_data,
    };

    let genconstraint_vec: Vec<predicer::ConFactor> = vec![_confactor];

    let _c_interiorair_up = predicer::GenConstraint {
        name: String::from("c_interiorair_up"),
        gc_type: String::from("st"),
        is_setpoint: true,
        penalty: 1000.0,
        factors: genconstraint_vec,
        constant: &time_series_data,
    };

    _genconstraints.insert(&_c_interiorair_up.name, &_c_interiorair_up);

    predicer::_predicer(
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

}

fn main() {

    run_predicer();


    /*
    let (tx, rx) = channel();
    let mut children = Vec::new();
    start_sending(tx);
    main_loop::_task_loop(rx);
    */
}
