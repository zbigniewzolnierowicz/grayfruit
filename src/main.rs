use eframe::egui;
use std::path::PathBuf;

use clap::Parser;
use lopdf::Document;

#[derive(Parser)]
struct Cli {
    file: PathBuf,
}

fn generate_poem(path: &PathBuf) -> Vec<String> {
    let doc = Document::load(path).unwrap();
    let pages = doc.get_pages();
    let pages_keys = pages.keys();
    let page_numbers: Vec<u32> = pages_keys.into_iter().copied().collect();

    let contents = doc.extract_text(&page_numbers).unwrap();

    contents
        .lines()
        .map(|line| line.split_whitespace())
        .map(|line| {
            line.map(|word| word.chars().next().unwrap())
                .collect::<String>()
        })
        .collect()
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 900.0]),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_| {
            // This gives us image support:
            // egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

#[derive(Default)]
struct MyApp {
    picked_path: Option<PathBuf>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Poemificator");

            if ui.button("Open file...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.picked_path = Some(path);
                }
            };
            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Generating a poem for: ");
                    ui.monospace(picked_path.display().to_string());
                });

                egui::ScrollArea::vertical()
                    .max_width(640.0)
                    .show(ui, |ui| {
                        let label = egui::Label::new(
                            egui::RichText::new(generate_poem(picked_path).join("\n")).font(
                                egui::FontId {
                                    size: 18.0,
                                    family: egui::FontFamily::Monospace,
                                },
                            ),
                        )
                        .wrap();

                        ui.add(label);
                    });
            } else {
                ui.label("Select a file");
            };
        });
    }
}
