use lbflow::prelude::*;

fn main() {
    let n = vec![10_usize, 10_usize];
    let m_params = m::Parameters {
        n: n.clone(),
        node_types: functions::only_fluid_nodes(n.clone()),
        velocity_set: D2Q9,
        collision_operator: BGK(0.5_f64),
        delta_x: 0.001,
        delta_t: 0.001,
        physical_density: 998,
        reference_pressure: 101325,
        initial_density: functions::uniform_density(1_f64, n.clone()),
        initial_velocity: functions::uniform_velocity(vec![0_f64, 0_f64], n.clone()),
        boundary_conditions: vec![(West, m::bc::NoSlip), (East, m::bc::NoSlip), (South, m::bc::NoSlip), (North, m::bc::NoSlip)],
        force: None,
        multiphase_parameters: None,
        post_functions: None,
    };

    m::solve(m_params);
}
