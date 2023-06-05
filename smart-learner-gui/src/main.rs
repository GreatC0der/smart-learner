#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{
    egui::{self, Id, Key},
    epaint::Vec2,
};
use egui_file::FileDialog;
use smart_learner_helper::app::App;

fn main() {
    env_logger::init();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
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
    new_deck_name: String,
    choose_folder_dialog: Option<FileDialog>,
}

enum GuiState {
    Main,
    NewCard,
    RevisingWithoutAnswer,
    RevisingWithAnswer,
    Settings,
}

impl Default for GuiApp {
    fn default() -> Self {
        Self {
            app: App::new(),
            state: GuiState::Main,
            new_deck_name: "".to_string(),
            choose_folder_dialog: None,
        }
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Menu
        egui::TopBottomPanel::bottom(Id::new("menu")).show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Home").clicked() {
                    self.state = GuiState::Main;
                };
                if ui.button("Settings").clicked() {
                    self.state = GuiState::Settings;
                };
                if ui.button("New card").clicked() {
                    self.state = GuiState::NewCard;
                };
            });
        });

        // Showing the page
        match self.state {
            GuiState::Main => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    // Displaying decks
                    let label = ui.label("Decks:");
                    for (index, deck) in self.app.decks.iter().enumerate() {
                        if ui.link(&deck.value.name).labelled_by(label.id).clicked() {
                            self.state = GuiState::RevisingWithoutAnswer;
                            self.app.current_deck = index;
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

            GuiState::RevisingWithoutAnswer => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    match self.app.get_front_for_revision() {
                        Some(value) => {
                            ui.heading(&value);
                            if ui.button("Show answer").clicked()
                                || ctx.input(|i| i.key_pressed(Key::Space))
                            {
                                self.state = GuiState::RevisingWithAnswer;
                            }
                        }
                        None => {
                            ui.heading("No cards to review.");
                        }
                    }
                });
            }

            GuiState::RevisingWithAnswer => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading(self.app.get_question());
                    ui.heading(self.app.get_answer());
                });
            }

            GuiState::NewCard => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Add a card");
                    egui::ComboBox::from_label("Deck")
                        .selected_text(self.app.current_deck_name())
                        .show_ui(ui, |ui| {
                            for (index, deck) in self.app.decks.iter().enumerate() {
                                ui.selectable_value(
                                    &mut self.app.current_deck,
                                    index,
                                    &deck.value.name,
                                );
                            }
                        });

                    ui.group(|ui| {
                        let label = ui.label("Front:");
                        ui.text_edit_multiline(&mut self.app.new_card_front)
                            .labelled_by(label.id);
                    });

                    ui.group(|ui| {
                        let label = ui.label("Back:");
                        ui.text_edit_multiline(&mut self.app.new_card_back)
                            .labelled_by(label.id);
                    });

                    if ui.button("Create").clicked() {
                        self.app.create_card();
                    }
                });
            }

            GuiState::Settings => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    if ui.button("Change folder with decks").clicked() {
                        let mut dialog =
                            FileDialog::select_folder(None).default_size(Vec2::new(480.0, 300.0));
                        dialog.open();
                        self.choose_folder_dialog = Some(dialog);
                    }

                    if let Some(dialog) = &mut self.choose_folder_dialog {
                        if dialog.show(ctx).selected() {
                            if let Some(file) = dialog.path() {
                                self.app.config.folder_path =
                                    file.as_path().to_str().unwrap().to_string();
                            }
                        }
                    }
                });
            }
        }
    }
}
