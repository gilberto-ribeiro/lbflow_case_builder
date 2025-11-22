use lbflow_case_builder::ui::GuiApp;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "lbflow case builder",
        native_options,
        Box::new(|_cc| Ok(Box::new(GuiApp::new()))),
    );
}
