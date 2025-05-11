use crate::model::repository::Repository;
use crate::parse::troy_parser::TroyParser;
use eframe::egui;
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};

pub struct BarnabyApp {
    code: String,
    repository: Repository,
}

impl BarnabyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            code: String::new(),
            repository: Repository::new(),
        }
    }
}

impl eframe::App for BarnabyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("code_panel")
            //.resizable(true)
            .exact_width(400.0)
            //.default_width(400.0)
            .show(ctx, |ui| {
                ui.heading("TROY Editor");
                ui.label("Enter the story");

                let mut editor = CodeEditor::default()
                    .with_syntax(Syntax::rust()) // TODO: Create TROY syntax
                    .with_theme(ColorTheme::GITHUB_DARK)
                    .with_rows(20)
                    .with_fontsize(12.0);
                //.with_font(egui::TextStyle::Monospace);

                let mut code = self.code.clone();
                editor.show(ui, &mut code);

                if code != self.code {
                    self.code = code;
                    // Parse TROY code and update repository

                    if self.code.ends_with("\n\n") {
                        TroyParser::build_model(&self.code, &mut self.repository);
                    }
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Graph View");
            ui.label("Graph visualization will be implemented here");
        });
    }
}
