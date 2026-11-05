//! Custom theme and typography for exchat.

use eframe::egui::{
    self, epaint::Shadow, Color32, Context, CornerRadius, FontFamily, FontId, Margin, Stroke,
    TextStyle, Theme, Vec2, Visuals,
};
use std::sync::Arc;

pub const PAPER: Color32 = Color32::from_rgb(0xED, 0xF0, 0xEE); // #EDF0EE
pub const PANEL: Color32 = Color32::from_rgb(0xF9, 0xFA, 0xF9); // #F9FAF9
pub const INK: Color32 = Color32::from_rgb(0x18, 0x24, 0x20); // #182420
pub const LINE: Color32 = Color32::from_rgb(0xC9, 0xD1, 0xCC); // #C9D1CC
pub const RESEARCH_BLUE: Color32 = Color32::from_rgb(0x1F, 0x3F, 0xD1); // #1F3FD1
pub const AMBER: Color32 = Color32::from_rgb(0xB9, 0x74, 0x09); // #B97409

const INTER_REGULAR: &[u8] = include_bytes!("../../assets/fonts/Inter-Regular.ttf");
const INTER_SEMIBOLD: &[u8] = include_bytes!("../../assets/fonts/Inter-SemiBold.ttf");
const JETBRAINS_MONO: &[u8] = include_bytes!("../../assets/fonts/JetBrainsMono-Regular.ttf");

pub fn setup_fonts(ctx: &Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Insert custom fonts if valid
    fonts.font_data.insert(
        "Inter-Regular".to_owned(),
        Arc::new(egui::FontData::from_static(INTER_REGULAR)),
    );
    fonts.font_data.insert(
        "Inter-SemiBold".to_owned(),
        Arc::new(egui::FontData::from_static(INTER_SEMIBOLD)),
    );
    fonts.font_data.insert(
        "JetBrainsMono-Regular".to_owned(),
        Arc::new(egui::FontData::from_static(JETBRAINS_MONO)),
    );

    // Prioritize Inter for Proportional
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "Inter-Regular".to_owned());

    // Prioritize JetBrains Mono for Monospace
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .insert(0, "JetBrainsMono-Regular".to_owned());

    // Custom named family for titles
    fonts.families.insert(
        FontFamily::Name("Inter-SemiBold".into()),
        vec!["Inter-SemiBold".to_owned(), "Inter-Regular".to_owned()],
    );

    ctx.set_fonts(fonts);
}

pub fn apply_theme(ctx: &Context) {
    setup_fonts(ctx);
    ctx.set_theme(Theme::Light);

    ctx.style_mut_of(Theme::Light, |style| {
        // Typography
        style.text_styles.insert(
            TextStyle::Heading,
            FontId::new(16.0, FontFamily::Name("Inter-SemiBold".into())),
        );
        style
            .text_styles
            .insert(TextStyle::Body, FontId::new(14.0, FontFamily::Proportional));
        style.text_styles.insert(
            TextStyle::Button,
            FontId::new(14.0, FontFamily::Proportional),
        );
        style.text_styles.insert(
            TextStyle::Small,
            FontId::new(12.0, FontFamily::Proportional),
        );
        style.text_styles.insert(
            TextStyle::Monospace,
            FontId::new(12.5, FontFamily::Monospace),
        );

        // Spacing and sizing: spacing 8.0
        style.spacing.item_spacing = Vec2::new(8.0, 8.0);
        style.spacing.window_margin = Margin::same(12);
        style.spacing.button_padding = Vec2::new(8.0, 6.0);

        // Custom light visuals
        let mut visuals = Visuals::light();
        visuals.dark_mode = false;
        visuals.panel_fill = PANEL;
        visuals.window_fill = PANEL;
        visuals.extreme_bg_color = Color32::WHITE;
        visuals.faint_bg_color = PAPER;

        // Line / stroke styling: 1px line
        visuals.window_stroke = Stroke::new(1.0, LINE);
        let border_stroke = Stroke::new(1.0, LINE);

        // Rounding 6.0
        let corner_radius = CornerRadius::same(6);
        visuals.window_corner_radius = corner_radius;
        visuals.menu_corner_radius = corner_radius;

        // Flat fills, no shadow
        visuals.window_shadow = Shadow::NONE;
        visuals.popup_shadow = Shadow::NONE;

        // Non-interactive widgets
        visuals.widgets.noninteractive.bg_fill = PAPER;
        visuals.widgets.noninteractive.weak_bg_fill = PAPER;
        visuals.widgets.noninteractive.bg_stroke = border_stroke;
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, INK);
        visuals.widgets.noninteractive.corner_radius = corner_radius;

        // Inactive widgets (buttons, checkboxes)
        visuals.widgets.inactive.bg_fill = PANEL;
        visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
        visuals.widgets.inactive.bg_stroke = border_stroke;
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, INK);
        visuals.widgets.inactive.corner_radius = corner_radius;

        // Hovered widgets
        visuals.widgets.hovered.bg_fill = PAPER;
        visuals.widgets.hovered.weak_bg_fill = PAPER;
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, RESEARCH_BLUE);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, INK);
        visuals.widgets.hovered.corner_radius = corner_radius;

        // Active widgets
        visuals.widgets.active.bg_fill = RESEARCH_BLUE;
        visuals.widgets.active.weak_bg_fill = RESEARCH_BLUE;
        visuals.widgets.active.bg_stroke = Stroke::new(1.0, RESEARCH_BLUE);
        visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
        visuals.widgets.active.corner_radius = corner_radius;

        // Open widgets (e.g. open dropdown)
        visuals.widgets.open.bg_fill = PAPER;
        visuals.widgets.open.weak_bg_fill = PAPER;
        visuals.widgets.open.bg_stroke = Stroke::new(1.0, RESEARCH_BLUE);
        visuals.widgets.open.fg_stroke = Stroke::new(1.0, INK);
        visuals.widgets.open.corner_radius = corner_radius;

        // Selection
        visuals.selection.bg_fill = Color32::from_rgba_unmultiplied(0x1F, 0x3F, 0xD1, 50);
        visuals.selection.stroke = Stroke::new(1.0, RESEARCH_BLUE);

        // Hyperlinks
        visuals.hyperlink_color = RESEARCH_BLUE;

        style.visuals = visuals;
    });
}
