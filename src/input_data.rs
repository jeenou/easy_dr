
use std::collections::HashMap;

#[derive(Clone)]
pub struct InputData {
    pub contains_reserves: bool,
    pub contains_online: bool,
    pub contains_state: bool,
    pub contains_piecewise_eff: bool,
    pub contains_risk: bool,
    pub contains_delay: bool,
    pub contains_diffusion: bool,
    pub nodes: HashMap<String, Node>,
    pub processes: HashMap<String, Process>,
    pub markets: HashMap<String, Market>,
    pub groups: HashMap<String, Group>,
    pub gen_constraints: HashMap<String, GenConstraint>,
    pub node_diffusion: HashMap<String, NodeDiffusion>,
    pub node_delay: HashMap<String, NodeDelay>,
}

#[derive(Clone)]
pub struct Process {
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
    pub topos: Vec<Topology>,
    pub eff_ops: Vec<String>,
}

#[derive(Clone)]
pub struct Node {
    pub name: String,
    pub is_commodity: bool,
    pub is_state: bool,
    pub is_res: bool,
    pub is_market: bool,
    pub is_inflow: bool,
    pub cost: TimeSeriesData,
    pub inflow: TimeSeriesData,
    pub state: State,
}

#[derive(Clone)]
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
    pub price: TimeSeriesData,
    pub up_price: TimeSeriesData,
    pub down_price: TimeSeriesData,
}

#[derive(Clone)]
pub struct Group {
    pub name: String,
    pub g_type: String,
    pub entity: String,
}

#[derive(Clone)]
pub struct NodeDiffusion {
    pub name: String,
    pub node1: String,
    pub node2: String,
    pub diff_coeff: f64,
}


#[derive(Clone)]
pub struct NodeDelay {
    pub name: String,
    pub node1: String,
    pub node2: String,
    pub delay: f64,
    pub min_flow: f64,
    pub max_flow: f64,
}

#[derive(Clone)]
pub struct Topology {
    pub source: String,
    pub sink: String,
    pub capacity: f64,
    pub vom_cost: f64,
    pub ramp_up: f64,
    pub ramp_down: f64,
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct TimeSeries {
    pub scenario: String,
    pub series: Vec<(String, f64)>,
}

#[derive(Clone)]
pub struct TimeSeriesData {
    pub ts_data: Vec<TimeSeries>,
}

#[derive(Clone)]
pub struct ConFactor {
    pub var_type: String,
    pub flow: (String, String),
    pub data: TimeSeriesData,
}

#[derive(Clone)]
pub struct NodeHistory {
    pub node: String,
    pub steps: TimeSeriesData,
}

#[derive(Clone)]
pub struct GenConstraint {
    pub name: String,
    pub gc_type: String,
    pub is_setpoint: bool,
    pub penalty: f64,
    pub factors: Vec<ConFactor>,
    pub constant: TimeSeriesData,
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