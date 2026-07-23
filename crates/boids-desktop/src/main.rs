use eframe::egui;


fn main() {
    eframe::run_native(
        "Boids Desktop App",
        eframe::NativeOptions::default(),
        Box::new(
            |cc| Ok(Box::new(BoidsDesktopApp::new(cc)))
        ),
    );
}

struct BoidsDesktopApp {
    // world: boids_core::World,
}

impl BoidsDesktopApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
    //     Self {
    //         world: boids_core::World::new_default_params(),
    //     }
        Self {}
    }
}

impl eframe::App for BoidsDesktopApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.label("Boids Desktop App");
        // Here you can add more UI elements to control the simulation
    }
}