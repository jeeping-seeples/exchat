//! Centered chat transcript with user block (2px left border), assistant plain text,
//! CollapsingHeader research steps, search results, and Sources [n] footer.

use crate::ui::state::{Message, MessageRole, ResearchStage, ResearchState};
use crate::ui::theme::{AMBER, INK, RESEARCH_BLUE};
use eframe::egui::{
    self, CollapsingHeader, Color32, FontFamily, FontId, Frame, Margin, RichText, ScrollArea,
    Stroke, Ui,
};

pub fn render_transcript(ui: &mut Ui, messages: &[Message]) {
    // Transcript must be centered with a max width of 680px
    ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let avail_width = ui.available_width();
                let transcript_width = 680.0f32.min(avail_width);
                let margin = ((avail_width - transcript_width) / 2.0).max(0.0);

                ui.add_space(margin);
                ui.vertical(|ui| {
                    ui.set_width(transcript_width);
                    ui.add_space(16.0);

                    if messages.is_empty() {
                        ui.add_space(40.0);
                        ui.vertical_centered(|ui| {
                            ui.label(
                                RichText::new("Start a research chat by asking a question below.")
                                    .size(14.0)
                                    .color(Color32::from_rgb(0x60, 0x6E, 0x68)),
                            );
                        });
                    }

                    for msg in messages {
                        match msg.role {
                            MessageRole::User => {
                                render_user_message(ui, &msg.content);
                            }
                            MessageRole::Assistant => {
                                render_assistant_message(ui, msg);
                            }
                        }
                        ui.add_space(18.0);
                    }
                    ui.add_space(20.0);
                });
                ui.add_space(margin);
            });
        });
}

fn render_user_message(ui: &mut Ui, content: &str) {
    // User block with 2px left rule, no chat bubbles
    let stroke_color = RESEARCH_BLUE;
    let text_color = INK;

    ui.scope(|ui| {
        let rect = ui.available_rect_before_wrap();
        let start_pos = ui.cursor().min;

        Frame::new()
            .inner_margin(Margin {
                left: 12,
                right: 8,
                top: 4,
                bottom: 4,
            })
            .show(ui, |ui| {
                ui.label(
                    RichText::new(content)
                        .font(FontId::new(14.0, FontFamily::Proportional))
                        .color(text_color),
                );
            });

        let end_pos = ui.cursor().min;
        let height = (end_pos.y - start_pos.y).max(20.0);
        let p1 = egui::pos2(rect.min.x + 2.0, start_pos.y + 2.0);
        let p2 = egui::pos2(rect.min.x + 2.0, start_pos.y + height - 2.0);

        ui.painter()
            .line_segment([p1, p2], Stroke::new(2.0, stroke_color));
    });
}

fn render_assistant_message(ui: &mut Ui, msg: &Message) {
    // Inline research steps if any
    if let Some(ref research) = msg.research {
        render_research_steps(ui, research);
        ui.add_space(8.0);
    }

    // Assistant plain text
    if !msg.content.is_empty() {
        ui.label(
            RichText::new(&msg.content)
                .font(FontId::new(14.0, FontFamily::Proportional))
                .color(INK),
        );
    }

    // Sources [n] footer
    if !msg.sources.is_empty() {
        ui.add_space(10.0);
        ui.scope(|ui| {
            ui.label(
                RichText::new("Sources")
                    .font(FontId::new(12.0, FontFamily::Proportional))
                    .color(Color32::from_rgb(0x60, 0x6E, 0x68)),
            );
            ui.add_space(4.0);
            for source in &msg.sources {
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new(format!("[{}]", source.number))
                            .font(FontId::new(12.0, FontFamily::Proportional))
                            .color(RESEARCH_BLUE),
                    );
                    ui.label(
                        RichText::new(&source.title)
                            .font(FontId::new(12.0, FontFamily::Proportional))
                            .color(INK),
                    );
                    ui.label(
                        RichText::new(format!("({})", source.domain))
                            .font(FontId::new(12.5, FontFamily::Monospace))
                            .color(Color32::from_rgb(0x60, 0x6E, 0x68)),
                    );
                });
            }
        });
    }
}

fn render_research_steps(ui: &mut Ui, research: &ResearchState) {
    let title_text = match &research.stage {
        ResearchStage::Refining => "Research: refining…".to_string(),
        ResearchStage::Searching { query } => format!("Research: searching ({})", query),
        ResearchStage::Judging => "Research: judging…".to_string(),
        ResearchStage::Writing => "Research: writing…".to_string(),
        ResearchStage::Done => format!(
            "Research completed ({} round{}, {} sources)",
            research.rounds_used,
            if research.rounds_used > 1 { "s" } else { "" },
            research.results.len()
        ),
    };

    let is_active = research.stage != ResearchStage::Done;

    // Header with status dot and CollapsingHeader
    ui.horizontal(|ui| {
        let dot_color = if is_active {
            AMBER
        } else {
            Color32::from_rgb(0x60, 0x6E, 0x68)
        };
        let (dot_rect, _) = ui.allocate_exact_size(egui::vec2(10.0, 10.0), egui::Sense::hover());
        ui.painter()
            .circle_filled(dot_rect.center(), 3.5, dot_color);

        CollapsingHeader::new(
            RichText::new(title_text)
                .font(FontId::new(12.0, FontFamily::Proportional))
                .color(if is_active { AMBER } else { INK }),
        )
        .id_salt("research_steps")
        .default_open(is_active)
        .show(ui, |ui| {
            ui.add_space(4.0);

            // Step 1: refining
            ui.horizontal(|ui| {
                let dot = if matches!(research.stage, ResearchStage::Refining) {
                    "●"
                } else {
                    "✓"
                };
                ui.label(
                    RichText::new(format!("{} refining prompt", dot))
                        .font(FontId::new(12.0, FontFamily::Proportional))
                        .color(Color32::from_rgb(0x60, 0x6E, 0x68)),
                );
            });

            // Step 2: searching
            ui.horizontal(|ui| {
                let dot = match &research.stage {
                    ResearchStage::Refining => "○",
                    ResearchStage::Searching { .. } => "●",
                    _ => "✓",
                };
                ui.label(
                    RichText::new(format!("{} searching web", dot))
                        .font(FontId::new(12.0, FontFamily::Proportional))
                        .color(Color32::from_rgb(0x60, 0x6E, 0x68)),
                );
                if let ResearchStage::Searching { query } = &research.stage {
                    ui.label(
                        RichText::new(query)
                            .font(FontId::new(12.5, FontFamily::Monospace))
                            .color(RESEARCH_BLUE),
                    );
                }
            });

            // Step 3: judging
            ui.horizontal(|ui| {
                let dot = match &research.stage {
                    ResearchStage::Refining | ResearchStage::Searching { .. } => "○",
                    ResearchStage::Judging => "●",
                    _ => "✓",
                };
                ui.label(
                    RichText::new(format!("{} judging evidence sufficiency", dot))
                        .font(FontId::new(12.0, FontFamily::Proportional))
                        .color(Color32::from_rgb(0x60, 0x6E, 0x68)),
                );
            });

            // Search results list if any
            if !research.results.is_empty() {
                ui.add_space(6.0);
                ui.label(
                    RichText::new("Collected evidence:")
                        .font(FontId::new(12.0, FontFamily::Proportional))
                        .color(Color32::from_rgb(0x60, 0x6E, 0x68)),
                );

                for r in &research.results {
                    ui.group(|ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.label(
                                RichText::new(format!("[{}]", r.domain))
                                    .font(FontId::new(12.5, FontFamily::Monospace))
                                    .color(RESEARCH_BLUE),
                            );
                            ui.label(
                                RichText::new(&r.title)
                                    .font(FontId::new(12.0, FontFamily::Proportional))
                                    .color(INK),
                            );
                        });
                        ui.label(
                            RichText::new(&r.snippet)
                                .font(FontId::new(12.0, FontFamily::Proportional))
                                .color(Color32::from_rgb(0x50, 0x5E, 0x58)),
                        );
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(format!("cite [{}]", r.cite_number))
                                    .font(FontId::new(12.5, FontFamily::Monospace))
                                    .color(Color32::from_rgb(0x70, 0x7E, 0x78)),
                            );
                        });
                    });
                }
            }
        });
    });
}
