use eframe::egui::{TextBuffer, TextEdit};

pub struct Input;

impl Input {
    pub fn singleline<'t>(text: &'t mut dyn TextBuffer) -> TextEdit<'t> {
        TextEdit::singleline(text).margin(8.0).min_size((0.0, 0.0).into())
    }

    pub fn multiline<'t>(text: &'t mut dyn TextBuffer) -> TextEdit<'t> {
        TextEdit::multiline(text).margin(8.0).min_size((0.0, 0.0).into())
    }
}