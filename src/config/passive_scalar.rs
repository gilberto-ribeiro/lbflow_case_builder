use super::LatticeGuiConfig;
use super::{BoundaryFaceGui, CollisionOperatorGui, Dimensionality, VelocitySetGui};
use eframe::egui;

#[derive(PartialEq)]
pub(crate) enum BoundaryConditionGui {
    AntiBounceBack { value: f64 },
    AntiBBNoFlux,
    Periodic,
}

impl BoundaryConditionGui {
    fn to_literal(&self) -> String {
        match self {
            BoundaryConditionGui::AntiBounceBack { value } => {
                format!("ps::bc::AntiBounceBack {{ scalar_value: {}_f64 }}", value)
            }
            BoundaryConditionGui::AntiBBNoFlux => "ps::bc::AntiBBNoFlux".to_string(),
            BoundaryConditionGui::Periodic => "ps::bc::Periodic".to_string(),
        }
    }
}

pub(crate) struct FaceBC {
    pub(crate) boundary_face: BoundaryFaceGui,
    pub(crate) boundary_condition: BoundaryConditionGui,
}

#[derive(PartialEq)]
pub(crate) enum InitialScalarValueGui {
    Uniform { value: f64 },
    FromFile { file_path: String },
}

pub struct GuiConfig {
    pub(crate) scalar_name: String,
    pub(crate) collision_operator: CollisionOperatorGui,
    pub(crate) velocity_set: VelocitySetGui,
    pub(crate) initial_scalar_value: InitialScalarValueGui,
    pub(crate) boundary_conditions: Vec<FaceBC>,
}

impl Default for GuiConfig {
    fn default() -> Self {
        GuiConfig {
            scalar_name: String::new(),
            collision_operator: CollisionOperatorGui::BGK { tau: 0.5 },
            velocity_set: VelocitySetGui::D2Q9,
            initial_scalar_value: InitialScalarValueGui::Uniform { value: 0.0 },
            boundary_conditions: vec![
                FaceBC {
                    boundary_face: BoundaryFaceGui::West,
                    boundary_condition: BoundaryConditionGui::AntiBBNoFlux,
                },
                FaceBC {
                    boundary_face: BoundaryFaceGui::East,
                    boundary_condition: BoundaryConditionGui::AntiBBNoFlux,
                },
                FaceBC {
                    boundary_face: BoundaryFaceGui::South,
                    boundary_condition: BoundaryConditionGui::AntiBBNoFlux,
                },
                FaceBC {
                    boundary_face: BoundaryFaceGui::North,
                    boundary_condition: BoundaryConditionGui::AntiBBNoFlux,
                },
            ],
        }
    }
}

impl GuiConfig {
    pub(crate) fn get_ps_params_literal(&self) -> String {
        format!("ps_params_{}", self.scalar_name)
    }

    fn get_scalar_name_literal(&self) -> String {
        format!("\"{}\".to_string()", self.scalar_name)
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

    fn get_initial_scalar_value_literal(&self) -> String {
        match &self.initial_scalar_value {
            InitialScalarValueGui::Uniform { value } => {
                format!("functions::uniform_scalar_value({}_f64, n.clone())", value)
            }
            InitialScalarValueGui::FromFile { file_path } => {
                format!("functions::from_scalar_file(\"{}\")", file_path)
            }
        }
    }

    fn get_boundary_conditions_literal(&self) -> String {
        let mut boundary_conditions_literals = vec![];
        for face_bc in &self.boundary_conditions {
            let boundary_face_literal = face_bc.boundary_face.to_literal();
            let boundary_condition_literal = face_bc.boundary_condition.to_literal();
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
    pub(crate) fn ui_scalar_name(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Scalar name:");
            ui.text_edit_singleline(&mut self.scalar_name);
        });
    }

    pub(crate) fn ui_lattice_parameters(&mut self, ui: &mut egui::Ui, dim: Dimensionality) {
        ui.heading("Lattice parameters");
        self.ui_velocity_set(ui, dim);
        ui.add_space(10.0);
        self.ui_collision_operator(ui);
    }

    pub(crate) fn ui_initial_scalar_value(&mut self, ui: &mut egui::Ui) {
        ui.heading("Initial scalar value");
        ui.horizontal(|ui| {
            ui.label("Mode:");
            let cur_value = match &self.initial_scalar_value {
                InitialScalarValueGui::Uniform { value } => *value,
                _ => 1.0,
            };
            let cur_file_path = match &self.initial_scalar_value {
                InitialScalarValueGui::FromFile { file_path } => file_path.clone(),
                _ => format!("./pre_processing/{}.csv", self.scalar_name),
            };

            egui::ComboBox::from_id_salt("initial_scalar_value_combo_box")
                .selected_text(match &self.initial_scalar_value {
                    InitialScalarValueGui::Uniform { value: _ } => "Uniform",
                    InitialScalarValueGui::FromFile { file_path: _ } => "From file",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.initial_scalar_value,
                        InitialScalarValueGui::Uniform { value: cur_value },
                        "Uniform",
                    );
                    ui.selectable_value(
                        &mut self.initial_scalar_value,
                        InitialScalarValueGui::FromFile {
                            file_path: cur_file_path.clone(),
                        },
                        "From file",
                    );
                });
        });
        ui.horizontal(|ui| match &mut self.initial_scalar_value {
            InitialScalarValueGui::Uniform { value } => {
                ui.label("Value:");
                ui.add(egui::DragValue::new(value).speed(0.01));
            }
            InitialScalarValueGui::FromFile { file_path } => {
                ui.label("File path:");
                ui.text_edit_singleline(file_path);
            }
        });
    }

    pub(crate) fn ui_boundary_conditions(&mut self, ui: &mut egui::Ui, dim: Dimensionality) {
        ui.heading("Boundary conditions");
        let (possible_faces, number_of_faces) = match dim {
            Dimensionality::D2 => (
                vec![
                    BoundaryFaceGui::West,
                    BoundaryFaceGui::East,
                    BoundaryFaceGui::South,
                    BoundaryFaceGui::North,
                ],
                4,
            ),
            Dimensionality::D3 => (
                vec![
                    BoundaryFaceGui::West,
                    BoundaryFaceGui::East,
                    BoundaryFaceGui::South,
                    BoundaryFaceGui::North,
                    BoundaryFaceGui::Bottom,
                    BoundaryFaceGui::Top,
                ],
                6,
            ),
        };
        for i in 0..number_of_faces {
            if i >= self.boundary_conditions.len() {
                self.boundary_conditions.push(FaceBC {
                    boundary_face: possible_faces[i],
                    boundary_condition: BoundaryConditionGui::AntiBBNoFlux,
                });
            }
            let face_bc = &mut self.boundary_conditions[i];
            ui.horizontal(|ui| {
                ui.label(format!("Face {}:", i));
                egui::ComboBox::from_id_salt(format!("boundary_face_combo_box_{}", i))
                    .selected_text(face_bc.boundary_face.to_literal())
                    .show_ui(ui, |ui| {
                        for possible_face in &possible_faces {
                            ui.selectable_value(
                                &mut face_bc.boundary_face,
                                *possible_face,
                                possible_face.to_literal(),
                            );
                        }
                    });
                let cur_abb_value = match &face_bc.boundary_condition {
                    BoundaryConditionGui::AntiBounceBack { value } => *value,
                    _ => 0.0,
                };
                egui::ComboBox::from_id_salt(format!("boundary_condition_combo_box_{}", i))
                    .selected_text(match &face_bc.boundary_condition {
                        BoundaryConditionGui::AntiBBNoFlux => "No-flux",
                        BoundaryConditionGui::AntiBounceBack { .. } => "Anti-bounce-back",
                        BoundaryConditionGui::Periodic => "Periodic",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut face_bc.boundary_condition,
                            BoundaryConditionGui::AntiBBNoFlux,
                            "No-flux",
                        );
                        ui.selectable_value(
                            &mut face_bc.boundary_condition,
                            BoundaryConditionGui::AntiBounceBack {
                                value: cur_abb_value,
                            },
                            "Anti-bounce-back",
                        );
                        ui.selectable_value(
                            &mut face_bc.boundary_condition,
                            BoundaryConditionGui::Periodic,
                            "Periodic",
                        );
                    });
                if let BoundaryConditionGui::AntiBounceBack { value } =
                    &mut face_bc.boundary_condition
                {
                    ui.label("value:");
                    ui.add(egui::DragValue::new(value).speed(0.01));
                }
            });
        }
    }
}

impl GuiConfig {
    pub(crate) fn get_ps_params_content(&self) -> String {
        let ps_params_name_literal = self.get_ps_params_literal();
        let scalar_name_literal = self.get_scalar_name_literal();
        let collision_operator_literal = self.get_collision_operator_literal();
        let velocity_set_literal = self.get_velocity_set_literal();
        let initial_scalar_value_literal = self.get_initial_scalar_value_literal();
        let boundary_conditions_literal = self.get_boundary_conditions_literal();
        format!(
            r#"    let {ps_params_name_literal} = ps::Parameters {{
        scalar_name: {scalar_name_literal},
        collision_operator: {collision_operator_literal},
        velocity_set: {velocity_set_literal},
        initial_scalar_value: {initial_scalar_value_literal},
        boundary_conditions: {boundary_conditions_literal},
        source_value: None,
    }};
"#
        )
    }
}
