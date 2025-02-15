use eframe::egui;
use egui_extras::{Column, TableBuilder};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Financial Manager",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct Record {
    date: String,
    description: String,
    earnings_in_cents: u32,
    spendings_in_cents: u32,
}

impl Record {
    fn new(
        date: String,
        description: String,
        earnings_in_cents: u32,
        spendings_in_cents: u32,
    ) -> Self {
        Record {
            date,
            description,
            earnings_in_cents,
            spendings_in_cents,
        }
    }
}

struct MyApp {
    records: Vec<Record>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            records: vec![Record::new(
                "2025-02-15".to_owned(),
                "Little treat :)".to_owned(),
                0u32,
                8u32,
            )],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Transaction records");
            TableBuilder::new(ui)
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Date");
                    });
                    header.col(|ui| {
                        ui.heading("Description");
                    });
                    header.col(|ui| {
                        ui.heading("Earnings");
                    });
                    header.col(|ui| {
                        ui.heading("Spendings");
                    });
                })
                .body(|mut body| {
                    self.records.iter().for_each(|record| {
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.label(&record.date);
                            });
                            row.col(|ui| {
                                ui.label(&record.description);
                            });
                            row.col(|ui| {
                                ui.label(record.earnings_in_cents.to_string());
                            });
                            row.col(|ui| {
                                ui.label(record.spendings_in_cents.to_string());
                            });
                        });
                    });
                });
        });
    }
}
