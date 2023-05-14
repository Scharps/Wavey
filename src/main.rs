use std::fs;

use eframe::{
    egui::{
        self,
        plot::{Bar, BarChart, Line, Plot, PlotBounds, PlotPoints},
        PointerButton, Ui,
    },
    epaint::Color32,
    run_native, App, Frame,
};

fn main() {
    let native_options = eframe::NativeOptions::default();
    run_native(
        "Wavey",
        native_options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
    .expect("Unable to start application.");
}

struct MyApp {
    num_samples: i32,
    samples: Vec<f64>,
    path: String,
}

impl MyApp {
    fn new(_: &eframe::CreationContext) -> Self {
        Self {
            num_samples: 100,
            samples: Vec::with_capacity(100),
            path: "samples.csv".to_string(),
        }
    }

    fn truncate_samples(&mut self) {
        self.num_samples = self.num_samples.max(10);
        if (self.num_samples as usize) < self.samples.len() {
            self.samples.truncate(self.num_samples as usize);
            if let Some(last) = self.samples.last_mut() {
                *last = 0.0;
            }
        } else {
            self.samples.resize(self.num_samples as usize, 0.0);
        }
    }

    fn scale_samples(&mut self) {
        let max = self
            .samples
            .iter()
            .map(|n| n.abs())
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        if max == 0.0 {
            return;
        }
        for n in 1..self.samples.len() - 1 {
            self.samples[n] /= max;
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.truncate_samples();

            build_plot(&mut self.samples, ui);
            if ui.add(egui::Button::new("Scale samples")).clicked() {
                self.scale_samples()
            };
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.add(egui::Label::new("Number of Samples"));
                ui.add(egui::DragValue::new(&mut self.num_samples).speed(1.0));
            });
            ui.add(egui::Label::new("Export"));
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.add(egui::TextEdit::singleline(&mut self.path));
                if ui
                    .add_enabled(!self.path.is_empty(), egui::Button::new("Export"))
                    .clicked()
                {
                    fs::write(
                        &self.path,
                        self.samples
                            .iter()
                            .map(|n| format!("{},", n))
                            .collect::<String>(),
                    )
                    .unwrap();
                }
            });
        });
    }
}

fn build_plot(samples: &mut [f64], ui: &mut Ui) {
    let points: PlotPoints = samples
        .iter()
        .enumerate()
        .map(|(i, n)| [i as f64, *n])
        .collect();
    let mouse_down = ui.input(|i| i.pointer.button_down(PointerButton::Primary));
    Plot::new("Plot")
        .allow_drag(false)
        .allow_scroll(false)
        .allow_boxed_zoom(false)
        .allow_double_click_reset(false)
        .view_aspect(2.0)
        .show(ui, |plot_ui| {
            if mouse_down && plot_ui.plot_hovered() {
                if let Some(pos) = plot_ui.pointer_coordinate() {
                    let i = (pos.x.round() as usize).min(samples.len() - 2).max(1);
                    let y = pos.y.max(-1.0).min(1.0);
                    samples[i] = y;
                }
            }
            let line = Line::new(points).color(Color32::from_rgb(255, 0, 0));
            let bar_chart = BarChart::new(
                samples
                    .iter()
                    .enumerate()
                    .map(|(i, n)| Bar::new(i as f64, *n))
                    .collect(),
            );
            plot_ui.line(line);
            plot_ui.bar_chart(bar_chart);
            plot_ui.set_plot_bounds(PlotBounds::from_min_max(
                [0., -1.2],
                [samples.len() as f64, 1.2],
            ));
        });
}
