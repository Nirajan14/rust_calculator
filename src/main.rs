use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Calculator",
        options,
        Box::new(|_cc| Box::new(CalculatorApp::default())),
    );
}

#[derive(Default)]
struct CalculatorApp {
    input: String,
    result: String,
}

impl CalculatorApp {
    fn press(&mut self, value: &str) {
        self.input.push_str(value);
    }

    fn clear(&mut self) {
        self.input.clear();
        self.result.clear();
    }

    fn backspace(&mut self) {
        self.input.pop();
    }

    fn calculate(&mut self) {
        match meval::eval_str(&self.input) {
            Ok(val) => self.result = val.to_string(),
            Err(_) => self.result = "Error".to_string(),
        }
    }
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(10.0, 10.0);

            ui.add_space(10.0);

            // Title
            ui.heading("Calculator");

            ui.add_space(15.0);

            // Display Box
            egui::Frame::none()
                .inner_margin(egui::Margin::same(15.0))
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(&self.input)
                            .size(28.0)
                            .color(egui::Color32::BLACK),
                    );

                    ui.add_space(5.0);

                    ui.label(
                        egui::RichText::new(&self.result)
                            .size(22.0)
                            .color(egui::Color32::LIGHT_GREEN),
                    );
                });

            ui.add_space(20.0);

            // Buttons Grid
            let buttons = [
                ["C", "X", "/", "*"],
                ["7", "8", "9", "-"],
                ["4", "5", "6", "+"],
                ["1", "2", "3", "="],
                ["0", ".", "", ""],
            ];

            for row in buttons {
                ui.horizontal(|ui| {
                    for btn in row {
                        if btn.is_empty() {
                            ui.add_space(70.0);
                            continue;
                        }

                        let mut button =
                            egui::Button::new(egui::RichText::new(btn).size(22.0))
                                .min_size(egui::vec2(70.0, 55.0))
                                .rounding(egui::Rounding::same(12.0));

                        if btn == "=" {
                            button = button.fill(egui::Color32::from_rgb(0, 120, 255));
                        }

                        if btn == "C" {
                            button = button.fill(egui::Color32::from_rgb(200, 50, 50));
                        }

                        if ui.add(button).clicked() {
                            match btn {
                                "C" => self.clear(),
                                "X" => self.backspace(),
                                "=" => self.calculate(),
                                _ => self.press(btn),
                            }
                        }
                    }
                });
            }

            ui.add_space(10.0);

            ui.label(
                egui::RichText::new("Built with Rust and egui")
                    .size(14.0)
                    .color(egui::Color32::GRAY),
            );
        });
    }
}
