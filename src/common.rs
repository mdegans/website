//! Common content to multiple pages.

use egui_commonmark::CommonMarkCache;

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
pub fn hobbies(ui: &mut egui::Ui, commonmark_cache: &mut CommonMarkCache) {
    egui::CollapsingHeader::new("Hobbies")
        .default_open(false)
        .show(ui, |ui| {
            hobbies_inner(ui, commonmark_cache);
        })
        .header_response
        .on_hover_text_at_pointer("Hobbies outside programming.");
}

/// Hobbies, but without the [`egui::CollapsingHeader`].
pub fn hobbies_inner(
    ui: &mut egui::Ui,
    commonmark_cache: &mut CommonMarkCache,
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
            // TODO: Add a gallery of photos.
        })
        .header_response
        .on_hover_text_at_pointer("Some of my photography.");

    markdown_dropdown!(
        ui,
        commonmark_cache,
        dogs,
        "Dogs",
        "assets/dogs.md",
        "My Dogs."
    );
    markdown_dropdown!(
        ui,
        commonmark_cache,
        games,
        "Video Games",
        "assets/games.md",
        "A bit on games I've enjoyed and game design."
    );
}
