use macroquad::prelude::vec2;
use megaui_macroquad::{megaui::*, *};

pub fn draw() {
    println!("menu");
    //#[cfg(feature = "confui")]
    draw_window(
        hash!(),
        vec2(400., 200.),
        vec2(320., 400.),
        WindowParams {
            label: "Shop".to_string(),
            close_button: false,
            ..Default::default()
        },
        |ui| {},
    );
}
