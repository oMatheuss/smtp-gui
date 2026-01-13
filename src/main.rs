// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod query;
mod components;
mod mailer;

use eframe::egui;
use eframe::egui::Widget;

use crate::config::SmtpConfig;
use crate::components::AppUI;
use crate::mailer::{Mail, MailError, Mailer};
use crate::query::AsyncQuery;

fn main() -> eframe::Result {
    let smtp_config = match std::fs::File::open("config.json") {
        Ok(file) => {
            let reader = std::io::BufReader::new(file);
            match serde_json::de::from_reader::<_, SmtpConfig>(reader) {
                Ok(config) => config,
                Err(..) => SmtpConfig::default(),
            }
        },
        Err(..) => SmtpConfig::default(),
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_resizable(false)
            .with_maximize_button(false),
        ..Default::default()
    };

    eframe::run_native("SMTP Client", options, Box::new(|_cc| Ok(Box::new(App::from(smtp_config)))))
}

#[derive(Debug, Default)]
struct App {
    config: SmtpConfig,

    subject: String,
    to: String,
    body: String,

    submit: AsyncQuery<(), MailError>,
}

impl From<SmtpConfig> for App {
    fn from(config: SmtpConfig) -> Self {
        Self { config, ..Default::default() }
    }
}


impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).inner_margin(20.0))
            .show(ctx, |ui| {
                ui.heading("Smtp Settings");
                ui.add_space(10.0);

                egui::Grid::new("smtp_settings")
                    .min_col_width((ui.available_width() - 40.0) / 3.0)
                    .num_columns(3)
                    .spacing(&[20f32, 10f32])
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            let label = ui.label("From");
                            AppUI(ui).input(&mut self.config.from).labelled_by(label.id);
                        });

                        ui.vertical(|ui| {
                            let label = ui.label("Host");
                            AppUI(ui).input(&mut self.config.host).labelled_by(label.id);
                        });

                        ui.vertical(|ui| {
                            let label = ui.label("Port");
                            AppUI(ui).numeric(&mut self.config.port).labelled_by(label.id);
                        });

                        ui.end_row();

                        ui.vertical(|ui| {
                            let label = ui.label("Username");
                            AppUI(ui).input(&mut self.config.username).labelled_by(label.id);
                        });

                        ui.vertical(|ui| {
                            let label = ui.label("Password");
                            AppUI(ui).password(&mut self.config.password).labelled_by(label.id);
                        });

                        ui.end_row();
                    });

                ui.add_space(20.0);
                ui.separator();
                ui.add_space(20.0);

                ui.heading("Message");
                ui.add_space(10.0);

                ui.vertical(|ui| {
                    let label = ui.label("To");
                    AppUI(ui).input(&mut self.to).labelled_by(label.id);
                });

                ui.add_space(10.0);

                ui.vertical(|ui| {
                    let label = ui.label("Subject");
                    AppUI(ui).input(&mut self.subject).labelled_by(label.id);
                });

                ui.add_space(10.0);

                ui.vertical(|ui| {
                    let label = ui.label("Message");
                    AppUI(ui).textarea(&mut self.body).labelled_by(label.id);
                });

                ui.add_space(20.0);

                let handler = ui.vertical_centered(|ui| {
                    egui::Button::new("Enviar").min_size((ui.available_width(), 40.0).into()).ui(ui)
                });

                if handler.inner.clicked() && self.submit.is_ready() {
                    let config = self.config.clone();
                    let mail = Mail {
                        subject: self.subject.clone(),
                        to: self.to.clone(),
                        body: self.body.clone(),
                    };
                    
                    self.submit.fetch(async move { Mailer::send(config, mail).await });
                }

                self.submit.poll();

                match &self.submit.state {
                    query::QueryState::Idle => {},
                    query::QueryState::Loading => {
                        ui.spinner();
                        ui.label("Sending the e-mail");
                    },
                    query::QueryState::Success(_) => {
                        ui.label("Email has been delivered successfuly");
                    },
                    query::QueryState::Error(err) => {
                        ui.label("An error occurred while sending this email");
                        ui.label(egui::RichText::new(err.to_string()).color(egui::Color32::RED));
                    },
                }
            });
    }
}