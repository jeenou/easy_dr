
use std::collections::HashMap;

struct _InputData<'a> {
    contains_reserves: bool,
    contains_online: bool,
    contains_state: bool,
    contains_piecewise_eff: bool,
    contains_risk: bool,
    contains_delay: bool,
    contains_diffusion: bool,
    nodes: HashMap<&'a String, &'a Node<'a>>,
    processes: HashMap<&'a String, &'a Process<'a>>,
    markets: HashMap<&'a String, &'a Market<'a>>,
    groups: HashMap<&'a String, &'a Group>,
    gen_constraints: HashMap<&'a String, &'a GenConstraint<'a>>,
    node_diffusion: HashMap<&'a String, &'a NodeDiffusion>,
    node_delay: HashMap<&'a String, &'a NodeDelay>,
}

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


pub fn create_time_point(string: String, number: f64) -> (String, f64) {

    return (string, number)

}

pub fn add_time_point(ts_vec: &mut Vec<(String, f64)>, time_point: (String, f64)) {

    ts_vec.push(time_point);

}

pub fn add_time_serie(ts_data_vec: &mut Vec<TimeSeries>, time_series: TimeSeries) {

    ts_data_vec.push(time_series);

}