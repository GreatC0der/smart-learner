#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, Id};
use smart_learner_helper::app::App;
fn main() {
    env_logger::init();
    let options = eframe::NativeOptions {
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
    revising_deck: usize,
    new_deck_name: String,
}

enum GuiState {
    MainPage,
    Revising,
}

impl Default for GuiApp {
    fn default() -> Self {
        Self {
            app: App::new(),
            state: GuiState::MainPage,
            revising_deck: 0,
            new_deck_name: "".to_string(),
        }
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                egui::CentralPanel::default().show(ctx, |ui| {
                    // Displaying decks
                    let label = ui.label("Decks:");
                    for (index, deck) in self.app.decks.iter().enumerate() {
                        if ui.link(&deck.value.name).labelled_by(label.id).clicked() {
                            self.state = GuiState::Revising;
                            self.revising_deck = index;
                        }
                    }

                    // Horisontal thingy to create new decks
                    ui.horizontal(|ui| {
                        let label = ui.label("Deck name:");
                        ui.text_edit_singleline(&mut self.new_deck_name)
                            .labelled_by(label.id);
                        let button = ui.button("Create deck");
                        if button.clicked() {
                            self.app.new_deck(self.new_deck_name.clone());
                        }
                    });
                });
            }
            GuiState::Revising => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label("Revise in your book!");
                });
            }
        }
    }
}