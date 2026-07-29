use eframe::egui;

const FIXED_DT: f32 = 1.0/120.0;

fn main() -> eframe::Result {
    eframe::run_native(
        "Boids Desktop App",
        eframe::NativeOptions::default(),
        Box::new(
            |cc| Ok(Box::new(BoidsDesktopApp::new(cc)))
        ),
    )
}

struct BoidsDesktopApp {
    world: boids_core::World,
    previous_time: std::time::Instant,
    accumulator: f32,
}

impl BoidsDesktopApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut rng = rand::rng();

        Self {
            world: boids_core::World::new_default_params(&mut rng),
        }
    }
}

impl eframe::App for BoidsDesktopApp {
    fn logic(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.label("Boids Desktop App");
        // Here you can add more UI elements to control the simulation
    }
}