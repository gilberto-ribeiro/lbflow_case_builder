pub(crate) mod momentum;
pub(crate) mod passive_scalar;

use eframe::egui;

pub(crate) trait LatticeGuiConfig {
    fn get_velocity_set_gui(&mut self) -> &mut VelocitySetGui;

    fn get_collision_operator_gui(&mut self) -> &mut CollisionOperatorGui;

    fn ui_collision_operator(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Collision operator:");
            let current_tau = match &self.get_collision_operator_gui() {
                CollisionOperatorGui::BGK { tau } => *tau,
                _ => 0.5,
            };
            let (current_omega_plus, current_omega_minus) = match &self.get_collision_operator_gui()
            {
                CollisionOperatorGui::TRT {
                    omega_plus,
                    omega_minus,
                } => (*omega_plus, *omega_minus),
                _ => (0.5, 0.5),
            };
            ui.selectable_value(
                self.get_collision_operator_gui(),
                CollisionOperatorGui::BGK { tau: current_tau },
                "BGK",
            );
            ui.selectable_value(
                self.get_collision_operator_gui(),
                CollisionOperatorGui::TRT {
                    omega_plus: current_omega_plus,
                    omega_minus: current_omega_minus,
                },
                "TRT",
            );
            ui.selectable_value(
                self.get_collision_operator_gui(),
                CollisionOperatorGui::MRT,
                "MRT",
            );
        });
        ui.horizontal(|ui| match self.get_collision_operator_gui() {
            CollisionOperatorGui::BGK { tau } => {
                ui.label("Tau:");
                ui.add(egui::DragValue::new(tau).speed(0.01).range(0.0..=10.0));
            }
            CollisionOperatorGui::TRT {
                omega_plus,
                omega_minus,
            } => {
                ui.label("Omega +:");
                ui.add(
                    egui::DragValue::new(omega_plus)
                        .speed(0.01)
                        .range(0.0..=2.0),
                );
                ui.label("Omega -:");
                ui.add(
                    egui::DragValue::new(omega_minus)
                        .speed(0.01)
                        .range(0.0..=2.0),
                );
            }
            CollisionOperatorGui::MRT => {
                ui.label("Set the MRT parameters in the generated code.");
            }
        });
    }

    fn ui_velocity_set(&mut self, ui: &mut egui::Ui, dim: Dimensionality) {
        ui.horizontal(|ui| {
            ui.label("Velocity set:");
            match dim {
                Dimensionality::D2 => {
                    ui.selectable_value(self.get_velocity_set_gui(), VelocitySetGui::D2Q9, "D2Q9");
                }
                Dimensionality::D3 => {
                    ui.selectable_value(
                        self.get_velocity_set_gui(),
                        VelocitySetGui::D3Q15,
                        "D3Q15",
                    );
                    ui.selectable_value(
                        self.get_velocity_set_gui(),
                        VelocitySetGui::D3Q19,
                        "D3Q19",
                    );
                    ui.selectable_value(
                        self.get_velocity_set_gui(),
                        VelocitySetGui::D3Q27,
                        "D3Q27",
                    );
                }
            };
        });
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub(crate) enum Dimensionality {
    D2,
    D3,
}

#[derive(PartialEq, Eq)]
pub(crate) enum VelocitySetGui {
    D2Q9,
    D3Q15,
    D3Q19,
    D3Q27,
}

#[derive(PartialEq)]
pub(crate) enum CollisionOperatorGui {
    BGK { tau: f64 },
    TRT { omega_plus: f64, omega_minus: f64 },
    MRT,
}

#[derive(PartialEq, Clone, Copy)]
pub(crate) enum BoundaryFaceGui {
    West,
    East,
    South,
    North,
    Bottom,
    Top,
}

impl BoundaryFaceGui {
    pub(crate) fn to_literal(&self) -> String {
        match self {
            BoundaryFaceGui::West => "West".to_string(),
            BoundaryFaceGui::East => "East".to_string(),
            BoundaryFaceGui::South => "South".to_string(),
            BoundaryFaceGui::North => "North".to_string(),
            BoundaryFaceGui::Bottom => "Bottom".to_string(),
            BoundaryFaceGui::Top => "Top".to_string(),
        }
    }
}

#[derive(PartialEq, Eq)]
pub(crate) enum NodeTypesGui {
    FromBounceBackMapFile,
    OnlyFluidNodes,
}

pub(crate) struct CargoGuiConfig {
    pub(crate) case_name: String,
    pub(crate) commit_hash: String,
}

impl Default for CargoGuiConfig {
    fn default() -> Self {
        CargoGuiConfig {
            case_name: "case_000".to_string(),
            commit_hash: String::new(),
        }
    }
}

impl CargoGuiConfig {
    fn get_commit_hash_literal(&self) -> String {
        if self.commit_hash.is_empty() {
            "".to_string()
        } else {
            format!(", rev = \"{}\"", self.commit_hash)
        }
    }
}

impl CargoGuiConfig {
    pub fn get_cargo_toml(&self) -> String {
        let case_name = &self.case_name;
        let commit_hash_literal = self.get_commit_hash_literal();
        format!(
            r#"[package]
name = "{case_name}"
version = "0.1.0"
edition = "2024"

[dependencies]
lbflow = {{ version = "0.1.0", git = "https://github.com/gilberto-ribeiro/lbflow.git"{commit_hash_literal} }}
"#
        )
    }
}
