use eframe::egui;
use std::path::PathBuf;

use crate::config::*;

pub struct GuiApp {
    m_cfg: momentum::GuiConfig,
    n_passive_scalars: usize,
    ps_cfg_vec: Vec<passive_scalar::GuiConfig>,
    c_cfg: CargoGuiConfig,
    parent_dir: String,
    status: String,
}

impl Default for GuiApp {
    fn default() -> Self {
        Self {
            m_cfg: momentum::GuiConfig::default(),
            n_passive_scalars: 0,
            ps_cfg_vec: vec![],
            c_cfg: CargoGuiConfig::default(),
            parent_dir: String::from("./cases"),
            status: String::new(),
        }
    }
}

impl GuiApp {
    pub fn new() -> Self {
        Self::default()
    }

    fn ui_dim(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Dimensions:");
            ui.selectable_value(&mut self.m_cfg.dim, Dimensionality::D2, "2D");
            ui.selectable_value(&mut self.m_cfg.dim, Dimensionality::D3, "3D");
        });
    }

    fn ui_n(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("nx:");
            ui.add(egui::DragValue::new(&mut self.m_cfg.n[0]).range(1..=10_000));
            ui.label("ny:");
            ui.add(egui::DragValue::new(&mut self.m_cfg.n[1]).range(1..=10_000));
            if self.m_cfg.dim == Dimensionality::D3 {
                ui.label("nz:");
                ui.add(egui::DragValue::new(&mut self.m_cfg.n[2]).range(1..=10_000));
            } else {
                self.m_cfg.n[2] = 1;
            }
        });
    }

    fn ui_node_types(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Node types:");
            ui.selectable_value(
                &mut self.m_cfg.node_types,
                NodeTypesGui::OnlyFluidNodes,
                "Only fluid nodes",
            );
            ui.selectable_value(
                &mut self.m_cfg.node_types,
                NodeTypesGui::FromBounceBackMapFile,
                "From bounce-back map file",
            );
        });
    }

    fn ui_velocity_set(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Velocity set:");
            match self.m_cfg.dim {
                Dimensionality::D2 => {
                    ui.selectable_value(&mut self.m_cfg.velocity_set, VelocitySetGui::D2Q9, "D2Q9");
                }
                Dimensionality::D3 => {
                    ui.selectable_value(
                        &mut self.m_cfg.velocity_set,
                        VelocitySetGui::D3Q15,
                        "D3Q15",
                    );
                    ui.selectable_value(
                        &mut self.m_cfg.velocity_set,
                        VelocitySetGui::D3Q19,
                        "D3Q19",
                    );
                    ui.selectable_value(
                        &mut self.m_cfg.velocity_set,
                        VelocitySetGui::D3Q27,
                        "D3Q27",
                    );
                }
            };
        });
    }

    fn ui_delta_x(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Delta x:");
            ui.add(egui::DragValue::new(&mut self.m_cfg.delta_x).speed(0.01));
        });
    }

    fn ui_delta_t(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Delta t:");
            ui.add(egui::DragValue::new(&mut self.m_cfg.delta_t).speed(0.01));
        });
    }

    fn ui_physical_density(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Physical density (kg/m^3):");
            ui.add(egui::DragValue::new(&mut self.m_cfg.physical_density).speed(0.1));
        });
    }

    fn ui_reference_pressure(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Reference pressure (Pa):");
            ui.add(egui::DragValue::new(&mut self.m_cfg.reference_pressure).speed(1.0));
        });
    }

    fn ui_initial_density(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Initial density:");
            let cur_rho = match &self.m_cfg.initial_density {
                momentum::InitialDensityGui::Uniform { rho } => *rho,
                _ => 1.0,
            };
            let cur_time_step = match &self.m_cfg.initial_density {
                momentum::InitialDensityGui::FromTimeStep { time_step } => *time_step,
                _ => 0usize,
            };
            let cur_file_path = match &self.m_cfg.initial_density {
                momentum::InitialDensityGui::FromFile { file_path } => file_path.clone(),
                _ => "./pre_processing/density.csv".to_string(),
            };

            egui::ComboBox::from_id_salt("initial_density_combo_box")
                .selected_text(match &self.m_cfg.initial_density {
                    momentum::InitialDensityGui::Uniform { rho: _ } => "Uniform",
                    momentum::InitialDensityGui::FromTimeStep { time_step: _ } => "From time step",
                    momentum::InitialDensityGui::FromFile { file_path: _ } => "From file",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.m_cfg.initial_density,
                        momentum::InitialDensityGui::Uniform { rho: cur_rho },
                        "Uniform",
                    );
                    ui.selectable_value(
                        &mut self.m_cfg.initial_density,
                        momentum::InitialDensityGui::FromTimeStep {
                            time_step: cur_time_step,
                        },
                        "From time step",
                    );
                    ui.selectable_value(
                        &mut self.m_cfg.initial_density,
                        momentum::InitialDensityGui::FromFile {
                            file_path: cur_file_path.clone(),
                        },
                        "From file",
                    );
                });
        });
        ui.horizontal(|ui| match &mut self.m_cfg.initial_density {
            momentum::InitialDensityGui::Uniform { rho } => {
                ui.label("rho:");
                ui.add(egui::DragValue::new(rho).speed(0.01));
            }
            momentum::InitialDensityGui::FromTimeStep { time_step } => {
                ui.label("Time step:");
                ui.add(
                    egui::DragValue::new(time_step)
                        .range(0..=1_000_000)
                        .speed(100),
                );
            }
            momentum::InitialDensityGui::FromFile { file_path } => {
                ui.label("File path:");
                ui.text_edit_singleline(file_path);
            }
        });
    }

    fn ui_initial_velocity(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Initial velocity:");
            let (cur_ux, cur_uy, cur_uz) = match &self.m_cfg.initial_velocity {
                momentum::InitialVelocityGui::Uniform { ux, uy, uz } => (*ux, *uy, *uz),
                _ => (0.0, 0.0, 0.0),
            };
            let cur_vel_time_step = match &self.m_cfg.initial_velocity {
                momentum::InitialVelocityGui::FromTimeStep { time_step } => *time_step,
                _ => 0usize,
            };
            let cur_vel_file_path = match &self.m_cfg.initial_velocity {
                momentum::InitialVelocityGui::FromFile { file_path } => file_path.clone(),
                _ => "./pre_processing/velocity.csv".to_string(),
            };
            egui::ComboBox::from_id_salt("initial_velocity_combo_box")
                .selected_text(match &self.m_cfg.initial_velocity {
                    momentum::InitialVelocityGui::Uniform {
                        ux: _,
                        uy: _,
                        uz: _,
                    } => "Uniform",
                    momentum::InitialVelocityGui::FromTimeStep { time_step: _ } => "From time step",
                    momentum::InitialVelocityGui::FromFile { file_path: _ } => "From file",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.m_cfg.initial_velocity,
                        momentum::InitialVelocityGui::Uniform {
                            ux: cur_ux,
                            uy: cur_uy,
                            uz: cur_uz,
                        },
                        "Uniform",
                    );
                    ui.selectable_value(
                        &mut self.m_cfg.initial_velocity,
                        momentum::InitialVelocityGui::FromTimeStep {
                            time_step: cur_vel_time_step,
                        },
                        "From time step",
                    );
                    ui.selectable_value(
                        &mut self.m_cfg.initial_velocity,
                        momentum::InitialVelocityGui::FromFile {
                            file_path: cur_vel_file_path.clone(),
                        },
                        "From file",
                    );
                });
        });
        ui.horizontal(|ui| match &mut self.m_cfg.initial_velocity {
            momentum::InitialVelocityGui::Uniform { ux, uy, uz } => {
                ui.label("ux:");
                ui.add(egui::DragValue::new(ux).speed(0.01));
                ui.label("uy:");
                ui.add(egui::DragValue::new(uy).speed(0.01));
                if self.m_cfg.dim == Dimensionality::D3 {
                    ui.label("uz:");
                    ui.add(egui::DragValue::new(uz).speed(0.01));
                } else {
                    *uz = 0.0;
                }
            }
            momentum::InitialVelocityGui::FromTimeStep { time_step } => {
                ui.label("Time step:");
                ui.add(
                    egui::DragValue::new(time_step)
                        .range(0..=1_000_000)
                        .speed(100),
                );
            }
            momentum::InitialVelocityGui::FromFile { file_path } => {
                ui.label("File path:");
                ui.text_edit_singleline(file_path);
            }
        });
    }

    fn ui_m_boundary_conditions(&mut self, ui: &mut egui::Ui) {
        ui.heading("Boundary conditions");
        let (possible_faces, number_of_faces) = match self.m_cfg.dim {
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
            if i >= self.m_cfg.boundary_conditions.len() {
                self.m_cfg.boundary_conditions.push(momentum::FaceBC {
                    boundary_face: possible_faces[i],
                    boundary_condition: momentum::BoundaryConditionGui::NoSlip,
                });
            }
            let face_bc = &mut self.m_cfg.boundary_conditions[i];
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
                let (cur_bb_rho, cur_ux, cur_uy, cur_uz) = match &face_bc.boundary_condition {
                    momentum::BoundaryConditionGui::BounceBack { rho, ux, uy, uz } => {
                        (*rho, *ux, *uy, *uz)
                    }
                    _ => (1.0, 0.0, 0.0, 0.0),
                };
                let cur_abb_rho = match &face_bc.boundary_condition {
                    momentum::BoundaryConditionGui::AntiBounceBack { rho } => *rho,
                    _ => 1.0,
                };
                egui::ComboBox::from_id_salt(format!("boundary_condition_combo_box_{}", i))
                    .selected_text(match &face_bc.boundary_condition {
                        momentum::BoundaryConditionGui::NoSlip => "No-slip",
                        momentum::BoundaryConditionGui::BounceBack { .. } => "Bounce-back",
                        momentum::BoundaryConditionGui::AntiBounceBack { .. } => "Anti-bounce-back",
                        momentum::BoundaryConditionGui::Periodic => "Periodic",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut face_bc.boundary_condition,
                            momentum::BoundaryConditionGui::NoSlip,
                            "No-slip",
                        );
                        ui.selectable_value(
                            &mut face_bc.boundary_condition,
                            momentum::BoundaryConditionGui::BounceBack {
                                rho: cur_bb_rho,
                                ux: cur_ux,
                                uy: cur_uy,
                                uz: cur_uz,
                            },
                            "Bounce-back",
                        );
                        ui.selectable_value(
                            &mut face_bc.boundary_condition,
                            momentum::BoundaryConditionGui::AntiBounceBack { rho: cur_abb_rho },
                            "Anti-bounce-back",
                        );
                        ui.selectable_value(
                            &mut face_bc.boundary_condition,
                            momentum::BoundaryConditionGui::Periodic,
                            "Periodic",
                        );
                    });
                match &mut face_bc.boundary_condition {
                    momentum::BoundaryConditionGui::BounceBack { rho, ux, uy, uz } => {
                        ui.label("rho:");
                        ui.add(egui::DragValue::new(rho).speed(0.01));
                        ui.label("ux:");
                        ui.add(egui::DragValue::new(ux).speed(0.01));
                        ui.label("uy:");
                        ui.add(egui::DragValue::new(uy).speed(0.01));
                        if self.m_cfg.dim == Dimensionality::D3 {
                            ui.label("uz:");
                            ui.add(egui::DragValue::new(uz).speed(0.01));
                        } else {
                            *uz = 0.0;
                        }
                    }
                    momentum::BoundaryConditionGui::AntiBounceBack { rho } => {
                        ui.label("rho:");
                        ui.add(egui::DragValue::new(rho).speed(0.01));
                    }
                    _ => {}
                }
            });
        }
    }

    fn ui_domain(&mut self, ui: &mut egui::Ui) {
        ui.heading("Domain configurations");
        self.ui_dim(ui);
        ui.add_space(10.0);
        self.ui_n(ui);
        ui.add_space(10.0);
        self.ui_node_types(ui);
    }

    fn ui_m_lattice_parameters(&mut self, ui: &mut egui::Ui) {
        ui.heading("Lattice parameters");
        let dim = self.m_cfg.dim.clone(); // copy enum to avoid borrowing self.m_cfg immutably
        self.m_cfg.ui_velocity_set(ui, dim);
        ui.add_space(10.0);
        self.m_cfg.ui_collision_operator(ui);
        ui.add_space(10.0);
        self.ui_delta_x(ui);
        self.ui_delta_t(ui);
        self.ui_physical_density(ui);
        self.ui_reference_pressure(ui);
    }

    fn ui_m_initial_values(&mut self, ui: &mut egui::Ui) {
        ui.heading("Initial values");
        self.ui_initial_density(ui);
        ui.add_space(10.0);
        self.ui_initial_velocity(ui);
    }

    fn ui_case_name(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Case name:");
            ui.text_edit_singleline(&mut self.c_cfg.case_name);
        });
    }

    fn ui_commit_hash(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Git commit hash:");
            ui.text_edit_singleline(&mut self.c_cfg.commit_hash);
        });
    }

    fn ui_parent_dir(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Parent directory:");
            ui.text_edit_singleline(&mut self.parent_dir);
        });
    }

    fn ui_case_informations(&mut self, ui: &mut egui::Ui) {
        ui.heading("Case informations");
        self.ui_case_name(ui);
        ui.add_space(10.0);
        self.ui_parent_dir(ui);
        ui.add_space(10.0);
        self.ui_commit_hash(ui);
    }

    fn ui_passive_scalars(&mut self, ui: &mut egui::Ui) {
        let dim = self.m_cfg.dim.clone();
        ui.heading("Passive scalars");
        ui.horizontal(|ui| {
            ui.label("Number of passive scalars:");
            ui.add(
                egui::DragValue::new(&mut self.n_passive_scalars)
                    .range(0..=10)
                    .speed(1),
            );
        });
        ui.add_space(10.0);
        while self.ps_cfg_vec.len() < self.n_passive_scalars {
            self.ps_cfg_vec.push(passive_scalar::GuiConfig::default());
        }
        while self.ps_cfg_vec.len() > self.n_passive_scalars {
            self.ps_cfg_vec.pop();
        }
        // for (i, _) in self.ps_cfg_vec.iter_mut().enumerate() {
        //     ui.collapsing(format!("Passive scalar {}", i), |ui| {
        //     ui.horizontal_wrapped(|ui| {
        //         ui.heading("Scalar name:");
        //         ui.label("It's a passive scalar.");
        //     });
        // });
        for (i, ps_cfg) in self.ps_cfg_vec.iter_mut().enumerate() {
            let id = egui::Id::new(format!("ps_window_open_{}", i));
            let mut open = ui.memory(|mem| mem.data.get_temp::<bool>(id).unwrap_or(false));

            ui.horizontal(|ui| {
                ui.label(format!("Scalar {}:", i));
                ui.text_edit_singleline(&mut ps_cfg.scalar_name);
                if ui.button(if open { "Close" } else { "Open" }).clicked() {
                    open = !open;
                }
            });

            if open {
                let ctx = ui.ctx();
                egui::Window::new(format!("Scalar {}: {}", i, ps_cfg.scalar_name))
                    .id(egui::Id::new(format!("ps_window_{}", i)))
                    .open(&mut open)
                    .show(ctx, |ui| {
                        ps_cfg.ui_scalar_name(ui);
                        ui.separator();
                        ps_cfg.ui_initial_scalar_value(ui);
                        ui.separator();
                        ps_cfg.ui_lattice_parameters(ui, dim);
                        ui.separator();
                        ps_cfg.ui_boundary_conditions(ui, dim);
                    });
            }

            ui.memory_mut(|mem| mem.data.insert_temp(id, open));
        }
    }

    fn ui_build_button(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Build case").clicked() {
                if self.c_cfg.case_name.trim().is_empty() {
                    self.status = "Case name vazio".to_string();
                } else {
                    let result = (|| {
                        let case_dir = PathBuf::from(&self.parent_dir).join(&self.c_cfg.case_name);
                        let src_dir = case_dir.join("src");
                        let pre_processing_dir = case_dir.join("pre_processing");
                        std::fs::create_dir_all(&src_dir)?;
                        std::fs::create_dir_all(&pre_processing_dir)?;
                        let cargo_toml_content = self.c_cfg.get_cargo_toml();
                        std::fs::write(case_dir.join("Cargo.toml"), cargo_toml_content)?;
                        let main_rs_content = self.get_main_rs_content();
                        std::fs::write(src_dir.join("main.rs"), main_rs_content)?;
                        Ok::<(), std::io::Error>(())
                    })();
                    match result {
                        Ok(_) => self.status = "Case criado com sucesso".to_string(),
                        Err(e) => self.status = format!("Erro: {}", e),
                    }
                }
            }
            ui.label(&self.status);
        });
    }
}

impl GuiApp {
    fn get_ps_params_vec_content(&self) -> String {
        let mut ps_params_contents = vec![];
        for ps_cfg in &self.ps_cfg_vec {
            let ps_params_content = ps_cfg.get_ps_params_content();
            ps_params_contents.push(ps_params_content);
        }
        ps_params_contents.join("\n")
    }

    fn get_ps_solve_content(&self) -> String {
        match self.n_passive_scalars {
            0 => "".to_string(),
            1 => {
                let ps_cfg = &self.ps_cfg_vec[0];
                let ps_params_literal = ps_cfg.get_ps_params_literal();
                format!("    // ps::solve(m_params, {});", ps_params_literal)
            }
            _ => {
                let ps_params_vec_literal = self
                    .ps_cfg_vec
                    .iter()
                    .map(|ps_cfg| ps_cfg.get_ps_params_literal())
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("    // ps::solve_vec(m_params, vec![{}]);", ps_params_vec_literal)
            }
        }
    }

    fn get_main_rs_content(&self) -> String {
        let n_literal = self.m_cfg.get_n_literal();
        let m_params_content = self.m_cfg.get_m_params_content();
        let ps_params_vec_content = self.get_ps_params_vec_content();
        let ps_params_solve_content = self.get_ps_solve_content();
        format!(
            r#"use lbflow::prelude::*;

fn main() {{
    let n = {n_literal};

{m_params_content}
{ps_params_vec_content}

    m::solve(m_params);
{ps_params_solve_content}
}}
"#
        )
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("lbflow case builder");
                ui.separator();
                ui.add_space(10.0);

                self.ui_case_informations(ui);
                ui.separator();

                self.ui_domain(ui);
                ui.separator();

                self.ui_m_lattice_parameters(ui);
                ui.separator();

                self.ui_m_initial_values(ui);
                ui.separator();

                self.ui_m_boundary_conditions(ui);
                ui.separator();

                self.ui_passive_scalars(ui);
                ui.separator();

                self.ui_build_button(ui);
            });
        });
    }
}
