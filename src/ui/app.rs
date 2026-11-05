//! Main exchat eframe App implementation with:
//! - SidePanel left 230px (search + history + new chat + providers)
//! - TopBottomPanel top bar (title + refine/judge/explain model pickers)
//! - CentralPanel with centered transcript (max 680px)
//! - Bottom input with multiline + Send + small status line
//! - Settings in separate Window

use crate::config::{ModelRef, Settings};
use crate::ui::settings_window::render_settings_window;
use crate::ui::state::AppState;
use crate::ui::theme::{apply_theme, AMBER, INK, LINE, PANEL, PAPER, RESEARCH_BLUE};
use crate::ui::transcript::render_transcript;
use eframe::egui::{
    self, Align, CentralPanel, Color32, ComboBox, FontFamily, FontId, Frame, Key, Layout, Margin,
    Panel, RichText, ScrollArea, Stroke, TextEdit, Ui, Vec2,
};
use std::time::Duration;

pub struct ExchatApp {
    pub state: AppState,
}

impl ExchatApp {
    pub fn new(cc: &eframe::CreationContext<'_>, settings: Settings) -> Self {
        apply_theme(&cc.egui_ctx);
        Self {
            state: AppState::new(settings),
        }
    }
}

impl eframe::App for ExchatApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        // Handle tick for simulated research loop
        self.state.tick_research();

        // Request repaint while research is running
        let mut has_active_research = false;
        if let Some(chat) = self.state.active_chat() {
            for m in &chat.messages {
                if let Some(r) = &m.research {
                    if r.stage != crate::ui::state::ResearchStage::Done {
                        has_active_research = true;
                        break;
                    }
                }
            }
        }
        if has_active_research {
            ctx.request_repaint_after(Duration::from_millis(100));
        }

        // Render Settings Window if open
        if self.state.show_settings_window {
            render_settings_window(
                &ctx,
                &mut self.state.show_settings_window,
                &mut self.state.settings,
                &mut self.state.status_message,
            );
        }

        // 1. Top Bar Panel
        Panel::top("top_panel")
            .frame(
                Frame::new()
                    .fill(PANEL)
                    .inner_margin(Margin {
                        left: 12,
                        right: 12,
                        top: 8,
                        bottom: 8,
                    })
                    .stroke(Stroke::new(1.0, LINE)),
            )
            .show(ui, |ui| {
                self.render_top_bar(ui);
            });

        // 2. Left SidePanel (230px)
        Panel::left("side_panel")
            .exact_size(230.0)
            .resizable(false)
            .frame(
                Frame::new()
                    .fill(PANEL)
                    .inner_margin(Margin::same(12))
                    .stroke(Stroke::new(1.0, LINE)),
            )
            .show(ui, |ui| {
                self.render_side_panel(ui);
            });

        // 3. Bottom Input Panel
        Panel::bottom("bottom_input_panel")
            .frame(
                Frame::new()
                    .fill(PANEL)
                    .inner_margin(Margin {
                        left: 16,
                        right: 16,
                        top: 10,
                        bottom: 10,
                    })
                    .stroke(Stroke::new(1.0, LINE)),
            )
            .show(ui, |ui| {
                self.render_bottom_input(ui);
            });

        // 4. CentralPanel: centered transcript
        CentralPanel::default()
            .frame(Frame::new().fill(PAPER))
            .show(ui, |ui| {
                let messages = self
                    .state
                    .active_chat()
                    .map(|c| c.messages.as_slice())
                    .unwrap_or(&[]);
                render_transcript(ui, messages);
            });
    }
}

impl ExchatApp {
    fn render_top_bar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            // App Title (Inter-SemiBold)
            ui.label(
                RichText::new("exchat")
                    .font(FontId::new(15.0, FontFamily::Name("Inter-SemiBold".into())))
                    .color(INK),
            );

            ui.add_space(16.0);

            // All available models across configured providers
            let all_models: Vec<(String, String)> = self
                .state
                .settings
                .providers
                .iter()
                .flat_map(|p| p.models.iter().map(move |m| (p.name.clone(), m.clone())))
                .collect();

            // Refine model picker
            ui.label(
                RichText::new("refine:")
                    .font(FontId::new(12.0, FontFamily::Proportional))
                    .color(Color32::from_rgb(0x60, 0x6E, 0x68)),
            );
            Self::model_picker(
                ui,
                "picker_refine",
                &mut self.state.settings.roles.refiner,
                &all_models,
            );

            ui.add_space(8.0);

            // Judge model picker
            ui.label(
                RichText::new("judge:")
                    .font(FontId::new(12.0, FontFamily::Proportional))
                    .color(Color32::from_rgb(0x60, 0x6E, 0x68)),
            );
            Self::model_picker(
                ui,
                "picker_judge",
                &mut self.state.settings.roles.judge,
                &all_models,
            );

            ui.add_space(8.0);

            // Explain model picker
            ui.label(
                RichText::new("explain:")
                    .font(FontId::new(12.0, FontFamily::Proportional))
                    .color(Color32::from_rgb(0x60, 0x6E, 0x68)),
            );
            Self::model_picker(
                ui,
                "picker_explain",
                &mut self.state.settings.roles.explainer,
                &all_models,
            );

            // Right-aligned Settings button
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui
                    .button(
                        RichText::new("⚙ Settings")
                            .font(FontId::new(12.0, FontFamily::Proportional))
                            .color(INK),
                    )
                    .clicked()
                {
                    self.state.show_settings_window = !self.state.show_settings_window;
                }
            });
        });
    }

    fn model_picker(
        ui: &mut Ui,
        id_salt: &str,
        current: &mut Option<ModelRef>,
        all_models: &[(String, String)],
    ) {
        let label = match current {
            Some(m) => format!("{}/{}", m.provider, m.model),
            None => "none".to_string(),
        };

        ComboBox::from_id_salt(id_salt)
            .width(110.0)
            .selected_text(
                RichText::new(label)
                    .font(FontId::new(12.0, FontFamily::Monospace))
                    .color(INK),
            )
            .show_ui(ui, |ui: &mut Ui| {
                ui.selectable_value(current, None, "none");
                for (prov, model) in all_models {
                    let m_ref = Some(ModelRef {
                        provider: prov.clone(),
                        model: model.clone(),
                    });
                    ui.selectable_value(current, m_ref, format!("{}/{}", prov, model));
                }
            });
    }

    fn render_side_panel(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            // "New chat" button
            let new_chat_btn = egui::Button::new(
                RichText::new("+ New chat")
                    .font(FontId::new(13.0, FontFamily::Proportional))
                    .color(INK),
            )
            .min_size(Vec2::new(ui.available_width(), 28.0));

            if ui.add(new_chat_btn).clicked() {
                self.state.new_chat();
            }

            ui.add_space(10.0);

            // History search input
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("🔍")
                        .font(FontId::new(12.0, FontFamily::Proportional))
                        .color(Color32::from_rgb(0x60, 0x6E, 0x68)),
                );
                ui.add(
                    TextEdit::singleline(&mut self.state.history_search_query)
                        .hint_text("Search chats...")
                        .desired_width(ui.available_width()),
                );
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(6.0);

            // History list
            ui.label(
                RichText::new("History")
                    .font(FontId::new(12.0, FontFamily::Name("Inter-SemiBold".into())))
                    .color(Color32::from_rgb(0x60, 0x6E, 0x68)),
            );

            let query = self.state.history_search_query.to_lowercase();
            let mut switch_to_idx = None;

            ScrollArea::vertical()
                .auto_shrink([false, false])
                .max_height(ui.available_height() - 70.0)
                .show(ui, |ui| {
                    for (idx, chat) in self.state.chats.iter().enumerate() {
                        if !query.is_empty() && !chat.title.to_lowercase().contains(&query) {
                            continue;
                        }

                        let is_active = idx == self.state.active_chat_index;
                        let text_color = if is_active { RESEARCH_BLUE } else { INK };

                        let mut job = egui::text::LayoutJob::default();
                        job.append(
                            &chat.title,
                            0.0,
                            egui::TextFormat {
                                font_id: FontId::new(13.0, FontFamily::Proportional),
                                color: text_color,
                                ..Default::default()
                            },
                        );

                        let btn = egui::Button::new(job)
                            .wrap()
                            .min_size(Vec2::new(ui.available_width(), 24.0));

                        let resp = ui.add(btn);
                        if resp.clicked() {
                            switch_to_idx = Some(idx);
                        }
                    }
                });

            if let Some(idx) = switch_to_idx {
                self.state.active_chat_index = idx;
            }

            // Providers footer summary in sidepanel
            ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
                ui.add_space(6.0);
                ui.separator();
                ui.horizontal(|ui| {
                    let prov_count = self.state.settings.providers.len();
                    ui.label(
                        RichText::new(format!(
                            "{} provider{}",
                            prov_count,
                            if prov_count == 1 { "" } else { "s" }
                        ))
                        .font(FontId::new(12.0, FontFamily::Proportional))
                        .color(Color32::from_rgb(0x60, 0x6E, 0x68)),
                    );

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui
                            .link(
                                RichText::new("manage")
                                    .font(FontId::new(12.0, FontFamily::Proportional))
                                    .color(RESEARCH_BLUE),
                            )
                            .clicked()
                        {
                            self.state.show_settings_window = true;
                        }
                    });
                });
            });
        });
    }

    fn render_bottom_input(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            // Centered input matching transcript width (max 680px)
            let avail_width = ui.available_width();
            let input_width = 680.0f32.min(avail_width);
            let margin = ((avail_width - input_width) / 2.0).max(0.0);

            ui.horizontal(|ui| {
                ui.add_space(margin);
                ui.vertical(|ui| {
                    ui.set_width(input_width);

                    let mut send_requested = false;

                    // Multiline text input
                    ui.horizontal(|ui| {
                        let text_edit = TextEdit::multiline(&mut self.state.input_text)
                            .desired_rows(2)
                            .desired_width(input_width - 80.0)
                            .hint_text("Ask a question to start researching...");

                        let output = ui.add(text_edit);

                        // Enter to send, Shift+Enter for newline
                        if output.has_focus()
                            && ui.input(|i| i.key_pressed(Key::Enter) && !i.modifiers.shift)
                        {
                            send_requested = true;
                        }

                        // Research Blue Send button
                        let send_btn = egui::Button::new(
                            RichText::new("Send")
                                .font(FontId::new(14.0, FontFamily::Proportional))
                                .color(Color32::WHITE),
                        )
                        .fill(RESEARCH_BLUE)
                        .min_size(Vec2::new(70.0, 48.0));

                        if ui.add(send_btn).clicked() {
                            send_requested = true;
                        }
                    });

                    if send_requested {
                        self.state.send_prompt();
                    }

                    ui.add_space(4.0);

                    // Small status line (sentence-case meta 12px)
                    ui.horizontal(|ui| {
                        let is_thinking = self.state.status_message.contains("...")
                            || self.state.status_message.starts_with("Searching");
                        let status_color = if is_thinking {
                            AMBER
                        } else {
                            Color32::from_rgb(0x60, 0x6E, 0x68)
                        };

                        ui.label(
                            RichText::new(&self.state.status_message)
                                .font(FontId::new(12.0, FontFamily::Proportional))
                                .color(status_color),
                        );
                    });
                });
                ui.add_space(margin);
            });
        });
    }
}
