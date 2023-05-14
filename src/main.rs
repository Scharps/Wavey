use eframe::{egui::{self, plot::Bar}, epi::App};

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(MyApp::new()), native_options);
}

struct MyApp {
    num_samples: i32,
}

impl MyApp {
    fn new() -> Self {
        Self {
            num_samples: 0,
        }
    }
}

impl App for MyApp{
    fn name(&self) -> &str {
        "My Egui Window"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &eframe::epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("");
            ui.label(format!("Count: {}", self.num_samples));

            let bar_plot = egui::widgets::plot::BarChart::new(vec![Bar::new(0., 10.), Bar::new(1., 20.), Bar::new(2., 40.)]).width(300.0);
            ui.add(bar_plot);
            ui.add(egui::Label::new("Number of samples"));

            ui.add(egui::DragValue::new(&mut self.num_samples).speed(1.0));
            ui.end_row();
        });
    }
}

