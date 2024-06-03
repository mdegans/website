//! Common content to multiple pages.

use egui_commonmark::CommonMarkCache;

use crate::gallery::{self, Gallery};

/// Recent projects.
pub fn projects(ui: &mut egui::Ui, commonmark_cache: &mut CommonMarkCache) {
    egui::CollapsingHeader::new("Projects")
        .default_open(false)
        .show(ui, |ui| {
            projects_inner(ui, commonmark_cache);
        })
        .header_response
        .on_hover_text_at_pointer("My recent projects.");
}

/// Macro to generate a collapsible section with markdown content.
macro_rules! markdown_dropdown {
    ($ui:ident, $commonmark_cache:ident, $f:ident, $title:expr, $markdown_file:expr, $hover:expr) => {
        fn $f(ui: &mut egui::Ui, commonmark_cache: &mut CommonMarkCache) {
            egui::CollapsingHeader::new($title)
                .default_open(false)
                .show(ui, |ui| {
                    egui_commonmark::commonmark_str!(
                        $markdown_file,
                        ui,
                        commonmark_cache,
                        $markdown_file
                    );
                })
                .header_response
                .on_hover_text_at_pointer($hover);
        }

        $f($ui, $commonmark_cache);
    };
    () => {};
}

/// Projects, but without the [`egui::CollapsingHeader`].
pub fn projects_inner(
    ui: &mut egui::Ui,
    commonmark_cache: &mut CommonMarkCache,
) {
    markdown_dropdown!(
        ui,
        commonmark_cache,
        weave,
        "Weave",
        "assets/weave.md",
        "A tree-based story writing tool featuring generative AI."
    );
    markdown_dropdown!(
        ui,
        commonmark_cache,
        website,
        "Drama Llama",
        "assets/drama_llama.md",
        "A safe and ergonomic Rust wrapper for `llama.cpp`."
    );
    markdown_dropdown!(
        ui,
        commonmark_cache,
        jeep,
        "Jeep",
        "assets/jeep.md",
        "A Rust library for parsing Jeep JL CAN bus events."
    );

    markdown_dropdown!(
        ui,
        commonmark_cache,
        nvalhalla,
        "NValhalla",
        "assets/nvalhalla.md",
        "A DeepStream social distancing demo for NVIDIA platforms."
    );

    markdown_dropdown!(
        ui,
        commonmark_cache,
        substream,
        "Substream",
        "assets/substream.md",
        "An audio to subtitle transcription tool using Google's TTS."
    );

    ui.horizontal(|ui| {
        ui.label("More projects:");
        ui.hyperlink("https://github.com/mdegans");
    });
}

/// About me.
pub fn about(ui: &mut egui::Ui, commonmark_cache: &mut CommonMarkCache) {
    markdown_dropdown!(
        ui,
        commonmark_cache,
        about_inner,
        "About",
        "assets/about.md",
        "A little about me."
    );
}

/// Hobbies outside of programming.
pub fn hobbies(
    ui: &mut egui::Ui,
    commonmark_cache: &mut CommonMarkCache,
    gallery: &mut Gallery,
    doggos: &mut Gallery,
) {
    egui::CollapsingHeader::new("Hobbies")
        .default_open(false)
        .show(ui, |ui| {
            hobbies_inner(ui, commonmark_cache, gallery, doggos);
        })
        .header_response
        .on_hover_text_at_pointer("Hobbies outside programming.");
}

/// Hobbies, but without the [`egui::CollapsingHeader`].
pub fn hobbies_inner(
    ui: &mut egui::Ui,
    commonmark_cache: &mut CommonMarkCache,
    gallery: &mut Gallery,
    doggos: &mut Gallery,
) {
    egui_commonmark::commonmark_str!(
        "Hobbies",
        ui,
        commonmark_cache,
        "assets/hobbies.md"
    );

    egui::CollapsingHeader::new("Photography")
        .default_open(false)
        .show(ui, |ui| {
            egui_commonmark::commonmark_str!(
                "assets/gallery.md",
                ui,
                commonmark_cache,
                "assets/gallery.md"
            );
            // TODO: Add a gallery of photos.
            gallery.ui(ui);
        })
        .header_response
        .on_hover_text_at_pointer("Some of my photography.");

    egui::CollapsingHeader::new("Dogs")
        .default_open(false)
        .show(ui, |ui| {
            egui_commonmark::commonmark_str!(
                "assets/doggos.md",
                ui,
                commonmark_cache,
                "assets/doggos.md"
            );
            doggos.ui(ui);
        })
        .header_response
        .on_hover_text_at_pointer("Photos of my dogs.");

    markdown_dropdown!(
        ui,
        commonmark_cache,
        games,
        "Video Games",
        "assets/games.md",
        "A bit on games I've enjoyed and game design."
    );
}
