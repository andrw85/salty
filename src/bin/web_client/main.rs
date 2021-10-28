use eframe::{egui, epi};

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl epi::App for MyApp {
    fn name(&self) -> &str {
        "Saltγ vault manager"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self { name, age } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Current Account entries:");
            ui.horizontal(|ui| {
                ui.label(format!("{}. {}", index, name));
                // ui.text_edit_singleline(name);
            });
            ui.add(egui::Slider::new(age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                *age += 1;
            }
            ui.label(format!("Hello '{}', age {}", name, age));
        });

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(MyApp::default()), options);
}
