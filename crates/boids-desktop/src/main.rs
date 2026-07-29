use eframe::egui;
use std::time::Instant;

const FIXED_DT: f32 = 1.0 / 120.0;
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
    fn paint_world(&self,painter: &egui::Painter, canvas_rect: egui::Rect, world_rect: egui::Rect) {
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

    fn paint_boids(&self, painter: &egui::Painter, world_rect: egui::Rect, scale: f32) {
        for boid in self.world.boids() {
            let screen_position = world_to_screen(
                boid.position,
                world_rect,
                scale
            );

            painter.circle_filled(
                screen_position,
                3.0,
                egui::Color32::LIGHT_BLUE
            );
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
        let canvas_rect = ui.available_rect_before_wrap();
        let world_size = self.world.area_size();

        if world_size.x <= 0.0 || world_size.y <= 0.0 {
            return;
        }

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
        
        self.paint_world(&painter, canvas_rect, world_rect);
        self.paint_boids(&painter, world_rect, scale);
    }
}

fn world_to_screen(position: boids_core::Vec2, world_rect: egui::Rect, scale: f32) -> egui::Pos2 {
    egui::pos2(
        world_rect.left() + position.x * scale,
        world_rect.top() + position.y * scale
    )
}