use crate::structures::structures;

//Input data: contains_reserves
//Boolean arvon voi vain muuttaa julia-booleaniksi ja lähettää lopulta input_data:n tekoon soveltuvalle funktiolle
pub fn contains_reserves(nodes: &HashMap<String, Node>) -> bool {
    if nodes.is_empty() {
        // Perform desired action when the HashMap is empty
        // For example, return false or throw an error
        return false;
    }

    for (_, value) in nodes.iter() {
        if value.is_res {
            return true;
        }
    }

    false
}

//Input data: contains_online
pub fn contains_online(processes: &HashMap<String, Process>) -> bool {
    if processes.is_empty() {
        // Perform desired action when the HashMap is empty
        // For example, return false or throw an error
        return false;
    }

    for (_, value) in processes.iter() {
        if value.is_online {
            return true;
        }
    }

    false
}

//Input data: contains_states
pub fn contains_states(nodes: &HashMap<String, Node>) -> bool {
    if nodes.is_empty() {
        // Perform desired action when the HashMap is empty
        // For example, return false or throw an error
        return false;
    }

    for (_, value) in nodes.iter() {
        if value.is_state {
            return true;
        }
    }

    false
}

//Input data: contains_piecewise_eff
pub fn contains_piecewise_eff(processes: &HashMap<String, Process>) -> bool {
    if processes.is_empty() {
        return true;
    }

    for (_, value) in processes.iter() {
        if !value.eff_ops.is_empty() {
            return false;
        }
    }

    true
}

/*
Input data: contains_risk
pub fn contains_risk() {

}
*/

//Input data: contains_delay
pub fn contains_delay(processes: &HashMap<String, Process>) -> bool {
    if processes.is_empty() {
        return false;
    }

    for (_, value) in processes.iter() {
        if value.delay != 0.0 {
            return true;
        }
    }

    false
}

//Input data: contains_diffusion MUOKKAA TÄMÄ
pub fn contains_delay(processes: &HashMap<String, Process>) -> bool {
    if processes.is_empty() {
        return false;
    }

    for (_, value) in processes.iter() {
        if value.delay != 0.0 {
            return true;
        }
    }

    false
}

//Processes

pub fn processes() {
    
}

//Nodes

//Node diffusion tuples

//Markets

//Groups

//Scenarios

//Reserve-type

//Risk

//Inflow blocks

//Gen constraints
