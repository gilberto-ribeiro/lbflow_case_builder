use super::LatticeGuiConfig;
use super::{BoundaryFaceGui, CollisionOperatorGui, Dimensionality, NodeTypesGui, VelocitySetGui};

#[derive(PartialEq)]
pub(crate) enum BoundaryConditionGui {
    NoSlip,
    BounceBack { rho: f64, ux: f64, uy: f64, uz: f64 },
    AntiBounceBack { rho: f64 },
    Periodic,
}

impl BoundaryConditionGui {
    fn to_literal(&self, dim: &Dimensionality) -> String {
        match self {
            BoundaryConditionGui::NoSlip => "m::bc::NoSlip".to_string(),
            BoundaryConditionGui::BounceBack { rho, ux, uy, uz } => {
                let velocity_vec = match dim {
                    Dimensionality::D2 => format!("vec![{}_f64, {}_f64]", ux, uy),
                    Dimensionality::D3 => format!("vec![{}_f64, {}_f64, {}_f64]", ux, uy, uz),
                };
                format!(
                    "m::bc::BounceBack {{ density: {}_f64, velocity: {} }}",
                    rho, velocity_vec
                )
            }
            BoundaryConditionGui::AntiBounceBack { rho } => {
                format!("m::bc::AntiBounceBack {{ density: {}_f64 }}", rho)
            }
            BoundaryConditionGui::Periodic => "m::bc::Periodic".to_string(),
        }
    }
}

pub(crate) struct FaceBC {
    pub(crate) boundary_face: BoundaryFaceGui,
    pub(crate) boundary_condition: BoundaryConditionGui,
}

#[derive(PartialEq)]
pub(crate) enum InitialDensityGui {
    Uniform { rho: f64 },
    FromTimeStep { time_step: usize },
    FromFile { file_path: String },
}

#[derive(PartialEq)]
pub(crate) enum InitialVelocityGui {
    Uniform { ux: f64, uy: f64, uz: f64 },
    FromTimeStep { time_step: usize },
    FromFile { file_path: String },
}

pub struct GuiConfig {
    pub(crate) dim: Dimensionality,
    pub(crate) n: [usize; 3],
    pub(crate) collision_operator: CollisionOperatorGui,
    pub(crate) velocity_set: VelocitySetGui,
    pub(crate) delta_x: f64,
    pub(crate) delta_t: f64,
    pub(crate) physical_density: f64,
    pub(crate) reference_pressure: f64,
    pub(crate) initial_density: InitialDensityGui,
    pub(crate) initial_velocity: InitialVelocityGui,
    pub(crate) boundary_conditions: Vec<FaceBC>,
    pub(crate) node_types: NodeTypesGui,
}

impl Default for GuiConfig {
    fn default() -> Self {
        GuiConfig {
            dim: Dimensionality::D2,
            n: [10, 10, 1],
            collision_operator: CollisionOperatorGui::BGK { tau: 0.5 },
            velocity_set: VelocitySetGui::D2Q9,
            delta_x: 0.001,
            delta_t: 0.001,
            physical_density: 998.0,
            reference_pressure: 101325.0,
            initial_density: InitialDensityGui::Uniform { rho: 1.0 },
            initial_velocity: InitialVelocityGui::Uniform {
                ux: 0.0,
                uy: 0.0,
                uz: 0.0,
            },
            boundary_conditions: vec![
                FaceBC {
                    boundary_face: BoundaryFaceGui::West,
                    boundary_condition: BoundaryConditionGui::NoSlip,
                },
                FaceBC {
                    boundary_face: BoundaryFaceGui::East,
                    boundary_condition: BoundaryConditionGui::NoSlip,
                },
                FaceBC {
                    boundary_face: BoundaryFaceGui::South,
                    boundary_condition: BoundaryConditionGui::NoSlip,
                },
                FaceBC {
                    boundary_face: BoundaryFaceGui::North,
                    boundary_condition: BoundaryConditionGui::NoSlip,
                },
            ],
            node_types: NodeTypesGui::OnlyFluidNodes,
        }
    }
}

impl GuiConfig {
    pub(crate) fn get_n_literal(&self) -> String {
        match self.dim {
            Dimensionality::D2 => format!("vec![{}_usize, {}_usize]", self.n[0], self.n[1]),
            Dimensionality::D3 => format!(
                "vec![{}_usize, {}_usize, {}_usize]",
                self.n[0], self.n[1], self.n[2]
            ),
        }
    }

    fn get_collision_operator_literal(&self) -> String {
        match &self.collision_operator {
            CollisionOperatorGui::BGK { tau } => format!("BGK({}_f64)", tau),
            CollisionOperatorGui::TRT {
                omega_plus,
                omega_minus,
            } => format!("TRT({}_f64, {}_f64)", omega_plus, omega_minus),
            CollisionOperatorGui::MRT => "MRT(vec![todo!(\"Insert parameters\")])".to_string(),
        }
    }

    fn get_velocity_set_literal(&self) -> String {
        match self.velocity_set {
            VelocitySetGui::D2Q9 => "D2Q9".to_string(),
            VelocitySetGui::D3Q15 => "D3Q15".to_string(),
            VelocitySetGui::D3Q19 => "D3Q19".to_string(),
            VelocitySetGui::D3Q27 => "D3Q27".to_string(),
        }
    }

    fn get_initial_density_literal(&self) -> String {
        match &self.initial_density {
            InitialDensityGui::Uniform { rho } => {
                format!("functions::uniform_density({}_f64, n.clone())", rho)
            }
            InitialDensityGui::FromTimeStep { time_step } => {
                format!("functions::density_from_time_step({}_usize)", time_step)
            }
            InitialDensityGui::FromFile { file_path } => {
                format!("functions::from_density_file(\"{}\")", file_path)
            }
        }
    }

    fn get_initial_velocity_literal(&self) -> String {
        match &self.initial_velocity {
            InitialVelocityGui::Uniform { ux, uy, uz } => match self.dim {
                Dimensionality::D2 => {
                    format!(
                        "functions::uniform_velocity(vec![{}_f64, {}_f64], n.clone())",
                        ux, uy
                    )
                }
                Dimensionality::D3 => {
                    format!(
                        "functions::uniform_velocity(vec![{}_f64, {}_f64, {}_f64], n.clone())",
                        ux, uy, uz
                    )
                }
            },
            InitialVelocityGui::FromTimeStep { time_step } => {
                format!("functions::velocity_from_time_step({}_usize)", time_step)
            }
            InitialVelocityGui::FromFile { file_path } => {
                format!("functions::from_velocity_file(\"{}\")", file_path)
            }
        }
    }

    fn get_node_types_literal(&self) -> String {
        match self.node_types {
            NodeTypesGui::OnlyFluidNodes => "functions::only_fluid_nodes(n.clone())".to_string(),
            NodeTypesGui::FromBounceBackMapFile => {
                "functions::from_bounce_back_map_file()".to_string()
            }
        }
    }

    fn get_boundary_conditions_literal(&self) -> String {
        let mut boundary_conditions_literals = vec![];
        for face_bc in &self.boundary_conditions {
            let boundary_face_literal = face_bc.boundary_face.to_literal();
            let boundary_condition_literal = face_bc.boundary_condition.to_literal(&self.dim);
            boundary_conditions_literals.push(format!(
                "({}, {})",
                boundary_face_literal, boundary_condition_literal
            ));
        }
        format!("vec![{}]", boundary_conditions_literals.join(", "))
    }
}

impl LatticeGuiConfig for GuiConfig {
    fn get_velocity_set_gui(&mut self) -> &mut VelocitySetGui {
        &mut self.velocity_set
    }

    fn get_collision_operator_gui(&mut self) -> &mut CollisionOperatorGui {
        &mut self.collision_operator
    }
}

impl GuiConfig {
    pub(crate) fn get_m_params_content(&self) -> String {
        let collision_operator_literal = self.get_collision_operator_literal();
        let velocity_set_literal = self.get_velocity_set_literal();
        let initial_density_literal = self.get_initial_density_literal();
        let initial_velocity_literal = self.get_initial_velocity_literal();
        let boundary_conditions_literal = self.get_boundary_conditions_literal();
        let node_types_literal = self.get_node_types_literal();
        let delta_x_literal = format!("{}_f64", self.delta_x);
        let delta_t_literal = format!("{}_f64", self.delta_t);
        let physical_density_literal = format!("{}_f64", self.physical_density);
        let reference_pressure_literal = format!("{}_f64", self.reference_pressure);
        format!(
            r#"    let m_params = m::Parameters {{
        n: n.clone(),
        node_types: {node_types_literal},
        velocity_set: {velocity_set_literal},
        collision_operator: {collision_operator_literal},
        delta_x: {delta_x_literal},
        delta_t: {delta_t_literal},
        physical_density: {physical_density_literal},
        reference_pressure: {reference_pressure_literal},
        initial_density: {initial_density_literal},
        initial_velocity: {initial_velocity_literal},
        boundary_conditions: {boundary_conditions_literal},
        force: None,
        multiphase_parameters: None,
        post_functions: None,
    }};
"#
        )
    }
}
