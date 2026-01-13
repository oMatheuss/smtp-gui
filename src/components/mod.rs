mod input;

use eframe::egui::{Response, TextBuffer, Ui};
use eframe::egui::emath::Numeric;
use input::Input;

pub struct AppUI<'a>(pub &'a mut Ui);

impl<'a> AppUI<'a> {
    pub fn input<'t>(self, text: &'t mut dyn TextBuffer) -> Response {
        let width = self.0.available_width();
        self.0.add(Input::singleline(text).desired_width(width))
    }

    pub fn password<'t>(self, text: &'t mut dyn TextBuffer) -> Response {
        let width = self.0.available_width();
        self.0.add(Input::singleline(text).password(true).desired_width(width))
    }

    pub fn textarea<'t>(self, text: &'t mut dyn TextBuffer) -> Response {
        let width = self.0.available_width();
        self.0.add(Input::multiline(text).desired_width(width))
    }

    pub fn numeric<'t, T: Numeric>(self, value: &'t mut T) -> Response {
        let f64value = value.to_f64();
        let mut text = if f64value == 0.0f64 { 
            String::new()
        } else {
            f64value.to_string()
        };
        let width = self.0.available_width();
        let edit = self.0.add(Input::singleline(&mut text).desired_width(width));
        let new_text: String = text.chars().filter(char::is_ascii_digit).collect();
        if let Ok(new_value) = new_text.parse() {
            *value = Numeric::from_f64(new_value);
        } else {
            *value = Numeric::from_f64(0.0);
        }
        edit
    }
}