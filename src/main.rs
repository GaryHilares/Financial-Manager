use eframe::egui;
use egui_extras::{Column, TableBuilder};
use regex::Regex;

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
    earnings_in_cents: i32,
    spendings_in_cents: i32,
}

impl Record {
    fn new(
        date: String,
        description: String,
        earnings_in_cents: i32,
        spendings_in_cents: i32,
    ) -> Self {
        Record {
            date,
            description,
            earnings_in_cents,
            spendings_in_cents,
        }
    }
}

struct FormInfo {
    pub date: String,
    pub description: String,
    pub earnings: String,
    pub spendings: String,
}

impl FormInfo {
    pub fn new() -> FormInfo {
        FormInfo {
            date: "".to_owned(),
            description: "".to_owned(),
            earnings: "".to_owned(),
            spendings: "".to_owned(),
        }
    }

    pub fn try_to_parse_record(&self) -> Result<Record, &str> {
        let re = Regex::new("([0-9]+)-([0-9]{2})-([0-9]{2})").unwrap();
        if re.captures(&self.date).is_none() {
            return Err("Invalid date found.");
        }

        let earnings = match self.earnings.parse::<i32>() {
            Ok(num) => num,
            Err(_) => return Err("Invalid earnings amount found."),
        };

        let spendings = match self.spendings.parse::<i32>() {
            Ok(num) => num,
            Err(_) => return Err("Invalid spendings amount found."),
        };

        return Ok(Record::new(
            self.date.to_owned(),
            self.description.to_owned(),
            earnings,
            spendings,
        ));
    }
}

struct MyApp {
    records: Vec<Record>,
    form_info: FormInfo,
    error_message: Option<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            records: vec![Record::new(
                "2025-02-15".to_owned(),
                "Little treat :)".to_owned(),
                0i32,
                8i32,
            )],
            form_info: FormInfo::new(),
            error_message: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Transaction records");
            ui.horizontal(|ui| {
                ui.label("Date");
                ui.text_edit_singleline(&mut self.form_info.date);
            });
            ui.horizontal(|ui| {
                ui.label("Description");
                ui.text_edit_singleline(&mut self.form_info.description);
            });
            ui.horizontal(|ui| {
                ui.label("Earnings");
                ui.text_edit_singleline(&mut self.form_info.earnings);
            });
            ui.horizontal(|ui| {
                ui.label("Spendings");
                ui.text_edit_singleline(&mut self.form_info.spendings);
            });
            if ui.button("Add").clicked() {
                match self.form_info.try_to_parse_record() {
                    Ok(result) => {
                        self.form_info = FormInfo::new();
                        self.records.push(result);
                        self.error_message = None;
                    }
                    Err(error) => self.error_message = Some(error.to_owned()),
                }
            }
            if let Some(error) = &self.error_message {
                ui.label(error);
            }
            TableBuilder::new(ui)
                .column(Column::auto())
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
                    header.col(|ui| {
                        ui.heading("Remaining balance");
                    });
                })
                .body(|mut body| {
                    let mut balance = 0;
                    self.records.iter().for_each(|record| {
                        balance += record.earnings_in_cents - record.spendings_in_cents;
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
                            row.col(|ui| {
                                ui.label(balance.to_string());
                            });
                        });
                    });
                });
        });
    }
}
