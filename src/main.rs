use std::mem;

use eframe::{self, Frame};
use egui::Context;

use gol::Game;

fn main() -> eframe::Result {
    eframe::run_native(
        "Game of Life",
        Default::default(),
        Box::new(|_| {
            Ok(Box::new(App {
                state: State::Pending("10".to_string(), "10".to_string()),
            }))
        }),
    )
}

enum State {
    Pending(String, String),
    Ready(Game),
}

struct App {
    state: State,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::TopBottomPanel::top("Control Panel").show(ctx, |ui| {
            ui.label("Game of Life");

            // if we haven't initialized the game yet then we will show the x & y value loading
            // screen with a button to click. Once that value is shown then we'll show the game
            // board with some other buttons to run it.
            match &mut self.state {
                State::Pending(ref mut x_value, ref mut y_value) => {
                    ui.label("X Value:");
                    ui.text_edit_singleline(x_value);
                    ui.label("Y Value:");
                    ui.text_edit_singleline(y_value);

                    if ui.button("Set Game Size").clicked() {
                        let x_size: usize = match x_value.parse() {
                            Ok(x_size) => x_size,
                            Err(err) => {
                                ui.label(format!("Error parsing X Value: {}", err));
                                return;
                            }
                        };

                        let y_size: usize = match y_value.parse() {
                            Ok(y_size) => y_size,
                            Err(err) => {
                                ui.label(format!("Error parsing Y Value: {}", err));
                                return;
                            }
                        };

                        mem::swap(
                            &mut self.state,
                            &mut State::Ready(Game::new(x_size, y_size)),
                        );
                    }
                }
                State::Ready(game) => {
                    egui::Grid::new("Board")
                        .num_columns(game.x_size())
                        .show(ui, |ui| {
                            for y in 0..game.y_size() {
                                for x in 0..game.x_size() {
                                    ui.radio_value(game.cell(x, y), true, "");
                                }
                                ui.end_row();
                            }
                        });

                    ui.horizontal(|ui| {
                        if ui.button("Clear Board").clicked() {
                            game.clear();
                        }

                        if ui.button("Run Once").clicked() {
                            game.iterate();
                        }
                    });
                }
            };
        });
    }
}
