#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, Id};
use smart_learner_helper::app::App;
fn main() {
    env_logger::init();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Smart learner",
        options,
        Box::new(|_cc| Box::<GuiApp>::default()),
    )
    .unwrap();
}

struct GuiApp {
    app: App,
    state: GuiState,
}

enum GuiState {
    MainPage,
}

impl Default for GuiApp {
    fn default() -> Self {
        Self {
            app: App::new(),
            state: GuiState::MainPage,
        }
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Menu
        egui::TopBottomPanel::bottom(Id::new("menu")).show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Home").clicked() {
                    self.state = GuiState::MainPage;
                };
            });
        });

        // Showing the page
        match self.state {
            GuiState::MainPage => {
                egui::CentralPanel::default().show(ctx, |ui| {});
            }
        }
    }
}
