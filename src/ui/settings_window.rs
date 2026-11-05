//! Settings window for managing providers and assigning model roles.

use crate::config::{ModelRef, Provider, Settings};
use crate::ui::theme::{INK, RESEARCH_BLUE};
use eframe::egui::{
    self, Color32, ComboBox, FontFamily, FontId, Layout, RichText, ScrollArea, TextEdit, Ui, Window,
};

pub fn render_settings_window(
    ctx: &egui::Context,
    open: &mut bool,
    settings: &mut Settings,
    status_msg: &mut String,
) {
    let mut is_open = *open;
    let mut close_requested = false;

    Window::new(
        RichText::new("Settings")
            .font(FontId::new(16.0, FontFamily::Name("Inter-SemiBold".into())))
            .color(INK),
    )
    .open(&mut is_open)
    .resizable(true)
    .default_width(520.0)
    .default_height(480.0)
    .show(ctx, |ui| {
        ScrollArea::vertical().show(ui, |ui| {
            ui.add_space(4.0);

            // Model Roles Section
            ui.label(
                RichText::new("Model Roles")
                    .font(FontId::new(14.0, FontFamily::Name("Inter-SemiBold".into())))
                    .color(INK),
            );
            ui.label(
                RichText::new("Assign configured provider models to each pipeline role.")
                    .font(FontId::new(12.0, FontFamily::Proportional))
                    .color(Color32::from_rgb(0x60, 0x6E, 0x68)),
            );
            ui.add_space(8.0);

            let all_models: Vec<(String, String)> = settings
                .providers
                .iter()
                .flat_map(|p| p.models.iter().map(move |m| (p.name.clone(), m.clone())))
                .collect();

            egui::Grid::new("roles_grid")
                .num_columns(2)
                .spacing([16.0, 10.0])
                .show(ui, |ui| {
                    ui.label(
                        RichText::new("Refiner:")
                            .font(FontId::new(13.0, FontFamily::Proportional))
                            .color(INK),
                    );
                    role_combobox(ui, "role_refiner", &mut settings.roles.refiner, &all_models);
                    ui.end_row();

                    ui.label(
                        RichText::new("Judge:")
                            .font(FontId::new(13.0, FontFamily::Proportional))
                            .color(INK),
                    );
                    role_combobox(ui, "role_judge", &mut settings.roles.judge, &all_models);
                    ui.end_row();

                    ui.label(
                        RichText::new("Explainer:")
                            .font(FontId::new(13.0, FontFamily::Proportional))
                            .color(INK),
                    );
                    role_combobox(
                        ui,
                        "role_explainer",
                        &mut settings.roles.explainer,
                        &all_models,
                    );
                    ui.end_row();

                    ui.label(
                        RichText::new("Titler:")
                            .font(FontId::new(13.0, FontFamily::Proportional))
                            .color(INK),
                    );
                    role_combobox(ui, "role_titler", &mut settings.roles.titler, &all_models);
                    ui.end_row();

                    ui.label(
                        RichText::new("Max search rounds:")
                            .font(FontId::new(13.0, FontFamily::Proportional))
                            .color(INK),
                    );
                    ui.add(egui::Slider::new(&mut settings.max_search_rounds, 1..=10));
                    ui.end_row();
                });

            ui.add_space(16.0);
            ui.separator();
            ui.add_space(12.0);

            // Providers Section
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Providers")
                        .font(FontId::new(14.0, FontFamily::Name("Inter-SemiBold".into())))
                        .color(INK),
                );
                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .button(
                            RichText::new("+ Add provider")
                                .font(FontId::new(12.0, FontFamily::Proportional))
                                .color(RESEARCH_BLUE),
                        )
                        .clicked()
                    {
                        settings.providers.push(Provider {
                            name: format!("provider-{}", settings.providers.len() + 1),
                            base_url: "https://api.openai.com/v1".into(),
                            api_key: String::new(),
                            models: vec!["gpt-4o".into(), "gpt-4o-mini".into()],
                        });
                    }
                });
            });

            ui.add_space(8.0);

            let mut delete_index = None;
            for (idx, provider) in settings.providers.iter_mut().enumerate() {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!("Provider #{}:", idx + 1))
                                .font(FontId::new(12.0, FontFamily::Proportional))
                                .color(Color32::from_rgb(0x60, 0x6E, 0x68)),
                        );
                        ui.text_edit_singleline(&mut provider.name);

                        ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui
                                .button(
                                    RichText::new("Remove")
                                        .font(FontId::new(12.0, FontFamily::Proportional))
                                        .color(Color32::from_rgb(0xB0, 0x20, 0x20)),
                                )
                                .clicked()
                            {
                                delete_index = Some(idx);
                            }
                        });
                    });

                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Base URL:")
                                .font(FontId::new(12.0, FontFamily::Proportional))
                                .color(INK),
                        );
                        ui.text_edit_singleline(&mut provider.base_url);
                    });

                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("API Key:")
                                .font(FontId::new(12.0, FontFamily::Proportional))
                                .color(INK),
                        );
                        ui.add(TextEdit::singleline(&mut provider.api_key).password(true));
                    });

                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Models (comma separated):")
                                .font(FontId::new(12.0, FontFamily::Proportional))
                                .color(INK),
                        );
                        let mut models_str = provider.models.join(", ");
                        if ui.text_edit_singleline(&mut models_str).changed() {
                            provider.models = models_str
                                .split(',')
                                .map(|s| s.trim().to_string())
                                .filter(|s| !s.is_empty())
                                .collect();
                        }
                    });
                });
                ui.add_space(8.0);
            }

            if let Some(idx) = delete_index {
                settings.providers.remove(idx);
            }

            ui.add_space(16.0);
            ui.separator();
            ui.add_space(12.0);

            // Save / Close
            ui.horizontal(|ui| {
                if ui
                    .add(
                        egui::Button::new(
                            RichText::new("Save settings")
                                .font(FontId::new(13.0, FontFamily::Proportional))
                                .color(Color32::WHITE),
                        )
                        .fill(RESEARCH_BLUE),
                    )
                    .clicked()
                {
                    match settings.save() {
                        Ok(()) => *status_msg = "Settings saved successfully".into(),
                        Err(e) => *status_msg = format!("Failed to save settings: {}", e),
                    }
                }

                if ui
                    .button(
                        RichText::new("Close")
                            .font(FontId::new(13.0, FontFamily::Proportional))
                            .color(INK),
                    )
                    .clicked()
                {
                    let _ = settings.save();
                    close_requested = true;
                }
            });
        });
    });

    if close_requested {
        *open = false;
    } else {
        *open = is_open;
    }
}

fn role_combobox(
    ui: &mut Ui,
    id_salt: &str,
    current: &mut Option<ModelRef>,
    all_models: &[(String, String)],
) {
    let display_text = match current {
        Some(m) => format!("{}: {}", m.provider, m.model),
        None => "None selected".to_string(),
    };

    ComboBox::from_id_salt(id_salt)
        .selected_text(
            RichText::new(display_text)
                .font(FontId::new(12.5, FontFamily::Monospace))
                .color(INK),
        )
        .show_ui(ui, |ui: &mut Ui| {
            if ui.selectable_value(current, None, "None").clicked() {
                // cleared
            }

            for (prov, model) in all_models {
                let model_ref = Some(ModelRef {
                    provider: prov.clone(),
                    model: model.clone(),
                });
                let label = format!("{}: {}", prov, model);
                ui.selectable_value(current, model_ref, label);
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_assignment() {
        let mut settings = Settings::default();
        settings.providers.push(Provider {
            name: "openai".into(),
            base_url: "https://api.openai.com/v1".into(),
            api_key: "sk-test".into(),
            models: vec!["gpt-4o".into()],
        });

        settings.roles.refiner = Some(ModelRef {
            provider: "openai".into(),
            model: "gpt-4o".into(),
        });

        assert_eq!(settings.roles.refiner.as_ref().unwrap().model, "gpt-4o");
    }
}
