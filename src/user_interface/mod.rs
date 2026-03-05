use crate::lang_proc::troy_parser::TroyParser;
use crate::model::repository::{self, Repository};
use eframe::egui;
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use petgraph::stable_graph::StableGraph;

pub struct BarnabyApp {
    code: String,
    repository: Repository,
    error_message: String, // Add this field
}

impl BarnabyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            code: String::new(),
            repository: Repository::new(),
            error_message: String::new(),
        }
    }
}

impl eframe::App for BarnabyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("code_panel")
            .exact_width(400.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.heading("TROY Editor");
                    ui.label("Enter the story");

                    let mut editor = CodeEditor::default()
                        .with_syntax(Syntax::rust())
                        .with_theme(ColorTheme::GITHUB_DARK)
                        .with_rows(20)
                        .with_fontsize(12.0);

                    let mut code = self.code.clone();
                    editor.show(ui, &mut code);

                    // Add error reporting section with full width
                    ui.separator();
                    ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                        let line_height = ui.text_style_height(&egui::TextStyle::Body);
                        ui.set_min_height(4.0 * line_height); // Force 4 lines height
                        ui.heading("Errors");
                        let error_text = if self.error_message.is_empty() {
                            egui::RichText::new("No errors").color(egui::Color32::GREEN)
                        } else {
                            egui::RichText::new(&self.error_message).color(egui::Color32::RED)
                        };
                        ui.add(egui::Label::new(error_text).wrap(true));
                    });

                    if code != self.code {
                        self.code = code;
                        self.repository = Repository::new();
                        self.error_message.clear();

                        for statement in self.code.lines() {
                            if !statement.is_empty() {
                                match TroyParser::build_model(statement, &mut self.repository) {
                                    Ok(_) => println!(
                                        "RREPOSITORY:\n\n{}",
                                        self.repository.display_state()
                                    ),
                                    Err(e) => {
                                        self.error_message =
                                            format!("Error parsing '{}': {}", statement, e);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Graph View");

            let mut g = StableGraph::new();

            let a = g.add_node(());
            let b = g.add_node(());
            let c = g.add_node(());

            g.add_edge(a, b, ());
            g.add_edge(b, c, ());
            g.add_edge(c, a, ());

            // Allocate space for the graph
            let (response, painter) = ui.allocate_painter(
                egui::Vec2::new(ui.available_width(), 300.0),
                egui::Sense::hover(),
            );

            let rect = response.rect;

            // Define positions for nodes (simple hardcoded layout)
            let node_positions = [
                (rect.center() + egui::Vec2::new(-50.0, -50.0)), // a
                (rect.center() + egui::Vec2::new(50.0, -50.0)),  // b
                (rect.center() + egui::Vec2::new(0.0, 50.0)),    // c
            ];

            let node_radius = 20.0;

            // Draw edges
            for edge in g.edge_indices() {
                if let Some((source, target)) = g.edge_endpoints(edge) {
                    let source_pos = node_positions[source.index()];
                    let target_pos = node_positions[target.index()];
                    painter.line_segment(
                        [source_pos, target_pos],
                        egui::Stroke::new(2.0, egui::Color32::BLACK),
                    );
                }
            }

            // Draw nodes
            for (i, &pos) in node_positions.iter().enumerate() {
                painter.circle_filled(pos, node_radius, egui::Color32::BLUE);
                // Optionally add labels
                painter.text(
                    pos,
                    egui::Align2::CENTER_CENTER,
                    format!("N{}", i),
                    egui::FontId::default(),
                    egui::Color32::WHITE,
                );
            }
        });
    }
}
