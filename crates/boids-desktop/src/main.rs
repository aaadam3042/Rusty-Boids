use eframe::egui::{self};
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

impl BoidsDesktopApp {
    fn paint_world(&self, ui: &mut egui::Ui) {
        let canvas_rect = ui.available_rect_before_wrap();
        let world_size = self.world.area_size();

        // Calculate scale factor
        let scale_x = canvas_rect.width() / world_size.x;
        let scale_y = canvas_rect.height() / world_size.y;
        let scale = scale_x.min(scale_y);

        let displayed_size = egui::vec2(
            world_size.x * scale,
            world_size.y * scale,
        );

        let world_rect = egui::Rect::from_center_size(
            canvas_rect.center(),
            displayed_size,
        );

        let painter = ui.painter_at(canvas_rect);

        painter.rect_filled(
            canvas_rect, 
            0.0,
            egui::Color32::BLACK
        );

        painter.rect_stroke(
            world_rect, 
            0.0, 
            egui::Stroke::new(1.0, egui::Color32::GRAY), 
            egui::StrokeKind::Inside
        );
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
        
        self.paint_world(ui);
    }
}