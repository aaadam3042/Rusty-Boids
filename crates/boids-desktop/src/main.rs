use eframe::egui;
use std::time::Instant;

const FIXED_DT: f32 = 1.0/120.0;
const MAX_FRAME_TIME: f32 = 0.1;

fn main() -> eframe::Result {
    eframe::run_native(
        "Boids Desktop App",
        eframe::NativeOptions::default(),
        Box::new(
            |_cc| Ok(Box::new(BoidsDesktopApp::new()))
        ),
    )
}

struct BoidsDesktopApp {
    world: boids_core::World,
    previous_time: Instant,
    accumulator: f32,
}

impl BoidsDesktopApp {
    fn new() -> Self {
        let mut rng = rand::rng();

        Self {
            world: boids_core::World::new_default_params(&mut rng),
            previous_time: Instant::now(),
            accumulator: 0.0,
        }
    }
}

impl eframe::App for BoidsDesktopApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();

        let elapsed = now.duration_since(self.previous_time).as_secs_f32();

        self.previous_time = now;

        let elapsed = elapsed.min(MAX_FRAME_TIME);
        self.accumulator += elapsed;

        while self.accumulator >= FIXED_DT {
            self.world.tick(FIXED_DT);
            self.accumulator -= FIXED_DT;
        }

        ctx.request_repaint();
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.label("Boids Desktop App");
        // Here you can add more UI elements to control the simulation
    }
}